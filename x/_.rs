#![feature(duration_millis_float)]
#![feature(stmt_expr_attributes)]
#![feature(trait_alias)]

pub fn main() {
  println!("Angle for chord length 1 is {:.64} pixels", wealth(to_rad(103_f64 / 2.), 1. / 1280., 6400.));

  let mut handle = vec![];
  const APP: &str = "VAL";

  handle.push(thread::spawn(|| {
    let device = xyloid::device();
    let high_y = screen::high();
    let wide_x = screen::wide();

    let get_y_ = |ay: f64| wealth(to_rad(70.53_f64 / 2.), ay / (high_y / 2.), _360);
    let get_x_ = |ax: f64| wealth(to_rad(103.0_f64 / 2.), ax / (wide_x / 2.), _360);
    let xy = |ax: f64, ay: f64| d1::xy(&device, get_x_(ax), get_y_(ay));

    let is_kl = || d2::is_i();
    let kl = |is: bool| d2::i(&device, is);

    let axis_y = high_y / 256.;
    let axis_x = wide_x / 256.;

    let mut at = time::now();
    let mut an = N;

    const COLOR_TINT: u8 = 255 - 24;
    const COLOR_DIFF: u8 = 24;

    const _360: f64 = 6400.;
    const FREQ: u32 = 18;

    #[inline(always)]
    fn is_pixel(k: u8, n: u8, x: u32) -> bool {
      let n1 = ((x >> 16) & 0xff) as u8;
      let n2 = ((x >> 8) & 0xff) as u8;
      let n3 = (x & 0xff) as u8;

      match n1 > k && n3 > k {
        T => match n1 > n3 {
          T => match n3.abs_diff(n2) > n {
            T => T,
            _ => F,
          },
          _ => match n1.abs_diff(n2) > n {
            T => T,
            _ => F,
          },
        },
        _ => F,
      }
    }

    #[inline(always)]
    fn recoil(k: f64, n: f64) -> f64 {
      let n_ = (k / 16.).round();
      match n {
        800.0..=f64::MAX => N,
        700.0..=800. => n_ * -2.,
        600.0..=700. => n_ * -4.,
        500.0..=600. => n_ * -4.,
        400.0..=500. => n_ * -4.,
        300.0..=400. => n_ * -5.,
        200.0..=300. => n_ * -4.,
        100.0..=200. => n_ * -2.,
        0.0..=100. => n_ * -1.,
        _ => N,
      }
    }

    #[inline(always)]
    fn into(i: f64, k: f64, n: f64) -> (bool, f64) {
      match n.abs() >= i {
        T => (F, n / k),
        _ => (T, n),
      }
    }

    #[inline(always)]
    fn each(n: f64) -> bool {
      match n {
        64.0..=f64::MAX => F,
        16.0..=64. => (n % 2.) == 1.,
        0.0..=64. => T,
        _ => F,
      }
    }

    #[inline(always)]
    fn add_y(n: f64) -> f64 {
      n / 4.
    }

    #[inline(always)]
    fn add_x(n: f64) -> f64 {
      n / 16.
    }

    screen::watch(
      |(a, n, v, x, y)| match a {
        T => match is_kl() {
          T => {
            let ay = recoil(FREQ as f64, time::till(at));
            an = match each(n) {
              T => {
                let (_, ax) = into(axis_x, 2., x + add_x(v));

                xy(ax, ay);
                an + 1.
              },
              _ => {
                xy(N, ay);
                an + 1.
              },
            };

            T
          },
          _ => {
            an = match each(n) {
              T => {
                let (is_y, ay) = into(axis_y, 4., y - add_y(v));
                let (is_x, ax) = into(axis_x, 4., x + add_x(v));

                at = match is_x && is_y {
                  T => {
                    xy(ax, ay);
                    kl(F);
                    time::now()
                  },
                  _ => {
                    xy(ax, ay);
                    time::now()
                  },
                };

                N
              },
              _ => N,
            };

            T
          },
        },
        _ => {
          an = match is_kl() {
            T => {
              xy(N, recoil(FREQ as f64, time::till(at)));
              an + 1.
            },
            _ => N,
          };

          T
        },
      },
      |(n, v, x, y)| {
        let mut y_ = N;
        let mut x_ = N;
        let mut v_ = N;
        let mut is = F;

        for yn in 0..y {
          let ny = unsafe { n.add(yn * v) } as *const u32;
          let ay = (y as i32 / 2) - yn as i32;

          'x: for xn in 0..x {
            let nx = unsafe { *ny.add(xn) };
            let ax = (x as i32 / 2) - xn as i32;

            match is_pixel(COLOR_TINT, COLOR_DIFF, nx) {
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
        T => match d2::is_ml() || d2::is_mr() {
          T => T,
          _ => {
            kl(T);
            F
          },
        },
        _ => F,
      },
      FREQ,
      wide_x,
      high_y,
    );
  }));

  handle.push(thread::spawn(|| {
    let io = xyloid::device();
    let mut d = (F, time::now());
    let mut a = (F, time::now());
    let mut w = (F, time::now());
    let mut s = (F, time::now());

    #[inline(always)]
    fn on_key<F1: Fn() -> bool, F2: Fn(&Device, bool) -> bool>(f1: F1, f2: F2, io: &Device, z1: BI) -> BI {
      on(
        f1,
        |_| (T, time::now()),
        |x| {
          let n = (time::till(x.1) / 10.).round() as u64;
          match n {
            17..=32 => {
              f2(io, F);
              time::rest(time::MS * ((4 * 16) + ((n - 16) * 2)) as u32);
              f2(io, T)
            },
            6..=16 => {
              f2(io, F);
              time::rest(time::MS * (4 * n) as u32);
              f2(io, T)
            },
            0..=5 => T,
            _ => {
              f2(io, F);
              time::rest(time::MS * 96);
              f2(io, T)
            },
          };
          (F, time::now())
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

    loop {
      match screen::name().contains(APP) {
        T => {
          d = on_key(d2::is_d, d2::al, &io, d);
          a = on_key(d2::is_a, d2::ar, &io, a);
          w = on_key(d2::is_w, d2::ad, &io, w);
          s = on_key(d2::is_s, d2::au, &io, s);
          time::rest(time::MS)
        },
        _ => time::rest(time::MS),
      };
    }
  }));

  for x in handle {
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

// #[inline(always)]
// fn ease(t: f64) -> f64 {
//   let t = t.clamp(N, 1.0);
//   (3.0 * t * t) - (2.0 * t * t * t)
// }

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
  },
  xyloid::{
    Device,
    d1,
    d2,
  },
};
