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
      let data_2 = data(x / 8, y / 16, x, y);
      let data_1 = data(x / 4, y / 16, x, y);
      let freq_1 = MS * HZ;
      let mut an = 0;

      loop {
        an = f1(an);

        match an {
          2..=i32::MAX => sure(|| send(&i1, (an, data_2.nx, data_2.ny, screen(&data_2))), freq_1),
          1 => sure(|| send(&i1, (an, data_1.nx, data_1.ny, screen(&data_1))), freq_1),
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
  fn main() {
    unsafe {
      let mut device_context: Option<ID3D11DeviceContext> = None;
      let mut device: Option<ID3D11Device> = None;
      let mut feature_level = D3D_FEATURE_LEVEL_12_2;

      D3D11CreateDevice(
        None,                             // pAdapter
        D3D_DRIVER_TYPE_HARDWARE,         // drivertype
        HMODULE::default(),               // software
        D3D11_CREATE_DEVICE_BGRA_SUPPORT, // flags
        None,                             // pfeaturelevels
        D3D11_SDK_VERSION,                // sdkversion
        Some(&mut device),                // ppdevice
        Some(&mut feature_level),         // pfeaturelevel
        Some(&mut device_context),        // ppimmediatecontext
      )
      .unwrap();

      let device_context = device_context.unwrap();
      let device = device.unwrap();
      let device_cast: IDXGIDevice = device.cast().unwrap();
      let bridge: IDXGIAdapter = device_cast.GetAdapter().unwrap();
      let output: IDXGIOutput = bridge.EnumOutputs(0).unwrap();
      let output_cast: IDXGIOutput1 = output.cast().unwrap();
      let framer = output_cast.DuplicateOutput(&device).unwrap();

      let wide = wide();
      let high = high();

      let ny = high / 8.;
      let nx = wide / 4.;
      let my = (high / 2.) - (ny / 2.);
      let mx = (wide / 2.) - (nx / 2.);

      let desc = D3D11_TEXTURE2D_DESC {
        Width: nx as u32,
        Height: ny as u32,
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
      let mut time = Instant::now();
      let mut curr = 0;
      loop {
        sure(
          || {
            let mut meta = DXGI_OUTDUPL_FRAME_INFO::default();
            let mut data: Option<IDXGIResource> = None;
            match framer.AcquireNextFrame(HZ, &mut meta, &mut data).is_ok() {
              T => {
                let data = data.unwrap();
                let data_cast: ID3D11Texture2D = data.cast().unwrap();
                capture_box_to_png(&device, &device_context, &data_cast, &desc, mx as u32, my as u32);

                framer.ReleaseFrame().unwrap();

                curr = curr + 1;
                if time.elapsed().as_millis_f64() > 1000. {
                  println!("FPS: {}", curr);
                  time = Instant::now();
                  curr = 0;
                }
                T
              },
              _ => F,
            };
          },
          MS * HZ,
        );
      }
    };
  }

  main();
}

pub fn capture_box_to_png(device: &ID3D11Device, device_context: &ID3D11DeviceContext, source_texture: &ID3D11Texture2D, desc: &D3D11_TEXTURE2D_DESC, offset_x: u32, offset_y: u32) {
  let mut staging_texture: Option<ID3D11Texture2D> = None;

  unsafe { device.CreateTexture2D(desc, None, Some(&mut staging_texture)).unwrap() };

  let region = D3D11_BOX {
    left: offset_x,
    top: offset_y,
    front: 0,
    right: offset_x + desc.Width,
    bottom: offset_y + desc.Height,
    back: 1,
  };

  unsafe { device_context.CopySubresourceRegion(staging_texture.as_ref().unwrap(), 0, 0, 0, 0, source_texture, 0, Some(&region)) };

  let mut mapped = D3D11_MAPPED_SUBRESOURCE::default();
  unsafe { device_context.Map(staging_texture.as_ref().unwrap(), 0, D3D11_MAP_READ, 0, Some(&mut mapped)).unwrap() };

  let row_pitch = mapped.RowPitch as usize;
  let ptr = mapped.pData as *const u8;

  let mut img = ImageBuffer::<Rgba<u8>, Vec<u8>>::new(desc.Width, desc.Height);

  for y in 0..desc.Height {
    let row_start = unsafe { ptr.add(y as usize * row_pitch) };
    for x in 0..desc.Width {
      let pixel = unsafe { std::slice::from_raw_parts(row_start.add((x * 4) as usize), 4) };
      img.put_pixel(x, y, Rgba([pixel[2], pixel[1], pixel[0], pixel[3]])); // BGRA → RGBA
    }
  }

  unsafe { device_context.Unmap(staging_texture.as_ref().unwrap(), 0) };
  img.save("subregion.png").unwrap();
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

pub const MS: Duration = Duration::from_millis(1);
pub const HZ: u32 = 16;

pub const N: i32 = 0;
pub const F: bool = false;
pub const T: bool = true;

pub trait Functor<T1, T2> = Fn(T1) -> T2 + Send + Sync + 'static;
use {
  crossbeam::channel::{
    Receiver,
    Sender,
    bounded,
  },
  image::{
    GenericImageView,
    ImageBuffer,
    Pixel,
    Rgba,
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
        Gdi::{
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

// let data = std::slice::from_raw_parts(mapped.pData as *const u8, (mapped.RowPitch * desc.Height) as usize);
// if let Some(img) = image::ImageBuffer::<image::Rgb<u8>, Vec<u8>>::from_raw(width, height, data.chunks_exact(4).flat_map(|p| [p[2], p[1], p[0]]).collect()) {
//   img.save("a.png").unwrap();
// }

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
