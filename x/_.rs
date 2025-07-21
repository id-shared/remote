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

  let (i2, o2): (Sender<(u32, i32, i32)>, Receiver<(u32, i32, i32)>) = bounded(64);
  handle.push(thread::spawn(
    #[inline(always)]
    move || match xyloid::type_1() {
      Some(io) => {
        // #[inline(always)]
        // fn zz(io: xyloid::HANDLE, i: i32, n: i32, x: f64, y: f64) -> bool {
        //   match i <= n {
        //     T => {
        //       let rr = ease(i as f64 / n as f64);
        //       let ay = rr * y;
        //       let ax = rr * x;

        //       // println!("{} {} | {:.2} {} {}", i, n, rr, ax, ay);

        //       xyloid::xy(io, ax, ay);
        //       xo(MS * HZ);
        //       zz(io, i + 1, n, x - ax, y - ay)
        //     },
        //     _ => {
        //       xyloid::xy(io, x, y);
        //       match xyloid::is_h() {
        //         T => T,
        //         _ => xyloid::key_h(io, F),
        //       }
        //     },
        //   }
        // }

        // #[inline(always)]
        // fn ease(t: f64) -> f64 {
        //   let t = t.clamp(0.0, 1.0);
        //   (3.0 * t * t) - (2.0 * t * t * t)
        // }

        #[inline(always)]
        fn f_zn(n1: u32) -> u32 {
          match n1 {
            61..=u32::MAX => 16,
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
        fn f_yn(n1: u32) -> f64 {
          (f_zn(n1) as f64) / 2.
        }

        #[inline(always)]
        fn f_xn(n1: u32) -> f64 {
          (f_zn(n1) as f64) / 4.
        }

        const BS: i32 = 64;

        let yy = #[inline(always)]
        |n: u32, n1: i32| fy(n1 as f64 - f_yn(n)) as i32;
        let xx = #[inline(always)]
        |n: u32, n1: i32| fx(n1 as f64 + f_xn(n)) as i32;
        let zz = #[inline(always)]
        |n: u32, x: i32, y: i32| {
          let ny = yy(n, y);
          let nx = xx(n, x);

          println!("Abc: {}", nx.abs());

          match BS >= nx.abs() {
            T => {
              xyloid::xy(io, nx as f64, ny as f64);
              xo(MS * 4);
              match xyloid::is_h() {
                T => T,
                _ => xyloid::key_h(io, F),
              }
            },
            _ => {
              let ay = ny.clamp(-1 * BS, BS);
              let ax = nx.clamp(-1 * BS, BS);

              xyloid::xy(io, ax as f64, ay as f64)
            },
          }
        };

        // for i in 1..=10 {
        //   let n = i as f64 / 10.;
        //   println!("{:.2}", ease(n));
        // }

        let mut time = Instant::now();
        let mut curr = N;

        while let Ok((an, ax, ay)) = o2.recv() {
          // let yy = yy(an, ay);
          // let xx = xx(an, ax);

          // TODO: difference should be atleast 2.

          curr = match time.elapsed().as_millis_f64() < 128. {
            T => {
              time = Instant::now();
              curr + 1
            },
            _ => {
              time = Instant::now();
              N
            },
          };

          println!("Current: {}", curr);

          println!("{}, {}, {}, {}", curr, an, ax, ay);

          // zz(an, ax, ay);

          match curr {
            1..=u32::MAX => match curr % 2 {
              N => zz(an, ax, N as i32),
              _ => F,
            },
            _ => zz(an, ax, ay),
          };
        }
      },
      _ => {},
    },
  ));

  let (i1, o1): (Sender<bool>, Receiver<bool>) = bounded(64);
  handle.push(thread::spawn(
    #[inline(always)]
    move || match xyloid::type_1() {
      Some(io) => {
        let yy = #[inline(always)]
        |n1: f64| xyloid::xy(io, N as f64, fy(n1));

        let mut cy = N;

        while let Ok(a) = o1.recv() {
          cy = match a {
            T => match xyloid::is_h() {
              T => {
                yy(match cy {
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
      const CLR: u8 = 255 - 24;
      const ABS: u8 = 24;

      screen::watch(
        #[inline(always)]
        move |_a| match screen::name().contains(APP) {
          T => match xyloid::is_mouse_l() {
            T => {
              send(&i1, T);
              T
            },
            _ => {
              send(&i1, F);
              F
            },
          },
          _ => F,
        },
        #[inline(always)]
        move |x| {
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
        },
        #[inline(always)]
        move |x| send(&i2, x),
        zx,
        zy,
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
        _ => F,
      };
    },
  ));

  #[inline(always)]
  fn on_key<F1: Fn() -> bool, F2: Fn(xyloid::HANDLE, bool) -> bool>(f1: F1, f2: F2, io: xyloid::HANDLE, z1: BI) -> BI {
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

  #[inline(always)]
  fn calc(radian: f64, factor: f64, size: f64) -> f64 {
    (tan(radian, factor) / (2. * PI)) * size
  }

  #[inline(always)]
  fn send<T>(i: &Sender<T>, o: T) -> bool {
    i.try_send(o).is_ok()
  }

  #[inline(always)]
  fn tan(n1: f64, n2: f64) -> f64 {
    (n1.tan() * n2).atan()
  }

  #[inline(always)]
  fn fov(n: f64) -> f64 {
    (n / 2.).to_radians()
  }

  for x in handle {
    x.join().unwrap();
  }
}

#[inline(always)]
pub fn xo(n: Duration) -> bool {
  thread::sleep(n);
  T
}

pub type BI = (bool, Instant);

pub const APP: &str = "VAL";

pub const MS: Duration = Duration::from_millis(1);
pub const HZ: u32 = 16;

pub const N: u32 = 0;
pub const F: bool = false;
pub const T: bool = true;

use {
  crossbeam::channel::{
    Receiver,
    Sender,
    bounded,
  },
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
};
