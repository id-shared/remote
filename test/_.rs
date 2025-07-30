pub fn main() {
  let img = image::open("test.png").expect("Failed to open image");

  const TINT: u8 = 255 - 24;
  const DIFF: u8 = 24;

  for pixel in img.pixels() {
    let rgba = pixel.2.to_rgba();
    let n1 = rgba[0];
    let n2 = rgba[1];
    let n3 = rgba[2];

    let result = match n1 > TINT && TINT > n2 && n3 > TINT {
      T => match n1 > n3 {
        T => match n3.abs_diff(n2) > DIFF {
          T => T,
          _ => F,
        },
        _ => match n1.abs_diff(n2) > DIFF {
          T => T,
          _ => F,
        },
      },
      _ => F,
    };

    match result {
      T => (),
      _ => println!("FA: {}, {}, {}", n1, n2, n3),
    }
  }
}

#[inline(always)]
fn check(x: u32) -> bool {
  let n1 = ((x >> 16) & 0xff) as u8;
  let n2 = ((x >> 8) & 0xff) as u8;
  let n3 = (x & 0xff) as u8;

  match n1 >= 254 {
    T => match n3 >= 254 {
      T => match 16 >= n1.abs_diff(n3) {
        T => match n1 >= n3 {
          T => n1.abs_diff(n2) >= 48,
          _ => n3.abs_diff(n2) >= 48,
        },
        _ => F,
      },
      _ => F,
    },
    _ => F,
  }
}

use {
  common::{
    F,
    T,
  },
  image::{
    GenericImageView,
    Pixel,
  },
};
