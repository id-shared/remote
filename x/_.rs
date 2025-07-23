#![feature(duration_millis_float)]
#![feature(stmt_expr_attributes)]
#![feature(trait_alias)]

pub fn main() {
  println!("Angle for chord length 1 is {:.64} pixels", wealth((103_f64 / 2.).to_radians(), 960. / 960., 6400.));

  let mut handle = vec![];

  handle.push(thread::spawn(|| {
    const PIXELS_360: f64 = 6400.;
    let screen_y = screen::high();
    let screen_x = screen::wide();
    let device = xyloid::device();

    let fy = |n: f64| wealth(to_rad(70.53_f64 / 2.), n / (screen_y / 2.), PIXELS_360);
    let fx = |n: f64| wealth(to_rad(103_f64 / 2.), n / (screen_x / 2.), PIXELS_360);
    let xy = |x: f64, y: f64| d1::xy(&device, fx(x), fy(y));
    let kh = |a: bool| d2::h(&device, a);

    const MAX: f64 = 64.;
    screen::watch(
      |(n, v, x, y)| match n {
        16..=u32::MAX => xy(N as f64, recoil(n)),
        0..=15 => match n % 2 {
          N => {
            let (is_y, y_) = match y.abs() >= MAX {
              T => (F, y.min(MAX).max(-MAX) - add_y(v)),
              _ => (T, y - add_y(v)),
            };
            let (is_x, x_) = match x.abs() >= MAX {
              T => (F, x.min(MAX).max(-MAX) + add_x(v)),
              _ => (T, x + add_x(v)),
            };

            match is_x && is_y {
              T => match d2::is_h() {
                T => {
                  xy(x_, y_ + recoil(n));
                  xo(MS * 4);
                  kh(F)
                },
                _ => {
                  xy(x_, y_);
                  xo(MS * 4);
                  kh(F)
                },
              },
              _ => match d2::is_h() {
                T => xy(x_, y_ + recoil(n)),
                _ => xy(x_, y_),
              },
            }
          },
          _ => F,
        },
      },
      |(n, v, x, y)| {
        let mut y_ = 0.;
        let mut x_ = 0.;
        let mut v_ = N;
        let mut is = F;

        for yn in 0..y {
          let ny = unsafe { n.add(yn * v) } as *const u32;
          let ay = (y as i32 / 2) - yn as i32;

          'x: for xn in 0..x {
            let nx = unsafe { *ny.add(xn) };
            let ax = (x as i32 / 2) - xn as i32;

            match is_pixel(nx) {
              T => match is {
                T => {
                  v_ = v_ + 1;
                  break 'x;
                },
                _ => {
                  y_ = ay as f64;
                  x_ = ax as f64;
                  v_ = v_ + 1;
                  is = T;
                  break 'x;
                },
              },
              _ => F,
            };
          }
        }

        (is, v_, -x_, y_)
      },
      || match screen::name().contains(APP) {
        T => match d2::is_ml() {
          T => T,
          _ => {
            kh(T);
            F
          },
        },
        _ => F,
      },
      screen_x,
      screen_y,
    );
  }));

  handle.push(thread::spawn(|| {
    let io = xyloid::device();
    let mut d = (F, Instant::now());
    let mut a = (F, Instant::now());
    let mut w = (F, Instant::now());
    let mut s = (F, Instant::now());

    loop {
      match screen::name().contains(APP) {
        T => {
          d = on_key(d2::is_d, d2::al, &io, d);
          a = on_key(d2::is_a, d2::ar, &io, a);
          w = on_key(d2::is_w, d2::d, &io, w);
          s = on_key(d2::is_s, d2::u, &io, s);
          xo(MS)
        },
        _ => xo(MS),
      };
    }
  }));

  for x in handle {
    x.join().unwrap();
  }
}

#[inline(always)]
fn on_key<F1: Fn() -> bool, F2: Fn(&Device, bool) -> bool>(f1: F1, f2: F2, io: &Device, z1: BI) -> BI {
  on(
    f1,
    |_| (T, Instant::now()),
    |x| {
      let n = (x.1.elapsed().as_millis_f64() / 10.).round() as u64;
      match n {
        17..=32 => {
          f2(io, F);
          xo(MS * ((4 * 16) + ((n - 16) * 2)) as u32);
          f2(io, T)
        },
        6..=16 => {
          f2(io, F);
          xo(MS * (4 * n) as u32);
          f2(io, T)
        },
        0..=5 => T,
        _ => {
          f2(io, F);
          xo(MS * 96);
          f2(io, T)
        },
      };
      (F, Instant::now())
    },
    z1,
  )
}

#[inline(always)]
fn on<F1: Fn() -> bool, F2: Fn(BI) -> BI, F3: Fn(BI) -> BI>(f1: F1, f2: F2, f3: F3, z1: BI) -> BI {
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

type BI = (bool, Instant);

#[inline(always)]
fn is_pixel(x: u32) -> bool {
  let n1 = ((x >> 16) & 0xff) as u8;
  let n2 = ((x >> 8) & 0xff) as u8;
  let n3 = (x & 0xff) as u8;

  match n1 > CLR && n3 > CLR {
    T => match n1 > n3 {
      T => match n3.abs_diff(n2) > ABS {
        T => T,
        _ => F,
      },
      _ => match n1.abs_diff(n2) > ABS {
        T => T,
        _ => F,
      },
    },
    _ => F,
  }
}

#[inline(always)]
fn recoil(n: u32) -> f64 {
  match n {
    48..=u32::MAX => 0.,
    44..=47 => -1.,
    40..=43 => -1.,
    36..=39 => -3.,
    33..=35 => -3.,
    28..=31 => -3.,
    24..=27 => -3.,
    20..=23 => -3.,
    16..=19 => -5.,
    12..=15 => -5.,
    8..=11 => -5.,
    4..=7 => -3.,
    0..=3 => -1.,
    _ => 0.,
  }
}

#[inline(always)]
fn add_y(n1: u32) -> f64 {
  add(n1) / 2.
}

#[inline(always)]
fn add_x(n1: u32) -> f64 {
  add(n1) / 4.
}

#[inline(always)]
fn add(n1: u32) -> f64 {
  match n1 {
    60..=u32::MAX => 16.,
    56..=59 => 15.,
    52..=55 => 14.,
    48..=51 => 13.,
    44..=47 => 12.,
    40..=43 => 11.,
    36..=39 => 10.,
    33..=35 => 9.,
    28..=31 => 8.,
    24..=27 => 7.,
    20..=23 => 6.,
    16..=19 => 5.,
    12..=15 => 4.,
    8..=11 => 3.,
    4..=7 => 2.,
    0..=3 => 1.,
    _ => 0.,
  }
}

#[inline(always)]
fn wealth(radian: f64, factor: f64, size: f64) -> f64 {
  (dollar(radian, factor) / (2. * PI)) * size
}

#[inline(always)]
fn dollar(n1: f64, n2: f64) -> f64 {
  (n1.tan() * n2).atan()
}

#[inline(always)]
fn to_rad(n: f64) -> f64 {
  n.to_radians()
}

const CLR: u8 = 231;
const ABS: u8 = 24;
const APP: &str = "";

#[inline(always)]
pub fn xo(n: Duration) -> bool {
  thread::sleep(n);
  T
}

pub const MS: Duration = Duration::from_millis(1);
pub const HZ: u32 = 16;

pub const N: u32 = 0;
pub const F: bool = false;
pub const T: bool = true;

use {
  screen,
  std::{
    f64::consts::PI,
    i32,
    thread,
    time::{
      Duration,
      Instant,
    },
    u32,
  },
  xyloid::{
    Device,
    d1,
    d2,
  },
};
