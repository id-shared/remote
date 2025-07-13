pub fn main() -> windows::core::Result<()> {
  let width = unsafe { GetSystemMetrics(SM_CXSCREEN) };
  let height = unsafe { GetSystemMetrics(SM_CYSCREEN) };
  let pixel_count = (width * height) as usize;
  let buffer_size = pixel_count * size_of::<u32>();

  let screen_dc = Some(unsafe { GetDC(Some(HWND::default())) });
  let mem_dc = unsafe { CreateCompatibleDC(screen_dc) };
  let bitmap = unsafe { CreateCompatibleBitmap(screen_dc.unwrap(), width, height) };
  let old_obj = unsafe { SelectObject(mem_dc, bitmap.into()) };

  let start_time = Instant::now();
  let mut frame_count = 1;
  let mut last_fps_print = Instant::now();

  let buffers: Vec<_> = (0..3).map(|_| Arc::new(FrameBuffer::new(buffer_size))).collect();
  let buffers_producer = buffers.clone();
  let mut write_idx = 0;

  let process_thread = std::thread::spawn(move || {
    let mut read_idx = 0;

    loop {
      if buffers[read_idx].is_ready.load() {
        let buffer = {
          let data = buffers[read_idx].data.lock().unwrap();
          data.clone()
        };
        std::thread::spawn(move || {
          if let Some(img) = ImageBuffer::<Rgb<u8>, Vec<u8>>::from_raw(width as u32, height as u32, buffer.chunks_exact(4).flat_map(|p| [p[2], p[1], p[0]]).collect()) {
            // Convert to JPEG and send through WebSocket
            let mut jpg_buffer = std::io::Cursor::new(Vec::new());
            img.write_to(&mut jpg_buffer, image::ImageFormat::Jpeg).expect("Failed to encode JPEG");
            let jpg_bytes = jpg_buffer.into_inner();
            let filename = format!("{frame_count}.jpg");
            std::fs::write(&filename, &jpg_bytes).expect("Failed to save JPEG image");
            // let filename = format!("vv/_/{frame_count}.png");
            // img.save(&filename).unwrap();
          }
        });
        buffers[read_idx].is_ready.store(false);
        read_idx = (read_idx + 1) % buffers.len();
      }
    }
  });

  loop {
    let success = unsafe { BitBlt(mem_dc, 0, 0, width, height, screen_dc, 0, 0, SRCCOPY) };

    if success.is_err() {
      break;
    }

    unsafe {
      let mut data = buffers_producer[write_idx].data.lock().unwrap();
      GetBitmapBits(bitmap, buffer_size as i32, data.as_mut_ptr().cast());
    }

    buffers_producer[write_idx].is_ready.store(true);
    write_idx = (write_idx + 1) % buffers_producer.len();

    frame_count += 1;

    std::thread::sleep(std::time::Duration::from_millis(250)); // Adjust as needed

    if last_fps_print.elapsed().as_secs() >= 1 {
      let fps = frame_count as f64 / start_time.elapsed().as_secs_f64();
      // println!("FPS: {:.2}", fps);
      last_fps_print = Instant::now();
    }
  }

  let _ = process_thread.join();

  unsafe { ReleaseDC(Some(HWND::default()), screen_dc.unwrap()) };
  unsafe { SelectObject(mem_dc, old_obj) };

  let _ = unsafe { DeleteObject(bitmap.into()) };
  let _ = unsafe { DeleteDC(mem_dc) };

  Ok(())
}

struct FrameBuffer {
  data: Mutex<Vec<u8>>,
  is_ready: AtomicCell<bool>,
}

impl FrameBuffer {
  fn new(size: usize) -> Self {
    Self {
      data: Mutex::new(vec![0u8; size]),
      is_ready: AtomicCell::new(false),
    }
  }
}

use {
  crossbeam_utils::atomic::AtomicCell,
  image::{
    ImageBuffer,
    Rgb,
  },
  std::{
    mem::size_of,
    sync::{
      Arc,
      Mutex,
    },
    time::Instant,
  },
  windows::Win32::{
    Foundation::HWND,
    Graphics::Gdi::{
      BitBlt,
      CreateCompatibleBitmap,
      CreateCompatibleDC,
      DeleteDC,
      DeleteObject,
      GetBitmapBits,
      GetDC,
      ReleaseDC,
      SRCCOPY,
      SelectObject,
    },
    UI::WindowsAndMessaging::{
      GetSystemMetrics,
      SM_CXSCREEN,
      SM_CYSCREEN,
    },
  },
};
