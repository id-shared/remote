#![feature(duration_millis_float)]
#![feature(stmt_expr_attributes)]
#![feature(trait_alias)]

pub fn watch<F: FnMut((bool, f64, f64, f64, f64)) -> bool, F1: FnMut(Record) -> (bool, f64, f64, f64), F2: FnMut() -> bool>(mut f: F, mut on_f: F1, mut is_f: F2, n: u32, x: f64, y: f64) -> bool {
  let recorder_1 = recorder(n);

  let supplier_4 = supplier(ltxy((16., x), (8., y)), &recorder_1);
  let supplier_3 = supplier(ltxy((12., x), (8., y)), &recorder_1);
  let supplier_2 = supplier(ltxy((8., x), (8., y)), &recorder_1);
  let supplier_1 = supplier(ltxy((4., x), (8., y)), &recorder_1);

  let mut id: f64 = N;
  loop {
    let oneach = || {
      let mut info: DXGI_OUTDUPL_FRAME_INFO = DXGI_OUTDUPL_FRAME_INFO::default();
      let mut data: Option<IDXGIResource> = None;
      match unsafe { recorder_1.framer.AcquireNextFrame(recorder_1.hz, &mut info, &mut data) } {
        Ok(_) => match is_f() {
          T => {
            let supplier = match (id + 1.) / 4. {
              3.0..=f64::MAX => &supplier_4,
              2.0..=3. => &supplier_3,
              1.0..=2. => &supplier_2,
              0.0..=1. => &supplier_1,
              _ => &supplier_1,
            };

            let data = data.unwrap();
            let cast = data.cast().unwrap();

            let (is, an, ax, ay) = on_f(each(cast, supplier, &recorder_1));
            unsafe { recorder_1.framer.ReleaseFrame().unwrap() };

            match is {
              T => {
                f((is, id, an, ax, ay));
                id = id + 1.;
                T
              },
              _ => {
                f((is, id, an, ax, ay));
                id = id + 1.;
                T
              },
            }
          },
          _ => {
            unsafe { recorder_1.framer.ReleaseFrame().unwrap() };
            id = N;
            F
          },
        },
        Err(e) => {
          println!("Frame: {}", e);
          F
        },
      }
    };

    time::sure(oneach, time::MS * recorder_1.hz);
  }
}

fn each(d: ID3D11Texture2D, v: &Supplier, z: &Recorder) -> Record {
  let texture = &v.texture;
  let region = v.region;

  unsafe { z.context.CopySubresourceRegion(texture.as_ref().unwrap(), 0, 0, 0, 0, &d, 0, Some(&region)) };

  let mut mapped = D3D11_MAPPED_SUBRESOURCE::default();
  unsafe { z.context.Map(texture.as_ref().unwrap(), 0, D3D11_MAP_READ, 0, Some(&mut mapped)).unwrap() };

  let pitch = mapped.RowPitch as usize;
  let data_ptr = mapped.pData as *const u8;

  unsafe { z.context.Unmap(texture.as_ref().unwrap(), 0) };

  (data_ptr, pitch, v.x as usize, v.y as usize)
}

fn supplier(v: (f64, f64, f64, f64), z: &Recorder) -> Supplier {
  let (l, t, x, y) = v;
  let desc = D3D11_TEXTURE2D_DESC {
    Width: x as u32,
    Height: y as u32,
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
    left: l as u32,
    top: t as u32,
    front: 0,
    right: (l + x) as u32,
    bottom: (t + y) as u32,
    back: 1,
  };

  Supplier {
    texture,
    region,
    y: y,
    x: x,
  }
}

struct Supplier {
  texture: Option<ID3D11Texture2D>,
  region: D3D11_BOX,
  y: f64,
  x: f64,
}

fn recorder(n: u32) -> Recorder {
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

  Recorder {
    context,
    device,
    framer,
    hz: n,
  }
}

struct Recorder {
  context: ID3D11DeviceContext,
  device: ID3D11Device,
  framer: IDXGIOutputDuplication,
  hz: u32,
}

fn ltxy(x: (f64, f64), y: (f64, f64)) -> (f64, f64, f64, f64) {
  let (y1, y2) = y;
  let (x1, x2) = x;
  let y3 = y2 / y1;
  let x3 = x2 / x1;
  ((x2 / 2.) - (x3 / 2.), (y2 / 2.) - (y3 / 2.), x3, y3)
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

type Record = (*const u8, usize, usize, usize);

use {
  common::{
    F,
    N,
    T,
    time,
  },
  std::{
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
