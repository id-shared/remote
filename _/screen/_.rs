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

      while let Ok((an, nx, ny, buffer)) = o1.recv() {
        let pixel = unsafe { std::slice::from_raw_parts(buffer.as_ptr() as *const u32, buffer.len() / 4) };

        let mut zz = 0;
        let mut az = 0;
        let mut ay = 0;
        let mut ax = 0;
        let mut aa = F;

        for y in 0..ny {
          let ay_ = (ny / 2) - y;

          'x: for x in 0..nx {
            az = match f2(px(pixel, az)) {
              T => {
                let ax_ = (nx / 2) - x;

                match aa {
                  T => {
                    zz = zz + 1;
                    az = az + (nx - x);
                    break 'x;
                  },
                  _ => {
                    zz = zz + 1;
                    az = az + (nx - x);
                    ay = ay_;
                    ax = ax_;
                    aa = T;
                    break 'x;
                  },
                }
              },
              _ => az + 1,
            }
          }
        }

        match aa {
          T => f3((an, -ax, ay, zz - 1)),
          _ => F,
        };
      }
    },
  ));

  handle.push(thread::spawn(
    #[inline(always)]
    move || {
      const HZ: Duration = Duration::from_millis(16);

      let data_2 = data(x / 8, y / 16, x, y);
      let data_1 = data(x / 4, y / 16, x, y);
      let mut an = 0;

      loop {
        an = f1(an);

        match an {
          2..=i32::MAX => sure(|| send(&i1, (an, data_2.nx, data_2.ny, screen(&data_2))), HZ),
          1 => sure(|| send(&i1, (an, data_1.nx, data_1.ny, screen(&data_1))), HZ),
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
pub fn data(nx: i32, ny: i32, zx: i32, zy: i32) -> Data {
  let screen = Some(unsafe { GetDC(Some(HWND::default())) });
  let memory = unsafe { CreateCompatibleDC(screen) };

  let bitmap = unsafe { CreateCompatibleBitmap(screen.unwrap(), nx, ny) };
  unsafe { SelectObject(memory, bitmap.into()) };

  let az = ((ny * nx) as usize) * size_of::<u32>();
  let ay = (zy / 2) - (ny / 2);
  let ax = (zx / 2) - (nx / 2);

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

pub fn test2() {
  use windows::{
    Win32::{
      Foundation::*,
      Graphics::{
        Direct3D::*,
        Direct3D11::*,
        Dxgi::{
          Common::*,
          *,
        },
      },
    },
    core::*,
  };

  fn main() {
    unsafe {
      let mut device: Option<ID3D11Device> = None;
      let mut context: Option<ID3D11DeviceContext> = None;
      let mut feature_level = D3D_FEATURE_LEVEL_11_0;

      D3D11CreateDevice(
        None,                             // pAdapter
        D3D_DRIVER_TYPE_HARDWARE,         // drivertype
        HMODULE::default(),               // software
        D3D11_CREATE_DEVICE_BGRA_SUPPORT, // flags
        None,                             // pfeaturelevels
        D3D11_SDK_VERSION,                // sdkversion
        Some(&mut device),                // ppdevice
        Some(&mut feature_level),         // pfeaturelevel
        Some(&mut context),               // ppimmediatecontext
      )
      .unwrap();

      let device = device.unwrap();
      let context = context.unwrap();

      // 2. Get DXGI Device.
      let dxgi_device: IDXGIDevice = device.cast().unwrap();

      // 3. Get DXGI Adapter
      let adapter: IDXGIAdapter = dxgi_device.GetAdapter().unwrap();

      // 4. Get Output (Monitor)
      let output: IDXGIOutput = adapter.EnumOutputs(0).unwrap();
      let output1: IDXGIOutput1 = output.cast().unwrap();

      // 5. Duplicate the output
      let duplication = output1.DuplicateOutput(&device).unwrap();

      println!("Device and context successfully created.");

      // Set region of interest (crop area)
      let left = 100;
      let top = 100;
      let width = 2560;
      let height = 1440;

      loop {
        // 6. Acquire next frame
        let mut frame_info = DXGI_OUTDUPL_FRAME_INFO::default();
        let mut resource: Option<IDXGIResource> = None;
        duplication.AcquireNextFrame(500, &mut frame_info, &mut resource).unwrap();

        // 7. Cast to ID3D11Texture2D
        let resource = resource.unwrap();
        let tex: ID3D11Texture2D = resource.cast().unwrap();

        // 8. Get texture description
        let mut desc = D3D11_TEXTURE2D_DESC::default();
        desc.Width = width;
        desc.Height = height;
        desc.MipLevels = 1;
        desc.ArraySize = 1;
        desc.Format = DXGI_FORMAT_B8G8R8A8_UNORM;
        desc.SampleDesc = DXGI_SAMPLE_DESC {
          Quality: 0,
          Count: 1,
        };
        tex.GetDesc(&mut desc);

        // 9. Create a CPU-readable texture to copy into
        desc.Usage = D3D11_USAGE_STAGING;
        desc.BindFlags = 0;
        desc.CPUAccessFlags = D3D11_CPU_ACCESS_READ.0 as u32;
        desc.MiscFlags = 0;

        println!("{}", desc.Height);

        let mut tex_cpu: Option<ID3D11Texture2D> = None;
        device.CreateTexture2D(&desc, None, Some(&mut tex_cpu)).unwrap();

        let tex_cpu = tex_cpu.unwrap();

        // 10. Copy from GPU texture to CPU-readable texture
        context.CopyResource(&tex_cpu, &tex);

        // 11. Map and access pixels
        let mut mapped = D3D11_MAPPED_SUBRESOURCE::default();
        context.Map(&tex_cpu, 0, D3D11_MAP_READ, 0, Some(&mut mapped)).unwrap();

        // mapped.pData is a *mut c_void pointing to pixel data
        println!("Mapped pitch: {}", mapped.RowPitch);

        // Don't forget to unmap and release frame
        context.Unmap(&tex_cpu, 0);

        println!("Frame captured.");

        let data = std::slice::from_raw_parts(mapped.pData as *const u8, (mapped.RowPitch * desc.Height) as usize);

        if let Some(img) = image::ImageBuffer::<image::Rgb<u8>, Vec<u8>>::from_raw(width, height, data.chunks_exact(4).flat_map(|p| [p[2], p[1], p[0]]).collect()) {
          img.save("a.png").unwrap();
        }

        duplication.ReleaseFrame().unwrap();

        xo(MS * 1000);
      }
    };
  }

  main();
}

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
    i32,
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

// Process BGRA data (4 bytes per pixel)
// for y in 0..desc.Height {
//   for x in 0..desc.Width {
//     let pixel_start = (y * mapped.RowPitch as u32 + x * 4) as usize;
//     let b = data[pixel_start];
//     let g = data[pixel_start + 1];
//     let r = data[pixel_start + 2];
//     let a = data[pixel_start + 3];

//     println!("{}, {}, {}, {}", b, g, r, a);
//   }
// }
