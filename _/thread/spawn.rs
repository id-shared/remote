pub fn worker(job_queue: Arc<SegQueue<Job>>) -> Worker {
  let running = Arc::new(AtomicBool::new(true));
  let running_clone = Arc::clone(&running);

  let handle = thread::spawn(move || {
    while running_clone.load(Ordering::Relaxed) {
      if let Some(job) = job_queue.pop() {
        job();
      }
      else {
        std::hint::spin_loop();
      }
    }
  });

  Worker {
    thread: Some(handle),
    running,
  }
}

pub struct Worker {
  thread: Option<thread::JoinHandle<()>>,
  running: Arc<AtomicBool>,
}

impl Worker {
  pub fn stop(self) {
    self.running.store(false, Ordering::SeqCst);
    if let Some(thread) = self.thread {
      let _ = thread.join();
    }
  }
}

type Job = Box<dyn FnOnce() + Send + 'static>;
use {
  crate::queue::SegQueue,
  std::{
    sync::{
      Arc,
      atomic::{
        AtomicBool,
        Ordering,
      },
    },
    thread,
  },
};

pub mod worker;
