pub struct RawSpinLock {
  locked: AtomicBool,
}

unsafe impl RawMutex for RawSpinLock {
  type GuardMarker = GuardSend;

  #[allow(clippy::declare_interior_mutable_const)]
  const INIT: Self = Self {
    locked: AtomicBool::new(false),
  };

  fn lock(&self) {
    let mut spin = 1;
    while self.locked.swap(true, Ordering::Acquire) {
      for _ in 0..spin {
        spin_loop();
      }
      spin = (spin * 2).min(64);
    }
  }

  fn try_lock(&self) -> bool {
    !self.locked.swap(true, Ordering::Acquire)
  }

  unsafe fn unlock(&self) {
    self.locked.store(false, Ordering::Release);
  }
}

pub type SpinLock<T> = Mutex<RawSpinLock, T>;

pub use ::lock_api::RawMutex;
use {
  ::core::{
    cmp::Ord,
    hint::spin_loop,
    sync::atomic::{
      AtomicBool,
      Ordering,
    },
  },
  ::lock_api::{
    GuardSend,
    Mutex,
  },
};
