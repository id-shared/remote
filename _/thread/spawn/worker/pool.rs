#[inline]
pub fn execute<F: Abc>(pool: &Pool, f: F) {
  pool.job_queue.push(Box::new(f));
}

pub fn exit(pool: &mut Pool) {
  for _ in 0..pool.workers.len() {
    pool.job_queue.push(Box::new(|| {
      std::thread::sleep(std::time::Duration::from_nanos(1));
    }));
  }

  for worker in &mut pool.workers {
    if let Some(thread) = worker.thread.take() {
      thread.join().unwrap();
    }
  }
}

pub trait Abc = FnOnce() + Send + 'static;

use crate::spawn::worker::Pool;
