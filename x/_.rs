#![feature(duration_millis_float)]
#![feature(stmt_expr_attributes)]
#![feature(trait_alias)]

pub fn main() {
  println!("Angle for chord length 1 is {:.64} pixels", xyz((103_f64 / 2.).to_radians(), 960. / 960., 6400.));

  let mut handle = vec![];

  handle.push(thread::spawn(|| {
    let screen_y = screen::high();
    let screen_x = screen::wide();
    let device = xyloid::device();

    let fy = |n1: f64| xyz(radian(70.53_f64 / 2.), n1 / screen_y / 2., 6400.);
    let fx = |n1: f64| xyz(radian(103_f64 / 2.), n1 / screen_x / 2., 6400.);

    let xxyy = |x: f64, y: f64| match d2::is_h() {
      T => d1::xy(&device, x, N as f64),
      _ => d1::xy(&device, x, y),
    };
    let yy = |n: u32, y: i32| fy(y as f64 - add_y(n));
    let xx = |n: u32, x: i32| fx(x as f64 + add_x(n));
    let kh = |a: bool| d2::h(&device, a);

    let mut abc_cy = N;
    let mut abc = || {
      let yy = |n1: f64| d1::xy(&device, N as f64, fy(n1));
      abc_cy = match d2::is_ml() {
        T => match d2::is_h() {
          T => {
            yy(match abc_cy {
              49..=u32::MAX => 0.,
              45..=48 => -1.,
              41..=44 => -1.,
              37..=40 => -3.,
              33..=36 => -3.,
              29..=32 => -3.,
              25..=28 => -3.,
              21..=24 => -3.,
              17..=20 => -5.,
              13..=16 => -5.,
              9..=12 => -5.,
              5..=8 => -3.,
              1..=4 => -1.,
              _ => 0.,
            });
            abc_cy + 1
          },
          _ => N,
        },
        _ => {
          d2::h(&device, T);
          N
        },
      };
    };

    let does = |c: u32, v: u32, x: i32, y: i32| {
      // TODO: difference should be atleast 2.
      println!("{}, {}, {}, {}", c, v, x, y);

      match c % 3 {
        1 => {
          let (ay, is_y) = match y.abs() >= MAX {
            T => (yy(v, y.min(MAX).max(-MAX)), F),
            _ => (yy(v, y), T),
          };

          let (ax, is_x) = match x.abs() >= MAX {
            T => (xx(v, x.min(MAX).max(-MAX)), F),
            _ => (xx(v, x), T),
          };

          match is_x && is_y {
            T => {
              xxyy(ax, ay);
              xo(MS * 4);
              kh(F)
            },
            _ => xxyy(ax, ay),
          }
        },
        _ => F,
      }
    };

    let mut at: u32 = 0;
    screen::watch(
      || match screen::name().contains(APP) {
        T => {
          abc();
          match d2::is_ml() {
            T => T,
            _ => F,
          }
        },
        _ => F,
      },
      |(nn, un, xn, yn)| {
        let mut is: bool = F;
        let mut ay: i32 = 0;
        let mut ax: i32 = 0;
        let mut an: u32 = N;

        for y in 0..yn {
          let yn_ = unsafe { nn.add(y * un) } as *const u32;
          let ay_ = (yn as i32 / 2) - y as i32;

          'x: for x in 0..xn {
            let xn_ = unsafe { *yn_.add(x) };
            let ax_ = (xn as i32 / 2) - x as i32;

            match is_pixel(xn_) {
              T => match is {
                T => {
                  an = an + 1;
                  break 'x;
                },
                _ => {
                  ay = ay_;
                  ax = ax_;
                  an = an + 1;
                  is = T;
                  break 'x;
                },
              },
              _ => F,
            };
          }
        }

        match is {
          T => {
            at = at + 1;
            does(at, an, -ax, ay)
          },
          _ => {
            at = N;
            does(at, 0, 0, 0)
          },
        }
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
    61..=u32::MAX => 16.,
    57..=60 => 15.,
    53..=56 => 14.,
    49..=52 => 13.,
    45..=48 => 12.,
    41..=44 => 11.,
    37..=40 => 10.,
    33..=36 => 9.,
    29..=32 => 8.,
    25..=28 => 7.,
    21..=24 => 6.,
    17..=20 => 5.,
    13..=16 => 4.,
    9..=12 => 3.,
    5..=8 => 2.,
    1..=4 => 1.,
    _ => 0.,
  }
}

#[inline(always)]
fn xyz(radian: f64, factor: f64, size: f64) -> f64 {
  (tan(radian, factor) / (2. * PI)) * size
}

#[inline(always)]
fn tan(n1: f64, n2: f64) -> f64 {
  (n1.tan() * n2).atan()
}

#[inline(always)]
fn radian(n: f64) -> f64 {
  n.to_radians()
}

const MAX: i32 = 128;
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
  },
  xyloid::{
    Device,
    d1,
    d2,
  },
};
