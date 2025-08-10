pub fn ms_sure<F: FnMut() -> bool>(mut f1: F, n1: u64) -> bool {
  let init = Instant::now();
  let back = f1();
  let rest = ms_till(init);
  match back && n1 > rest {
    T => {
      ms_rest(n1 - rest);
      back
    },
    _ => back,
  }
}

pub fn ms_till(n: Instant) -> u64 {
  crate::it(u64::try_from(n.elapsed().as_millis()))
}

pub fn ms_rest(n: u64) -> bool {
  sleep(Duration::from_millis(n));
  T
}

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
