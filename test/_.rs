pub fn main() {
  let img = image::open("test.png").expect("Failed to open image");

  for pixel in img.pixels() {
    let result = check(pixel.2.to_rgba());

    match result {
      T => (),
      _ => println!("{:#?}", color(pixel.2.to_rgba())),
    }
  }
}

#[inline(always)]
fn check(z: Rgba<u8>) -> bool {
  let (n1, n2, n3, _) = color(z);

  match n1 >= 191 {
    T => match n3 >= 191 {
      T => match 16 >= n1.abs_diff(n3) {
        T => match n1 >= n3 {
          T => match n3 >= n2 {
            T => n3.abs_diff(n2) >= 32,
            _ => T,
          },
          _ => match n1 >= n2 {
            T => n1.abs_diff(n2) >= 32,
            _ => T,
          },
        },
        _ => T,
      },
      _ => T,
    },
    _ => T,
  }
}

#[inline(always)]
fn color(z: Rgba<u8>) -> (u8, u8, u8, u8) {
  (z[0], z[1], z[2], z[3])
}

use {
  common::{
    F,
    T,
  },
  image::{
    GenericImageView,
    Pixel,
    Rgba,
  },
};
