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

    let xy = |ax: f64, ay: f64| d1::xy(axis(HFOV, _360, x_wide, ax), axis(VFOV, _360, y_high, ay), &device);

    let is_kl = || d2::is_i();
    let kl = |is: bool| {
      d2::i(is, &device);
      // d2::j(is, &device);
      T
    };

    const _360: f64 = 6396.5885;
    const VFOV: f64 = 70.53;
    const HFOV: f64 = 103.;
    const FREQ: u32 = 18;

    #[inline(always)]
    fn axis(k1: f64, k2: f64, n1: f64, n2: f64) -> f64 {
      wealth(to_rad(k1 / 2.), n2 / n1, k2)
    }

    #[inline(always)]
    fn finder<F: Fn(u32) -> bool, I: IntoIterator<Item = usize> + Clone>(f: F, n: *const u8, v: usize, x: I, y: I) -> (bool, f64, f64) {
      for yn in y {
        let ny = unsafe { n.add(yn * v) } as *const u32;

        for xn in x.clone() {
          let nx = unsafe { *ny.add(xn) };

          match f(nx) {
            T => {
              return (T, xn as f64, yn as f64);
            },
            _ => F,
          };
        }
      }

      (false, 0., 0.)
    }

    #[inline(always)]
    fn check(z: u32) -> bool {
      let (n1, n2, n3, _) = color(z);

      match n1 >= 191 {
        T => match n3 >= 191 {
          T => match 16 >= n1.abs_diff(n3) {
            T => match n1 >= n3 {
              T => match n3 >= n2 {
                T => n3.abs_diff(n2) >= 64,
                _ => F,
              },
              _ => match n1 >= n2 {
                T => n1.abs_diff(n2) >= 64,
                _ => F,
              },
            },
            _ => F,
          },
          _ => F,
        },
        _ => F,
      }
    }

    #[inline(always)]
    fn color(n: u32) -> (u8, u8, u8, u8) {
      let n4 = ((n >> 24) & 0xff) as u8;
      let n1 = ((n >> 16) & 0xff) as u8;
      let n2 = ((n >> 8) & 0xff) as u8;
      let n3 = (n & 0xff) as u8;

      (n1, n2, n3, n4)
    }

    #[inline(always)]
    fn push(n: u64) -> f64 {
      match n {
        48..=u64::MAX => N,
        42..=46 => -3.,
        36..=40 => -3.,
        30..=34 => -5.,
        24..=28 => -5.,
        18..=22 => -5.,
        12..=16 => -5.,
        6..=10 => -2.,
        0..=4 => -2.,
        _ => N,
      }
    }

    #[inline(always)]
    fn pull(k: f64, l: f64, n: f64) -> f64 {
      (l / k) * n
    }

    #[inline(always)]
    fn into(k1: f64, k2: f64, n: f64) -> (bool, f64) {
      match k2 >= n.abs() {
        T => (T, n),
        _ => (F, n / k1),
      }
    }

    #[inline(always)]
    fn each(k: u64, n: u64) -> bool {
      n % k == 0
    }

    let mut y_ = 0.;
    let mut x_ = 0.;
    let mut n_ = 0;
    screen::watch(
      |(a, n, v, x, y)| match a {
        T => match each(2, n) {
          T => match is_kl() {
            T => {
              xy(x + x_, push(n - n_));
              n + 1
            },
            _ => {
              y_ = pull(64., y_high, v);
              x_ = pull(256., x_wide, v);
              n_ = n;

              match n >= 16 {
                T => {
                  xy(x + x_, y - y_);
                  kl(F);
                  n + 1
                },
                _ => {
                  let (ax, zx) = into(2., x_wide / 32., x + x_);

                  match ax {
                    T => {
                      xy(x + x_, y - y_);
                      kl(F);
                      n + 1
                    },
                    _ => {
                      xy(zx, y - y_);
                      n + 1
                    },
                  }
                },
              }
            },
          },
          _ => match is_kl() {
            T => {
              xy(N, push(n - n_));
              n + 1
            },
            _ => n + 1,
          },
        },
        _ => match is_kl() {
          T => {
            xy(N, push(n - n_));
            n + 1
          },
          _ => n + 1,
        },
      },
      |(n, v, x, y)| {
        let (is, xn, yn) = finder(check, n, v, 0..x, 0..y);

        match is {
          T => {
            let (is, _, yn_) = finder(check, n, v, (0..x).rev(), (0..y).rev());
            let y_ = y as f64 / 2.;
            let x_ = x as f64 / 2.;

            match yn_ >= yn {
              T => (is, (yn_ - y_) / y_, -(x_ - xn), y_ - yn),
              _ => (is, 0., xn, yn),
            }
          },
          _ => (is, 0., xn, yn),
        }
      },
      || match screen::name().contains("VAL") {
        T => match d2::is_ml() {
          T => match d2::is_d() || d2::is_a() || d2::is_w() || d2::is_s() || d2::is_al() || d2::is_ar() || d2::is_ad() || d2::is_au() {
            T => {
              kl(T);

              F
            },
            _ => T,
          },
          _ => {
            kl(T);

            F
          },
        },
        _ => F,
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
    fn held<F1: Fn() -> bool, F2: Fn(bool, &Device) -> bool, F3: Fn(&Device) -> bool>(f1: F1, f2: F2, f3: F3, x: BI, z: &Device) -> BI {
      on(
        f1,
        |_| T,
        |n| {
          let n_ = (n / 10.).round() as u32;
          match n_ {
            17..=32 => {
              f2(F, z);
              time::rest(time::MS * ((4 * 16) + ((n_ - 16) * 2)));
              f2(T, z);

              f3(z)
            },
            6..=16 => {
              f2(F, z);
              time::rest(time::MS * (4 * n_));
              f2(T, z);

              f3(z)
            },
            0..=5 => T,
            _ => {
              f2(F, z);
              time::rest(time::MS * 96);
              f2(T, z);

              f3(z)
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
          d = held(d2::is_d, d2::al, |_| T, d, &device);
          a = held(d2::is_a, d2::ar, |_| T, a, &device);
          w = held(d2::is_w, d2::ad, |_| T, w, &device);
          s = held(d2::is_s, d2::au, |_| T, s, &device);
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
