#[inline(always)]
pub fn sure<F: FnMut() -> bool>(mut f1: F, n1: Duration) -> bool {
  let init = Instant::now();
  let back = f1();
  let rest = init.elapsed();
  match back && n1 > rest {
    T => {
      sleep(n1 - rest);
      back
    },
    _ => back,
  }
}

#[inline(always)]
pub fn till(n: Instant) -> f64 {
  n.elapsed().as_millis_f64()
}

#[inline(always)]
pub fn rest(n: Duration) -> bool {
  sleep(n);
  T
}

#[inline(always)]
pub fn now() -> Instant {
  Instant::now()
}

pub const MS: Duration = Duration::from_millis(1);

use {
  crate::T,
  std::{
    thread::sleep,
    time::{
      Duration,
      Instant,
    },
  },
};
