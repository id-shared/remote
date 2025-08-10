#![feature(duration_millis_float)]
#![feature(stmt_expr_attributes)]
#![feature(trait_alias)]

pub fn watch<F: FnMut((bool, u64, u64, i64, i64)) -> u64, F1: FnMut(Record) -> (bool, u64, i64, i64), F2: FnMut() -> bool>(mut f: F, mut on_f: F1, mut is_f: F2, n: u64, x: u64, y: u64) -> bool {
  let recorder_1 = recorder(n);

  let mut supplier_n: HashMap<u64, Supplier> = HashMap::new();

  for n in 0..=255 {
    make(n, 4, 1, 8, x, y, &recorder_1, &mut supplier_n);
  }

  let mut id: u64 = 0;
  loop {
    let oneach = || {
      let mut info: DXGI_OUTDUPL_FRAME_INFO = DXGI_OUTDUPL_FRAME_INFO::default();
      let mut data: Option<IDXGIResource> = None;
      match unsafe { recorder_1.framer.AcquireNextFrame(u32::try_from(recorder_1.hz).unwrap(), &raw mut info, &raw mut data) } {
        Ok(()) => match is_f() {
          T => {
            let supplier = match id {
              64..=u64::MAX => supplier_n.get(&16).unwrap(),
              0..=63 => match id.is_multiple_of(2) {
                T => supplier_n.get(&(id / 4)).unwrap(),
                _ => supplier_n.get(&255).unwrap(),
              },
            };

            let data = data.unwrap();
            let cast = data.cast().unwrap();

            let (is, an, ax, ay) = on_f(each(&cast, supplier, &recorder_1));
            unsafe { recorder_1.framer.ReleaseFrame().unwrap() };

            id = f((is, id, an, ax, ay));
            T
          },
          _ => {
            unsafe { recorder_1.framer.ReleaseFrame().unwrap() };
            id = 0;
            F
          },
        },
        Err(_e) => {
          // println!("Frame: {}", _e);
          F
        },
      }
    };

    time::ms_sure(oneach, recorder_1.hz);
  }
}

fn make(k1: u64, k2: u64, l1: u64, l2: u64, n1: u64, n2: u64, x: &Recorder, z: &mut HashMap<u64, Supplier>) {
  z.insert(k1, supplier(ltxy(((k1 * l1) + k2, n1), (l2, n2)), x));
}

fn each(d: &ID3D11Texture2D, v: &Supplier, z: &Recorder) -> Record {
  let texture = &v.texture;
  let region = v.region;

  unsafe { z.context.CopySubresourceRegion(texture.as_ref().unwrap(), 0, 0, 0, 0, d, 0, Some(&raw const region)) };

  let mut mapped = D3D11_MAPPED_SUBRESOURCE::default();
  unsafe { z.context.Map(texture.as_ref().unwrap(), 0, D3D11_MAP_READ, 0, Some(&raw mut mapped)).unwrap() };

  let pitch = mapped.RowPitch as usize;
  let data_ptr = mapped.pData as *const u32;

  unsafe { z.context.Unmap(texture.as_ref().unwrap(), 0) };

  println!("{pitch}");

  (data_ptr, pitch / 4, usize::try_from(v.x).unwrap(), usize::try_from(v.y).unwrap())
}

fn supplier(v: (u64, u64, u64, u64), z: &Recorder) -> Supplier {
  let l = u32::try_from(v.0).unwrap();
  let t = u32::try_from(v.1).unwrap();
  let x = u32::try_from(v.2).unwrap();
  let y = u32::try_from(v.3).unwrap();

  let desc = D3D11_TEXTURE2D_DESC {
    Width: x,
    Height: y,
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
  unsafe { z.device.CreateTexture2D(&raw const desc, None, Some(&raw mut texture)).unwrap() };

  let region = D3D11_BOX {
    left: l,
    top: t,
    front: 0,
    right: l + x,
    bottom: t + y,
    back: 1,
  };

  Supplier {
    texture,
    region,
    y: v.3,
    x: v.2,
  }
}

struct Supplier {
  texture: Option<ID3D11Texture2D>,
  region: D3D11_BOX,
  y: u64,
  x: u64,
}

fn recorder(n: u64) -> Recorder {
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
      Some(&raw mut device),            // ppdevice
      Some(&raw mut level),             // pfeaturelevel
      Some(&raw mut context),           // ppimmediatecontext
    )
  }
  .unwrap();

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
  hz: u64,
}

const fn ltxy(x: (u64, u64), y: (u64, u64)) -> (u64, u64, u64, u64) {
  let (y1, y2) = y;
  let (x1, x2) = x;
  let y3 = y2 / y1;
  let x3 = x2 / x1;
  ((x2 / 2) - (x3 / 2), (y2 / 2) - (y3 / 2), x3, y3)
}

pub fn name() -> String {
  match unsafe { GetForegroundWindow() } {
    HWND(ptr) => match ptr.is_null() {
      T => String::new(),
      _ => match unsafe { GetWindowTextLengthW(HWND(ptr)) } {
        0 => String::new(),
        n => {
          let mut buffer = vec![0u16; (n + 1).unsigned_abs() as usize];
          match unsafe { GetWindowTextW(HWND(ptr), &mut buffer) } {
            0 => String::new(),
            copied => String::from_utf16(&buffer[..copied.unsigned_abs() as usize]).unwrap_or_default(),
          }
        },
      },
    },
  }
}

#[allow(clippy::cast_sign_loss)]
pub fn wide() -> u64 {
  (unsafe { GetSystemMetrics(SM_CXSCREEN) }) as u64
}

#[allow(clippy::cast_sign_loss)]
pub fn high() -> u64 {
  (unsafe { GetSystemMetrics(SM_CYSCREEN) }) as u64
}

type Record = (*const u32, usize, usize, usize);

use {
  common::{
    F,
    T,
    time,
  },
  std::collections::HashMap,
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
    core::Interface,
  },
};
