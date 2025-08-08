use std::{
  marker::PhantomData,
  ptr,
  sync::atomic::{
    AtomicPtr,
    AtomicUsize,
    Ordering,
  },
};

#[repr(align(64))]
struct Node<T> {
  data: Option<T>,
  next: AtomicPtr<Node<T>>,
}

pub struct SegQueue<T> {
  head: AtomicPtr<Node<T>>,
  _pad1: [u8; 64],
  tail: AtomicPtr<Node<T>>,
  _pad2: [u8; 64],
  cache_line: AtomicUsize, // Used for backoff optimization
  _pad3: [u8; 56],         // Complete the cache line (64 - sizeof(AtomicUsize))
  _marker: PhantomData<T>,
}

unsafe impl<T: Send> Send for SegQueue<T> {
}
unsafe impl<T: Send> Sync for SegQueue<T> {
}

impl<T> SegQueue<T> {
  #[inline]
  pub fn new(_capacity: usize) -> Self {
    // Create a dummy node that serves as the initial sentinel
    let dummy = Box::into_raw(Box::new(Node {
      data: None,
      next: AtomicPtr::new(ptr::null_mut()),
    }));

    // Use Ordering::Relaxed for initialization as this is single-threaded
    Self {
      head: AtomicPtr::new(dummy),
      _pad1: [0; 64],
      tail: AtomicPtr::new(dummy),
      _pad2: [0; 64],
      cache_line: AtomicUsize::new(1), // Start with a small backoff value
      _pad3: [0; 56],
      _marker: PhantomData,
    }
  }

  /// Push to the queue. Lock-free single-writer/single-reader fast-path.
  #[inline]
  pub fn push(&self, t: T) {
    let new = Box::into_raw(Box::new(Node {
      data: Some(t),
      next: AtomicPtr::new(ptr::null_mut()),
    }));

    // Ultra-fast path: direct tail update with relaxed ordering
    let tail = self.tail.load(Ordering::Relaxed);
    if unsafe { (*tail).next.compare_exchange_weak(ptr::null_mut(), new, Ordering::Release, Ordering::Relaxed) }.is_ok() {
      // Successfully linked, update tail with relaxed ordering
      self.tail.store(new, Ordering::Release);
      return;
    }

    // Fast path: attempt with proper ordering
    let mut backoff = self.cache_line.load(Ordering::Relaxed).max(1);
    let mut contention_count = 0;

    loop {
      let tail = self.tail.load(Ordering::Acquire);
      let tail_next = unsafe { (*tail).next.load(Ordering::Acquire) };

      if tail_next.is_null() {
        // Tail is valid, try to append
        if unsafe { (*tail).next.compare_exchange_weak(ptr::null_mut(), new, Ordering::Release, Ordering::Relaxed) }.is_ok() {
          // Move the tail pointer forward
          self.tail.store(new, Ordering::Release);

          // Update cache_line based on contention
          if contention_count > 0 {
            self.cache_line.store(backoff, Ordering::Relaxed);
          }
          return;
        }

        // Exponential backoff to reduce contention
        contention_count += 1;
        if backoff < 32 {
          for _ in 0..backoff {
            std::hint::spin_loop();
          }
          backoff = backoff.saturating_mul(2);
        }
      }
      else {
        // Tail is lagging, help advance it
        self.tail.compare_exchange_weak(tail, tail_next, Ordering::Release, Ordering::Relaxed).ok();
        // Reset backoff after helping
        backoff = 1;
      }
    }
  }

  /// Pop from the queue. Lock-free single-writer/single-reader fast-path.
  #[inline]
  pub fn pop(&self) -> Option<T> {
    // Ultra-fast path: try direct dequeue with relaxed ordering
    let head = self.head.load(Ordering::Relaxed);
    let next = unsafe { (*head).next.load(Ordering::Relaxed) };

    if next.is_null() {
      // Queue is empty
      return None;
    }

    if self.head.compare_exchange_weak(head, next, Ordering::Acquire, Ordering::Relaxed).is_ok() {
      // SAFETY: We have unique access to `head` now.
      let ret = unsafe { (*next).data.take() };
      unsafe {
        drop(Box::from_raw(head));
      }
      return ret;
    }

    // Fast path: try with proper ordering
    let mut backoff = self.cache_line.load(Ordering::Relaxed).max(1);
    let mut contention_count = 0;

    loop {
      let head = self.head.load(Ordering::Acquire);
      let next = unsafe { (*head).next.load(Ordering::Acquire) };

      if next.is_null() {
        // Queue is empty
        return None;
      }

      if self.head.compare_exchange_weak(head, next, Ordering::Release, Ordering::Relaxed).is_ok() {
        // SAFETY: We have unique access to `head` now.
        let ret = unsafe { (*next).data.take() };
        unsafe {
          drop(Box::from_raw(head));
        }

        // Update cache_line based on contention
        if contention_count > 0 {
          self.cache_line.store(backoff, Ordering::Relaxed);
        }
        return ret;
      }

      // Exponential backoff to reduce contention
      contention_count += 1;
      if backoff < 32 {
        for _ in 0..backoff {
          std::hint::spin_loop();
        }
        backoff = backoff.saturating_mul(2);
      }
    }
  }

  /// Check if the queue is empty
  #[inline]
  pub fn is_empty(&self) -> bool {
    let head = self.head.load(Ordering::Relaxed);
    let next = unsafe { (*head).next.load(Ordering::Relaxed) };
    next.is_null()
  }

  /// Returns an approximation of the number of elements in the queue
  #[inline]
  pub fn len(&self) -> usize {
    let mut count = 0;
    let mut current = unsafe { (*self.head.load(Ordering::Relaxed)).next.load(Ordering::Relaxed) };

    while !current.is_null() {
      count += 1;
      current = unsafe { (*current).next.load(Ordering::Relaxed) };

      // Safety valve to prevent infinite loops in case of concurrent modifications
      if count > 10000 {
        break;
      }
    }

    count
  }

  /// Try to pop with a non-blocking approach
  #[inline]
  pub fn try_pop(&self) -> Option<T> {
    let head = self.head.load(Ordering::Relaxed);
    let next = unsafe { (*head).next.load(Ordering::Relaxed) };

    if next.is_null() {
      return None;
    }

    // Use Release ordering for success case to ensure proper synchronization
    // with subsequent operations, while keeping Relaxed for the failure case
    // since we're not retrying
    if self.head.compare_exchange_weak(head, next, Ordering::Acquire, Ordering::Relaxed).is_ok() {
      // SAFETY: We have unique access to `head` now.
      let ret = unsafe { (*next).data.take() };
      unsafe {
        drop(Box::from_raw(head));
      }
      ret
    }
    else {
      None // Don't retry, just return None
    }
  }
}

impl<T> Default for SegQueue<T> {
  fn default() -> Self {
    Self::new(655_536)
  }
}

impl<T> Drop for SegQueue<T> {
  fn drop(&mut self) {
    while self.pop().is_some() {}
    let dummy = self.head.load(Ordering::Relaxed);
    unsafe {
      drop(Box::from_raw(dummy));
    }
  }
}
