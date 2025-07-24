#![feature(duration_millis_float)]
#![feature(stmt_expr_attributes)]
#![feature(trait_alias)]

pub fn main() {
  println!("Angle for chord length 1 is {:.64} pixels", wealth(to_rad(103_f64 / 2.), 1. / 1280., 6400.));

  let mut handle = vec![];

  handle.push(thread::spawn(|| {
    let device = xyloid::device();
    let high_y = screen::high();
    let wide_x = screen::wide();
    let axis_y = high_y / 64.;
    let axis_x = wide_x / 64.;
    let mut at = N;

    let get_y_ = |ay: f64| wealth(to_rad(70.53_f64 / 2.), ay / (high_y / 2.), PIXELS_360);
    let get_x_ = |ax: f64| wealth(to_rad(103.0_f64 / 2.), ax / (wide_x / 2.), PIXELS_360);
    let xy = |ax: f64, ay: f64| d1::xy(&device, get_x_(ax), get_y_(ay));
    let kh = |is: bool| d2::h(&device, is);

    const PIXELS_360: f64 = 6400.;
    const UNTILL: f64 = 64.;
    const FACTOR: f64 = 3.;
    const EACH: f64 = 2.;

    screen::watch(
      |(n, v, x, y)| match d2::is_h() {
        T => {
          let zy = recoil(at);
          at = match UNTILL > n {
            T => match n % EACH {
              0. => {
                let zx = x + add_x(v);
                let ax = match zx.abs() >= axis_x {
                  T => zx / FACTOR,
                  _ => zx,
                };

                xy(ax, zy);
                at + 1
              },
              _ => {
                xy(0., zy);
                at + 1
              },
            },
            _ => {
              xy(0., zy);
              at + 1
            },
          };

          T
        },
        _ => {
          at = match UNTILL > n {
            T => match n % EACH {
              0. => {
                let zy = y - add_y(v);
                let zx = x + add_x(v);

                let (is_y, ay) = match zy.abs() >= axis_y {
                  T => (F, zy / FACTOR),
                  _ => (T, zy),
                };
                let (is_x, ax) = match zx.abs() >= axis_x {
                  T => (F, zx / FACTOR),
                  _ => (T, zx),
                };

                println!("{}, {}, {}, {}", is_x, is_y, x, y);

                match is_x && is_y {
                  T => {
                    xy(ax, ay);
                    xo(MS * 4);
                    kh(F);
                    N
                  },
                  _ => {
                    xy(ax, ay);
                    N
                  },
                }
              },
              _ => N,
            },
            _ => N,
          };

          T
        },
      },
      |(n, v, x, y)| {
        let mut y_ = 0.;
        let mut x_ = 0.;
        let mut v_ = 0.;
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
                  v_ = v_ + 1.;
                  break 'x;
                },
                _ => {
                  y_ = ay as f64;
                  x_ = ax as f64;
                  v_ = v_ + 1.;
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
      wide_x,
      high_y,
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
    48..=u32::MAX => -0.,
    44..=47 => -0.,
    40..=43 => -1.,
    36..=39 => -2.,
    33..=35 => -5.,
    28..=31 => -5.,
    24..=27 => -5.,
    20..=23 => -5.,
    16..=19 => -5.,
    12..=15 => -5.,
    8..=11 => -2.,
    4..=7 => -1.,
    0..=3 => -0.,
    _ => -0.,
  }
}

#[inline(always)]
fn add_y(n1: f64) -> f64 {
  n1 / 4.
}

#[inline(always)]
fn add_x(n1: f64) -> f64 {
  n1 / 16.
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

const CLR: u8 = 255 - 4;
const ABS: u8 = 48;
const APP: &str = "VAL";

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
