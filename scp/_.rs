fn main() {
  let _ = Command::new(r"C:\Program Files\Google\Chrome\Application\chrome.exe").args(["--remote-debugging-port=9222", "--new-window", "https://example.com"]).spawn().expect("Failed to start Chrome");

  // Wait a bit and then hide the window (not reliable across all systems)
  std::thread::sleep(std::time::Duration::from_secs(2));

  let mut hwnd = HWND(std::ptr::null_mut());
  let abc = time::now();

  unsafe {
    let result = FindWindowA(None, s!("Example Domain - Google Chrome"));

    match result {
      Ok(ok) => {
        hwnd = ok;

        if hwnd.0 != std::ptr::null_mut() {
          let _ = ShowWindow(hwnd, SW_HIDE);
        }
        else {
          println!("Could not find Chrome window.");
        }
      },
      _ => {
        println!("Error.");
      },
    }
  }

  loop {
    let _ = unsafe { EnumWindows(Some(enum_windows_proc), LPARAM(0)) };

    match time::till(abc) >= 5000. {
      true => unsafe { ShowWindow(hwnd, SW_SHOW).as_bool() },
      _ => false,
    };

    std::thread::sleep(std::time::Duration::from_secs(1));
  }
}

unsafe extern "system" fn enum_windows_proc(hwnd: HWND, _: LPARAM) -> BOOL {
  // Only get visible windows
  if !IsWindowVisible(hwnd).as_bool() {
    return true.into();
  }

  // Get window title
  let mut buffer = [0u8; 256];
  let len = GetWindowTextA(hwnd, &mut buffer);
  if len > 0 {
    if let Ok(title) = CStr::from_bytes_with_nul(&buffer[..=len as usize]) {
      if let Ok(title_str) = title.to_str() {
        println!("HWND: {:?}, Title: {}", hwnd.0, title_str);
      }
    }
  }

  true.into()
}

use {
  common::time,
  std::{
    ffi::CStr,
    process::Command,
  },
  windows::{
    Win32::{
      Foundation::{
        HWND,
        LPARAM,
      },
      UI::WindowsAndMessaging::{
        EnumWindows,
        FindWindowA,
        GetWindowTextA,
        IsWindowVisible,
        SW_HIDE,
        SW_MINIMIZE,
        SW_SHOW,
        ShowWindow,
      },
    },
    core::{
      BOOL,
      s,
    },
  },
};
