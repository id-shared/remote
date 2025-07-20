#![feature(duration_millis_float)]
#![feature(stmt_expr_attributes)]
#![feature(trait_alias)]

pub trait Is<T> = Functor<T, bool>;
pub trait At<T> = Functor<T, i32>;
pub fn watch<F1: At<i32>, F2: Is<u32>, F3: Is<(i32, i32, i32, i32)>>(f1: F1, f2: F2, f3: F3, x: f64, y: f64) -> () {
  let mut handle = vec![];

  handle.push(thread::spawn(
    #[inline(always)]
    move || {
      let data = #[inline(always)]
      |x1: f64, y1: f64| data((x / 2.) - (x1 / 2.), (y / 2.) - (y1 / 2.), x1, y1);

      each(data(x / 4., y / 8.), (Instant::now(), 0));
    },
  ));

  for x in handle {
    x.join().unwrap();
  }
}

pub fn turn(io: &IO, source_texture: &ID3D11Texture2D) {
  let mut staging_texture: Option<ID3D11Texture2D> = None;
  let desc = D3D11_TEXTURE2D_DESC {
    Width: io.x as u32,
    Height: io.y as u32,
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

  unsafe { io.device.CreateTexture2D(&desc, None, Some(&mut staging_texture)).unwrap() };

  let region = D3D11_BOX {
    left: io.l as u32,
    top: io.t as u32,
    front: 0,
    right: (io.l + io.x) as u32,
    bottom: (io.t + io.y) as u32,
    back: 1,
  };

  unsafe { io.context.CopySubresourceRegion(staging_texture.as_ref().unwrap(), 0, 0, 0, 0, source_texture, 0, Some(&region)) };

  let mut mapped = D3D11_MAPPED_SUBRESOURCE::default();
  unsafe { io.context.Map(staging_texture.as_ref().unwrap(), 0, D3D11_MAP_READ, 0, Some(&mut mapped)).unwrap() };

  let row_pitch = mapped.RowPitch as usize;
  let ptr = mapped.pData as *const u8;

  let mut img = ImageBuffer::<Rgba<u8>, Vec<u8>>::new(desc.Width, desc.Height);
  let buffer = img.as_mut();

  let width = desc.Width as usize;
  let height = desc.Height as usize;

  unsafe {
    for y in 0..height {
      let src_row = ptr.add(y * row_pitch);
      let dst_row = buffer.as_mut_ptr().add(y * width * 4);

      for x in 0..width {
        let src_px = src_row.add(x * 4);
        let dst_px = dst_row.add(x * 4);

        *dst_px = *src_px.add(2); // R
        *dst_px.add(1) = *src_px.add(1); // G
        *dst_px.add(2) = *src_px; // B
        *dst_px.add(3) = *src_px.add(3); // A
      }
    }
  }

  img.save("subregion.png").unwrap();

  unsafe { io.context.Unmap(staging_texture.as_ref().unwrap(), 0) };
}

pub fn each(io: IO, p: (Instant, u32)) -> bool {
  match name().contains("") {
    T => {
      let (time, curr) = p;

      match sure(
        || {
          let mut info: DXGI_OUTDUPL_FRAME_INFO = DXGI_OUTDUPL_FRAME_INFO::default();
          let mut data: Option<IDXGIResource> = None;
          match unsafe { io.framer.AcquireNextFrame(HZ, &mut info, &mut data).is_ok() } {
            T => {
              let data = data.unwrap();
              let cast = data.cast().unwrap();
              turn(&io, &cast);
              unsafe { io.framer.ReleaseFrame().unwrap() };
              T
            },
            _ => F,
          }
        },
        MS * HZ,
      ) {
        T => match time.elapsed().as_millis_f64() > 1000. {
          T => {
            println!("FPS: {}", curr);
            each(io, (Instant::now(), 0))
          },
          _ => each(io, (time, curr + 1)),
        },
        _ => each(io, (time, curr)),
      }
    },
    _ => F,
  }
}

pub fn data(l: f64, t: f64, x: f64, y: f64) -> IO {
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
    l,
    t,
    x,
    y,
  }
}

pub struct IO {
  context: ID3D11DeviceContext,
  device: ID3D11Device,
  framer: IDXGIOutputDuplication,
  l: f64,
  t: f64,
  x: f64,
  y: f64,
}

fn sure<F: FnOnce() -> bool>(f1: F, n1: Duration) -> bool {
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

pub const APP: &str = "VAL";

pub const MS: Duration = Duration::from_millis(1);
pub const HZ: u32 = 16;

pub const N: i32 = 0;
pub const F: bool = false;
pub const T: bool = true;

pub trait Functor<T1, T2> = Fn(T1) -> T2 + Send + Sync + 'static;
use {
  image::{
    GenericImageView,
    ImageBuffer,
    Pixel,
    Rgba,
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
    usize,
  },
  windows::{
    Win32::{
      Foundation::{
        HMODULE,
        HWND,
      },
      Graphics::{
        Direct3D::*,
        Direct3D11::*,
        Dxgi::{
          Common::*,
          *,
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
