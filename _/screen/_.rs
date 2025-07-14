#![feature(duration_millis_float)]
#![feature(stmt_expr_attributes)]
#![feature(trait_alias)]

pub trait Is<T> = Functor<T, bool>;
pub trait At<T> = Functor<T, i32>;
pub fn watch<F1: At<i32>, F2: Is<u32>, F3: Is<(i32, i32, i32, i32)>>(f1: F1, f2: F2, f3: F3, x: i32, y: i32) -> () {
  let mut handle = vec![];

  let (i1, o1): (Sender<(i32, i32, i32, Vec<u8>)>, Receiver<(i32, i32, i32, Vec<u8>)>) = bounded(64);
  handle.push(thread::spawn(
    #[inline(always)]
    move || {
      let px = #[inline(always)]
      |x: &[u32], z: i32| unsafe { *x.get_unchecked(z as usize) };

      while let Ok((az, nx, ny, buffer)) = o1.recv() {
        let pixel = unsafe { std::slice::from_raw_parts(buffer.as_ptr() as *const u32, buffer.len() / 4) };
        let mut z: i32 = 0;
        'y: for y in 0..ny {
          for x in 0..nx {
            z = match f2(px(pixel, z)) {
              T => {
                let ay = (ny / 2) - y;
                let ax = (nx / 2) - x;
                let mut at = 0;
                'x: for n in 1..(nx - x) {
                  at = match f2(px(pixel, z + n)) {
                    T => at + 1,
                    _ => {
                      break 'x;
                    },
                  };
                }
                f3((at, -ax, ay, az));
                break 'y;
              },
              _ => z + 1,
            };
          }
        }
      }
    },
  ));

  handle.push(thread::spawn(
    #[inline(always)]
    move || {
      let zone_y = y / 2;
      let zone_x = x / 2;
      let data_2 = detail(zone_x, zone_y, zone_x / 4, zone_y / 8);
      let data_1 = detail(zone_x, zone_y, zone_x / 4, zone_y / 8);
      let mut at = 0;
      loop {
        // TODO: make sure only 1 SS every 16ms.
        at = f1(at);

        match at {
          17..=i32::MAX => sure(|| send(&i1, (at, data_2.nx, data_2.ny, screen(&data_2))), MS * 16),
          1..=16 => sure(|| send(&i1, (at, data_1.nx, data_1.ny, screen(&data_1))), MS * 16),
          _ => xo(MS),
        };
      }
    },
  ));

  for x in handle {
    x.join().unwrap();
  }
}

fn sure<T, F: FnOnce() -> T>(f1: F, n1: Duration) -> T {
  let start = Instant::now();
  let back = f1();
  let rest = start.elapsed();
  match rest < n1 {
    T => {
      sleep(n1 - rest);
      back
    },
    F => back,
  }
}

#[inline(always)]
pub fn screen(detail_1: &Data) -> Vec<u8> {
  let mut buffer = vec![0u8; detail_1.az];

  unsafe { BitBlt(detail_1.memory, 0, 0, detail_1.nx, detail_1.ny, detail_1.screen, detail_1.ax, detail_1.ay, SRCCOPY).unwrap() };

  unsafe { GetBitmapBits(detail_1.bitmap, detail_1.az as i32, buffer.as_mut_ptr().cast()) };

  buffer
}

#[inline(always)]
pub fn name() -> String {
  match unsafe { GetForegroundWindow() } {
    HWND(ptr) => match ptr.is_null() as usize {
      1 => String::new(),
      _ => match unsafe { GetWindowTextLengthW(HWND(ptr)) } {
        0 => String::new(),
        n => {
          let mut buffer = vec![0u16; (n + 1) as usize];
          match unsafe { GetWindowTextW(HWND(ptr), &mut buffer) } {
            0 => String::new(),
            copied => match String::from_utf16(&buffer[..copied as usize]) {
              Ok(s) => s,
              Err(_) => String::new(),
            },
          }
        },
      },
    },
  }
}

#[inline(always)]
pub fn detail(zx: i32, zy: i32, nx: i32, ny: i32) -> Data {
  let screen = Some(unsafe { GetDC(Some(HWND::default())) });
  let memory = unsafe { CreateCompatibleDC(screen) };

  let bitmap = unsafe { CreateCompatibleBitmap(screen.unwrap(), nx, ny) };
  unsafe { SelectObject(memory, bitmap.into()) };

  let az = ((ny * nx) as usize) * size_of::<u32>();
  let ay = zy - (ny / 2);
  let ax = zx - (nx / 2);

  Data {
    screen,
    memory,
    bitmap,
    az,
    ny,
    nx,
    ay,
    ax,
  }
}

#[inline(always)]
pub fn send<T>(i: &Sender<T>, o: T) -> bool {
  i.try_send(o).is_ok()
}

#[inline(always)]
pub fn wide() -> f64 {
  unsafe { GetSystemMetrics(SM_CXSCREEN) as f64 }
}

#[inline(always)]
pub fn high() -> f64 {
  unsafe { GetSystemMetrics(SM_CYSCREEN) as f64 }
}

#[inline(always)]
pub fn xo(n: Duration) -> bool {
  thread::sleep(n);
  T
}

pub const MS: Duration = Duration::from_millis(1);
pub const F: bool = false;
pub const T: bool = true;

pub struct Data {
  bitmap: HBITMAP,
  screen: Option<HDC>,
  memory: HDC,
  az: usize,
  nx: i32,
  ny: i32,
  ay: i32,
  ax: i32,
}

pub fn test() {
  let img = image::open("test.png").expect("Failed to open image");

  for pixel in img.pixels() {
    let rgba = pixel.2.to_rgba();
    let n1 = rgba[0];
    let n2 = rgba[1];
    let n3 = rgba[2];

    let result = match n1 >= 231 && 231 >= n2 && n3 >= 231 {
      T => n1.min(n3) >= n2 && n1.min(n3).abs_diff(n2) >= 24,
      _ => T,
    };

    match result {
      T => (),
      _ => println!("FA: {}, {}, {}, {}", 1, n1, n2, n3),
    }
  }
}

pub trait Functor<T1, T2> = Fn(T1) -> T2 + Send + Sync + 'static;
use {
  crossbeam::channel::{
    Receiver,
    Sender,
    bounded,
  },
  image::{
    GenericImageView,
    Pixel,
  },
  std::{
    mem::size_of,
    thread::{
      self,
      sleep,
    },
    time::{
      Duration,
      Instant,
    },
    usize,
  },
  windows::Win32::{
    Foundation::HWND,
    Graphics::Gdi::{
      BitBlt,
      CreateCompatibleBitmap,
      CreateCompatibleDC,
      GetBitmapBits,
      GetDC,
      HBITMAP,
      HDC,
      SRCCOPY,
      SelectObject,
    },
    UI::WindowsAndMessaging::{
      GetForegroundWindow,
      GetSystemMetrics,
      GetWindowTextLengthW,
      GetWindowTextW,
      SM_CXSCREEN,
      SM_CYSCREEN,
    },
  },
};
