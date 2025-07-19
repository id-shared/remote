#![feature(duration_millis_float)]
#![feature(stmt_expr_attributes)]
#![feature(trait_alias)]

pub fn main() {
  println!("Angle for chord length 1 is {:.64} pixels", calc((103_f64 / 2.).to_radians(), 960. / 960., 6400.));

  let mut handle = vec![];
  let zy = screen::high();
  let zx = screen::wide();
  let vy = fov(70.53_f64);
  let vx = fov(103_f64);
  let uy = zy / 2.;
  let ux = zx / 2.;

  let fy = #[inline(always)]
  move |n1: f64| calc(vy, n1 / uy, 6400.);
  let fx = #[inline(always)]
  move |n1: f64| calc(vx, n1 / ux, 6400.);

  let (i2, o2): (Sender<(i32, i32, i32, i32)>, Receiver<(i32, i32, i32, i32)>) = bounded(64);
  handle.push(thread::spawn(
    #[inline(always)]
    move || match xyloid::type_1() {
      Some(io) => {
        const HZ: Duration = Duration::from_millis(16);
        #[inline(always)]
        pub fn zz(io: xyloid::HANDLE, i: i32, n: i32, x: f64, y: f64) -> bool {
          match i <= n {
            T => {
              let rr = feio(i as f64 / n as f64);
              let ay = (rr * y).round();
              let ax = (rr * x).round();

              // println!("{} {} {} | {:.2} {} {} {}", i, n, t, rr, at, ax, ay);

              xyloid::xy(io, ax, ay);
              xo(HZ);
              zz(io, i + 1, n, x - ax, y - ay)
            },
            _ => {
              xyloid::xy(io, x, y);
              match xyloid::is_h() {
                T => T,
                _ => xyloid::key_h(io, F),
              }
            },
          }
        }

        #[inline(always)]
        pub fn zn(n1: i32) -> i32 {
          match n1 {
            61..=i32::MAX => 16,
            57..=60 => 15,
            53..=56 => 14,
            49..=52 => 13,
            45..=48 => 12,
            41..=44 => 11,
            37..=40 => 10,
            33..=36 => 9,
            29..=32 => 8,
            25..=28 => 7,
            21..=24 => 6,
            17..=20 => 5,
            13..=16 => 4,
            9..=12 => 3,
            5..=8 => 2,
            1..=4 => 1,
            _ => 0,
          }
        }

        #[inline(always)]
        pub fn feio(t: f64) -> f64 {
          let t = t.clamp(0.0, 1.0);
          (3.0 * t * t) - (2.0 * t * t * t)
        }

        #[inline(always)]
        pub fn yn(n1: i32) -> f64 {
          (zn(n1) as f64) / 1.5
        }

        #[inline(always)]
        pub fn xn(n1: i32) -> f64 {
          (zn(n1) as f64) / 3.
        }

        let zz = #[inline(always)]
        |n: i32, x: f64, y: f64| zz(io, N, n, x, y);
        let yy = #[inline(always)]
        |n: i32, n1: i32| fy(n as f64 - yn(n1));
        let xx = #[inline(always)]
        |n: i32, n1: i32| fx(n as f64 + xn(n1));

        while let Ok((an, ax, ay, az)) = o2.recv() {
          let yy = yy(ay, az);
          let xx = xx(ax, az);

          match an {
            2..=i32::MAX => match an % 4 {
              1 => zz(1, xx, 0.),
              _ => F,
            },
            1 => zz(3, xx, yy),
            _ => F,
          };
        }
      },
      _ => {},
    },
  ));

  let (i1, o1): (Sender<i32>, Receiver<i32>) = bounded(64);
  handle.push(thread::spawn(
    #[inline(always)]
    move || match xyloid::type_1() {
      Some(io) => {
        let yy = #[inline(always)]
        |n1: f64| xyloid::xy(io, N as f64, fy(n1));

        let mut cy = N;

        while let Ok(n) = o1.recv() {
          cy = match n {
            1..=i32::MAX => match xyloid::is_h() {
              T => {
                yy(match cy {
                  49..=i32::MAX => 0.,
                  45..=48 => -1.,
                  41..=44 => -2.,
                  37..=40 => -3.,
                  33..=36 => -4.,
                  29..=32 => -5.,
                  25..=28 => -3.,
                  21..=24 => -3.,
                  17..=20 => -5.,
                  13..=16 => -4.,
                  9..=12 => -3.,
                  5..=8 => -2.,
                  1..=4 => -1.,
                  _ => 0.,
                });
                cy + 1
              },
              _ => N,
            },
            _ => {
              match xyloid::is_h() {
                T => xyloid::key_h(io, T),
                _ => F,
              };
              N
            },
          };
        }
      },
      _ => {},
    },
  ));

  handle.push(thread::spawn(
    #[inline(always)]
    move || {
      const C2: u8 = 255 - 64;
      const C1: u8 = 255 - 8;

      screen::watch(
        #[inline(always)]
        move |n| match screen::name().contains(APP) {
          T => match xyloid::is_mouse_l() {
            T => {
              send(&i1, n + 1);
              n + 1
            },
            _ => {
              send(&i1, 0);
              0
            },
          },
          _ => 0,
        },
        #[inline(always)]
        move |x| {
          let n1 = ((x >> 16) & 0xff) as u8;
          let n2 = ((x >> 8) & 0xff) as u8;
          let n3 = (x & 0xff) as u8;

          match n1 >= C1 {
            T => match n3 >= C1 {
              T => match C2 >= n2 {
                T => T,
                _ => F,
              },
              _ => F,
            },
            _ => F,
          }
        },
        #[inline(always)]
        move |x| send(&i2, x),
        zx as i32,
        zy as i32,
      )
    },
  ));

  handle.push(thread::spawn(
    #[inline(always)]
    move || {
      match xyloid::type_1() {
        Some(io) => {
          let mut d = (F, Instant::now());
          let mut a = (F, Instant::now());
          let mut w = (F, Instant::now());
          let mut s = (F, Instant::now());
          loop {
            match screen::name().contains(APP) {
              T => {
                d = on_key(xyloid::is_d, xyloid::key_arrow_l, io, d);
                a = on_key(xyloid::is_a, xyloid::key_arrow_r, io, a);
                w = on_key(xyloid::is_w, xyloid::key_arrow_d, io, w);
                s = on_key(xyloid::is_s, xyloid::key_arrow_u, io, s);
                xo(MS)
              },
              _ => xo(MS),
            };
          }
        },
        _ => {},
      };
    },
  ));

  for x in handle {
    x.join().unwrap();
  }
}

#[inline(always)]
pub fn on_key<F1: Fn() -> bool, F2: Fn(xyloid::HANDLE, bool) -> bool>(f1: F1, f2: F2, io: xyloid::HANDLE, z1: BI) -> BI {
  on(
    f1,
    |_| (T, Instant::now()),
    |x| {
      let n = (x.1.elapsed().as_millis_f64() / 10.).round() as u64;
      match n {
        17..=32 => {
          f2(io, F);
          xo(Duration::from_millis((4 * 16) + ((n - 16) * 2)));
          f2(io, T)
        },
        6..=16 => {
          f2(io, F);
          xo(Duration::from_millis(4 * n));
          f2(io, T)
        },
        0..=5 => T,
        _ => {
          f2(io, F);
          xo(Duration::from_millis(96));
          f2(io, T)
        },
      };
      (F, Instant::now())
    },
    z1,
  )
}

#[inline(always)]
pub fn on<F1: Fn() -> bool, F2: Fn(BI) -> BI, F3: Fn(BI) -> BI>(f1: F1, f2: F2, f3: F3, z1: BI) -> BI {
  match z1.0 {
    T => match f1() {
      T => z1,
      _ => f3(z1),
    },
    _ => match f1() {
      T => f2(z1),
      _ => z1,
    },
  }
}

#[inline(always)]
pub fn xfov(hfov: f64, x: f64, y: f64) -> f64 {
  (2. * ((hfov.to_radians() / 2.).tan() * (y / x)).atan()).to_degrees()
}

#[inline(always)]
pub fn calc(radian: f64, factor: f64, size: f64) -> f64 {
  (tan(radian, factor) / (2. * PI)) * size
}

#[inline(always)]
pub fn send<T>(i: &Sender<T>, o: T) -> bool {
  i.try_send(o).is_ok()
}

#[inline(always)]
fn tan(n1: f64, n2: f64) -> f64 {
  (n1.tan() * n2).atan()
}

#[inline(always)]
pub fn fov(n: f64) -> f64 {
  (n / 2.).to_radians()
}

#[inline(always)]
pub fn stim(n1: i32, n2: i32) -> (i32, i32) {
  let next = step(n1, n2);

  match next.cmp(&N) {
    Greater => (n1, next),
    Less => (-n1, next),
    Equal => (n2, next),
  }
}

#[inline(always)]
pub fn step(n1: i32, n2: i32) -> i32 {
  match n2.cmp(&N) {
    Greater => (n2 - n1).max(N),
    Less => (n2 + n1).min(N),
    Equal => N,
  }
}

#[inline(always)]
pub fn xo(n: Duration) -> bool {
  thread::sleep(n);
  T
}

pub const APP: &str = "VAL";
pub const MS: Duration = Duration::from_millis(1);

pub const N: i32 = 0;
pub const F: bool = false;
pub const T: bool = true;

pub type BI = (bool, Instant);

use {
  crossbeam::channel::{
    Receiver,
    Sender,
    bounded,
  },
  screen,
  std::{
    cmp::Ordering::{
      Equal,
      Greater,
      Less,
    },
    f64::consts::PI,
    i32,
    thread,
    time::{
      Duration,
      Instant,
    },
  },
};
