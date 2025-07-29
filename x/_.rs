#![feature(duration_millis_float)]
#![feature(stmt_expr_attributes)]
#![feature(trait_alias)]

pub fn main() {
  println!("Angle for chord length 1 is {:.64} pixels", wealth(to_rad(103_f64 / 2.), 1. / 1280., 6400.));

  let mut zz = vec![];

  zz.push(thread::spawn(move || {
    let screen_high = screen::high();
    let screen_wide = screen::wide();
    let device = xyloid::device();
    let y_high = screen_high / 2.;
    let x_wide = screen_wide / 2.;

    let get_y_ = |ay: f64| wealth(to_rad(VFOV / 2.), ay / y_high, _360);
    let get_x_ = |ax: f64| wealth(to_rad(HFOV / 2.), ax / x_wide, _360);
    let xy = |ax: f64, ay: f64| d1::xy(get_x_(ax), get_y_(ay), &device);

    let is_kl = || d2::is_i();
    let kl = |is: bool| {
      d2::i(is, &device);
      // d2::j(is, &device);
      T
    };

    let mut an = 0;

    const COLOR_N_3: u8 = 255 - 24;
    const COLOR_N_2: u8 = 96;
    const COLOR_N_1: u8 = 4;

    const _360: f64 = 6396.5885;
    const VFOV: f64 = 70.53;
    const HFOV: f64 = 103.;
    const FREQ: u32 = 19;

    #[inline(always)]
    fn is_pixel(n_1: u8, n_2: u8, n_3: u8, x: u32) -> bool {
      let n1 = ((x >> 16) & 0xff) as u8;
      let n2 = ((x >> 8) & 0xff) as u8;
      let n3 = (x & 0xff) as u8;

      match n1 >= n_3 {
        T => match n_1 >= n2.abs_diff(n3) {
          T => match n2 >= n3 {
            T => match n1.abs_diff(n2) > n_2 {
              T => T,
              _ => F,
            },
            _ => match n1.abs_diff(n3) > n_2 {
              T => T,
              _ => F,
            },
          },
          _ => F,
        },
        _ => F,
      }
    }

    #[inline(always)]
    fn pull(n: u64) -> f64 {
      match n {
        48..=u64::MAX => N,
        42..=46 => -3.,
        36..=40 => -3.,
        30..=34 => -3.,
        24..=28 => -6.,
        18..=22 => -5.,
        12..=16 => -5.,
        6..=10 => -2.,
        0..=4 => -2.,
        _ => N,
      }
    }

    #[inline(always)]
    fn into(k1: f64, k2: f64, n: f64) -> (bool, f64) {
      match k2 >= n.abs() {
        T => (T, n),
        _ => (F, n / k1),
      }
    }

    #[inline(always)]
    fn each(k1: u64, k2: u64, n: f64) -> (bool, f64) {
      match k1 % k2 == 0 {
        T => (T, n),
        _ => (F, 0.),
      }
    }

    #[inline(always)]
    fn add_y(n: u64) -> f64 {
      n as f64 / 8.
    }

    #[inline(always)]
    fn add_x(n: u64) -> f64 {
      n as f64 / 24.
    }

    screen::watch(
      |(a, n, v, x, y)| match a {
        T => match is_kl() {
          T => {
            let zy = pull(an);
            an = an + 1;

            let (ax, zx) = each(n, 5, x + add_x(v));

            match ax {
              T => {
                xy(zx, zy);
                T
              },
              _ => {
                xy(zx, zy);
                F
              },
            }
          },
          _ => {
            let zy = y - add_y(v);
            an = 0;

            let (ax, zx) = into(3., x_wide / 2., x + add_x(v));

            match ax {
              T => {
                let (ax, zx) = into(3., x_wide / 8., x + add_x(v));

                match ax {
                  T => {
                    let (ax, zx) = each(n, 2, x + add_x(v));

                    match ax {
                      T => {
                        xy(zx, zy);
                        kl(F);
                        T
                      },
                      _ => {
                        xy(zx, zy);
                        F
                      },
                    }
                  },
                  _ => {
                    xy(zx, zy);
                    F
                  },
                }
              },
              _ => {
                xy(zx, zy);
                F
              },
            }
          },
        },
        _ => match is_kl() {
          T => {
            let zy = pull(an);
            an = an + 1;

            xy(N, zy);
            T
          },
          _ => {
            an = 0;

            F
          },
        },
      },
      |(n, v, x, y)| {
        let mut zy = N;
        let mut zx = N;
        let mut vz = 0;
        let mut va = 0;
        let mut is = F;

        for yn in 0..y {
          let ny = unsafe { n.add(yn * v) } as *const u32;
          let ay = (y as i32 / 2) - yn as i32;

          'x: for xn in 0..x {
            let nx = unsafe { *ny.add(xn) };
            let ax = (x as i32 / 2) - xn as i32;

            match is_pixel(COLOR_N_1, COLOR_N_2, COLOR_N_3, nx) {
              T => match is {
                T => {
                  vz = yn as u64;
                  break 'x;
                },
                _ => {
                  zy = ay as f64;
                  zx = ax as f64;
                  va = yn as u64;
                  is = T;
                  break 'x;
                },
              },
              _ => F,
            };
          }
        }

        match vz >= va {
          T => (is, vz - va, -zx, zy),
          _ => (is, 0, -zx, zy),
        }
      },
      || match screen::name().contains("VAL") && d2::is_ml() {
        T => T,
        _ => {
          kl(T);

          F
        },
      },
      FREQ,
      screen_wide,
      screen_high,
    );
  }));

  zz.push(thread::spawn(|| {
    let device = xyloid::device();
    let mut d = (F, time::now());
    let mut a = (F, time::now());
    let mut w = (F, time::now());
    let mut s = (F, time::now());

    #[inline(always)]
    fn held<F1: Fn() -> bool, F2: Fn(bool, &Device) -> bool>(f_1: F1, f_2: F2, x: BI, z: &Device) -> BI {
      on(
        f_1,
        |_| T,
        |n| {
          let n_ = (n / 10.).round() as u64;
          match n_ {
            17..=32 => {
              f_2(F, z);
              time::rest(time::MS * ((4 * 16) + ((n_ - 16) * 2)) as u32);
              f_2(T, z)
            },
            6..=16 => {
              f_2(F, z);
              time::rest(time::MS * (4 * n_) as u32);
              f_2(T, z)
            },
            0..=5 => T,
            _ => {
              f_2(F, z);
              time::rest(time::MS * 96);
              f_2(T, z)
            },
          };
          T
        },
        x,
      )
    }

    loop {
      match screen::name().contains("VAL") {
        T => {
          d = held(d2::is_d, d2::al, d, &device);
          a = held(d2::is_a, d2::ar, a, &device);
          w = held(d2::is_w, d2::ad, w, &device);
          s = held(d2::is_s, d2::au, s, &device);
          T
        },
        _ => F,
      };

      time::rest(time::MS);
    }
  }));

  for x in zz {
    x.join().unwrap();
  }
}

#[inline(always)]
pub fn wealth(radian: f64, factor: f64, size: f64) -> f64 {
  (dollar(radian, factor) / (2. * PI)) * size
}

#[inline(always)]
pub fn dollar(n1: f64, n2: f64) -> f64 {
  (n1.tan() * n2).atan()
}

#[inline(always)]
pub fn to_rad(n: f64) -> f64 {
  n.to_radians()
}

#[inline(always)]
fn on<F1: Fn() -> bool, F2: Fn(f64) -> bool, F3: Fn(f64) -> bool>(f1: F1, f2: F2, f3: F3, z: BI) -> BI {
  let (is, it) = z;

  match is {
    T => match f1() {
      T => z,
      _ => match f3(time::till(it)) {
        T => (F, time::now()),
        _ => (T, time::now()),
      },
    },
    _ => match f1() {
      T => match f2(time::till(it)) {
        T => (T, time::now()),
        _ => (F, time::now()),
      },
      _ => z,
    },
  }
}

type BI = (bool, Instant);

use {
  common::{
    F,
    N,
    T,
    time,
  },
  screen,
  std::{
    f64::consts::PI,
    i32,
    thread,
    time::Instant,
    u32,
    u64,
  },
  xyloid::{
    Device,
    d1,
    d2,
  },
};
