#![feature(duration_millis_float)]
#![feature(stmt_expr_attributes)]
#![feature(trait_alias)]

pub fn watch<F: Func<bool, bool>, G: Func<u32, bool>, H: Func<(u32, i32, i32), bool>>(f: F, g: G, h: H, x: f64, y: f64) -> () {
  let mut handle = vec![];

  handle.push(thread::spawn(
    #[inline(always)]
    move || {
      let data = #[inline(always)]
      |x1: f64, y1: f64| data((x / 2.) - (x1 / 2.), (y / 2.) - (y1 / 2.), x1, y1);
      let on = move |x: Buffer| match f(T) {
        T => {
          let (nn, un, xn, yn) = x;

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

              match g(xn_) {
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

          println!("{}, {}, {}", an, -ax, ay);

          match is {
            T => h((an, -ax, ay)),
            _ => F,
          };

          T
        },
        _ => F,
      };

      each(on, data(x / 4., y / 8.));
    },
  ));

  fn turn(x: ID3D11Texture2D, z: &IO) -> Buffer {
    let high = z.y as usize;
    let wide = z.x as usize;
    let desc = D3D11_TEXTURE2D_DESC {
      Width: wide as u32,
      Height: high as u32,
      MipLevels: 1,
      ArraySize: 1,
      Format: DXGI_FORMAT_B8G8R8A8_UNORM,
      SampleDesc: DXGI_SAMPLE_DESC {
        Count: 1,
        Quality: 0,
      },
      Usage: D3D11_USAGE_STAGING,
      CPUAccessFlags: D3D11_CPU_ACCESS_READ.0 as u32,
      BindFlags: 0,
      MiscFlags: 0,
    };

    let mut texture: Option<ID3D11Texture2D> = None;
    unsafe { z.device.CreateTexture2D(&desc, None, Some(&mut texture)).unwrap() };

    let region = D3D11_BOX {
      left: z.l as u32,
      top: z.t as u32,
      front: 0,
      right: (z.l + z.x) as u32,
      bottom: (z.t + z.y) as u32,
      back: 1,
    };

    unsafe { z.context.CopySubresourceRegion(texture.as_ref().unwrap(), 0, 0, 0, 0, &x, 0, Some(&region)) };

    let mut mapped = D3D11_MAPPED_SUBRESOURCE::default();
    unsafe { z.context.Map(texture.as_ref().unwrap(), 0, D3D11_MAP_READ, 0, Some(&mut mapped)).unwrap() };

    let pitch = mapped.RowPitch as usize;
    let data_ptr = mapped.pData as *const u8;

    unsafe { z.context.Unmap(texture.as_ref().unwrap(), 0) };

    (data_ptr, pitch, wide, high)
  }

  fn each<G: Func<Buffer, bool>>(g: G, z: IO) -> bool {
    let mut time = Instant::now();
    let mut curr = N;
    loop {
      match T {
        T => {
          let framed = sure(
            || {
              let mut info: DXGI_OUTDUPL_FRAME_INFO = DXGI_OUTDUPL_FRAME_INFO::default();
              let mut data: Option<IDXGIResource> = None;
              match unsafe { z.framer.AcquireNextFrame(z.hz, &mut info, &mut data).is_ok() } {
                T => {
                  let data = data.unwrap();
                  let cast = data.cast().unwrap();
                  g(turn(cast, &z));
                  unsafe { z.framer.ReleaseFrame().unwrap() };
                  T
                },
                _ => F,
              }
            },
            MS * z.hz,
          );

          match framed {
            T => match time.elapsed().as_millis_f64() > 1000. {
              T => {
                println!("FPS: {}", curr);
                time = Instant::now();
                curr = N;
                T
              },
              _ => {
                curr = curr + 1;
                F
              },
            },
            _ => F,
          }
        },
        _ => F,
      };
    }
  }

  fn data(l: f64, t: f64, x: f64, y: f64) -> IO {
    let mut context: Option<ID3D11DeviceContext> = None;
    let mut device: Option<ID3D11Device> = None;
    let mut level = D3D_FEATURE_LEVEL_12_2;

    unsafe {
      D3D11CreateDevice(
        None,                             // pAdapter
        D3D_DRIVER_TYPE_HARDWARE,         // drivertype
        HMODULE::default(),               // software
        D3D11_CREATE_DEVICE_BGRA_SUPPORT, // flags
        None,                             // pfeaturelevels
        D3D11_SDK_VERSION,                // sdkversion
        Some(&mut device),                // ppdevice
        Some(&mut level),                 // pfeaturelevel
        Some(&mut context),               // ppimmediatecontext
      )
      .unwrap()
    };

    let device = device.unwrap();
    let context = context.unwrap();
    let device_cast: IDXGIDevice = device.cast().unwrap();
    let bridge: IDXGIAdapter = unsafe { device_cast.GetAdapter().unwrap() };
    let output: IDXGIOutput = unsafe { bridge.EnumOutputs(0).unwrap() };
    let output_cast: IDXGIOutput1 = output.cast().unwrap();
    let framer = unsafe { output_cast.DuplicateOutput(&device).unwrap() };

    IO {
      context,
      device,
      framer,
      hz: HZ,
      l,
      t,
      x,
      y,
    }
  }

  for x in handle {
    x.join().unwrap();
  }
}

pub struct IO {
  context: ID3D11DeviceContext,
  device: ID3D11Device,
  framer: IDXGIOutputDuplication,
  hz: u32,
  l: f64,
  t: f64,
  x: f64,
  y: f64,
}

pub type Buffer = (*const u8, usize, usize, usize);

fn sure<F: Fn() -> bool>(f1: F, n1: Duration) -> bool {
  let init = Instant::now();
  let back = f1();
  let rest = init.elapsed();
  match back && n1 > rest {
    T => {
      sleep(n1 - rest);
      back
    },
    F => back,
  }
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

pub trait Func<T1, T2> = Fn(T1) -> T2 + Send + Sync + 'static;

pub const APP: &str = "VAL";

pub const MS: Duration = Duration::from_millis(1);
pub const HZ: u32 = 16;

pub const N: u32 = 0;
pub const F: bool = false;
pub const T: bool = true;

use {
  image::{
    GenericImageView,
    Pixel,
  },
  std::{
    thread::{
      self,
      sleep,
    },
    time::{
      Duration,
      Instant,
    },
    u32,
    usize,
  },
  windows::{
    Win32::{
      Foundation::{
        HMODULE,
        HWND,
      },
      Graphics::{
        Direct3D::{
          D3D_DRIVER_TYPE_HARDWARE,
          D3D_FEATURE_LEVEL_12_2,
        },
        Direct3D11::{
          D3D11_BOX,
          D3D11_CPU_ACCESS_READ,
          D3D11_CREATE_DEVICE_BGRA_SUPPORT,
          D3D11_MAP_READ,
          D3D11_MAPPED_SUBRESOURCE,
          D3D11_SDK_VERSION,
          D3D11_TEXTURE2D_DESC,
          D3D11_USAGE_STAGING,
          D3D11CreateDevice,
          ID3D11Device,
          ID3D11DeviceContext,
          ID3D11Texture2D,
        },
        Dxgi::{
          Common::{
            DXGI_FORMAT_B8G8R8A8_UNORM,
            DXGI_SAMPLE_DESC,
          },
          DXGI_OUTDUPL_FRAME_INFO,
          IDXGIAdapter,
          IDXGIDevice,
          IDXGIOutput,
          IDXGIOutput1,
          IDXGIOutputDuplication,
          IDXGIResource,
        },
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
    core::*,
  },
};

pub fn test() {
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

// let row_pitch = mapped.RowPitch as usize;
// let ptr = mapped.pData as *const u8;

// let mut img = ImageBuffer::<Rgba<u8>, Vec<u8>>::new(desc.Width, desc.Height);
// let buffer = img.as_mut();

// let width = desc.Width as usize;
// let height = desc.Height as usize;

// unsafe {
//   for y in 0..height {
//     let src_row = ptr.add(y * row_pitch);
//     let dst_row = buffer.as_mut_ptr().add(y * width * 4);

//     for x in 0..width {
//       let src_px = src_row.add(x * 4);
//       let dst_px = dst_row.add(x * 4);

//       *dst_px = *src_px.add(2); // R
//       *dst_px.add(1) = *src_px.add(1); // G
//       *dst_px.add(2) = *src_px; // B
//       *dst_px.add(3) = *src_px.add(3); // A
//     }
//   }
// }

// img.save("subregion.png").unwrap();
