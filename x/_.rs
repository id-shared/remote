#![feature(duration_millis_float)]
#![feature(stmt_expr_attributes)]
#![feature(trait_alias)]

pub fn main() {
  println!("Angle for chord length 1 is {:.64} pixels", wealth(to_rad(103_f64 / 2.), 1. / 1280., 6400.));

  let mut zz = vec![];

  zz.push(thread::spawn(move || {
    let device = xyloid::device();
    let y_high = screen::high();
    let x_wide = screen::wide();

    let get_y_ = |ay: f64| wealth(to_rad(70.53_f64 / 2.), ay / (y_high / 2.), _360);
    let get_x_ = |ax: f64| wealth(to_rad(103.0_f64 / 2.), ax / (x_wide / 2.), _360);
    let xy = |ax: f64, ay: f64| d1::xy(get_x_(ax), get_y_(ay), &device);

    let is_kl = || d2::is_i();
    let kl = |is: bool| d2::i(is, &device);

    let mut at = time::now();
    let mut an = N;

    const COLOR_N_3: u8 = 255 - 4;
    const COLOR_N_2: u8 = 175;
    const COLOR_N_1: u8 = 4;

    const _360: f64 = 6400.;
    const FREQ: u32 = 18;

    #[inline(always)]
    fn is_pixel(n_1: u8, n_2: u8, n_3: u8, x: u32) -> bool {
      let n1 = ((x >> 16) & 0xff) as u8;
      let n2 = ((x >> 8) & 0xff) as u8;
      let n3 = (x & 0xff) as u8;

      match n1 >= n_3 && n2 >= n_3 {
        T => match n_1 >= n1.abs_diff(n2) {
          T => match n1 > n2 {
            T => match n2.abs_diff(n3) > n_2 {
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
    fn into(k: f64, l: f64, n: f64) -> (bool, f64) {
      let n_ = n.abs();

      match (l / 4.) >= n_ {
        T => (T, n),
        _ => match l >= n_ {
          T => (F, n / (k / 1.)),
          _ => (F, n / (k / 1.)),
        },
      }
    }

    #[inline(always)]
    fn add_y(n: f64) -> f64 {
      n / 8.
    }

    #[inline(always)]
    fn add_x(n: f64) -> f64 {
      n / 16.
    }

    screen::watch(
      |(a, _n, v, x, y)| match a {
        T => match is_kl() {
          T => {
            let ay = recoil(FREQ as f64, time::till(at));

            an = an + 1.;

            xy((x + add_x(v)) / 2., ay);
            T
          },
          _ => {
            let (__, zy) = into(4., y_high / 64., y - add_y(v));
            let (ax, zx) = into(4., x_wide / 64., x + add_x(v));

            at = time::now();
            an = N;

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
        },
        _ => match is_kl() {
          T => {
            at = time::now();
            an = an + 1.;

            xy(N, recoil(FREQ as f64, time::till(at)));
            T
          },
          _ => F,
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

            match is_pixel(COLOR_N_1, COLOR_N_2, COLOR_N_3, nx) {
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
      || match screen::name().contains("VAL") && d2::is_ml() {
        T => T,
        _ => {
          kl(T);
          F
        },
      },
      FREQ,
      x_wide,
      y_high,
    );
  }));

  zz.push(thread::spawn(|| {
    let io = xyloid::device();
    let mut d = (F, time::now());
    let mut a = (F, time::now());
    let mut w = (F, time::now());
    let mut s = (F, time::now());

    #[inline(always)]
    fn on_key<F1: Fn() -> bool, F2: Fn(bool, &Device) -> bool>(f1: F1, f2: F2, io: &Device, z1: BI) -> BI {
      on(
        f1,
        |_| (T, time::now()),
        |x| {
          let n = (time::till(x.1) / 10.).round() as u64;
          match n {
            17..=32 => {
              f2(F, io);
              time::rest(time::MS * ((4 * 16) + ((n - 16) * 2)) as u32);
              f2(T, io)
            },
            6..=16 => {
              f2(F, io);
              time::rest(time::MS * (4 * n) as u32);
              f2(T, io)
            },
            0..=5 => T,
            _ => {
              f2(F, io);
              time::rest(time::MS * 96);
              f2(T, io)
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
      match screen::name().contains("VAL") {
        T => {
          d = on_key(d2::is_d, d2::al, &io, d);
          a = on_key(d2::is_a, d2::ar, &io, a);
          w = on_key(d2::is_w, d2::ad, &io, w);
          s = on_key(d2::is_s, d2::au, &io, s);
          time::rest(time::MS);
          T
        },
        _ => F,
      };
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
