fn main() {
  let job = create_job_object();

  #[allow(clippy::zombie_processes)]
  let child = Command::new(r"C:\Program Files\Google\Chrome\Application\chrome.exe").args(["--new-window", "https://example.com"]).spawn().expect("Failed to start Chrome");

  // child.wait().expect("failed to wait on child");

  assign_child_to_job(job, &child);

  let mut hwnd = HWND(std::ptr::null_mut());
  let abc = time::now();

  let result = unsafe { FindWindowA(None, s!("Example Domain - Google Chrome")) };

  match result {
    Ok(ok) => {
      hwnd = ok;

      if hwnd.0.is_null() {
        println!("Could not find Chrome window.");
      }
      else {
        let _ = unsafe { ShowWindow(hwnd, SW_HIDE) };
      }
    },
    _ => {
      println!("Error.");
    },
  }

  loop {
    let _ = unsafe { EnumWindows(Some(enum_windows_proc), LPARAM(0)) };

    if time::ms_till(abc) >= 5000 {
      unsafe { ShowWindow(hwnd, SW_SHOW).as_bool() }
    }
    else {
      unsafe { ShowWindow(hwnd, SW_HIDE).as_bool() }
    };

    std::thread::sleep(std::time::Duration::from_secs(1));
  }
}

unsafe extern "system" fn enum_windows_proc(hwnd: HWND, _: LPARAM) -> BOOL {
  // Only get visible windows
  if unsafe { !IsWindowVisible(hwnd) }.as_bool() {
    return true.into();
  }

  // Get window title
  let mut buffer = [0u8; 256];
  let len = unsafe { GetWindowTextA(hwnd, &mut buffer) };
  if len > 0 {
    let len = len.unsigned_abs() as usize;
    if let Ok(title) = CStr::from_bytes_with_nul(&buffer[..=len]) &&
      let Ok(title_str) = title.to_str()
    {
      println!("HWND: {:?}, Title: {}", hwnd.0, title_str);
    }
  }

  true.into()
}

fn create_job_object() -> HANDLE {
  unsafe {
    let job = CreateJobObjectW(None, PCWSTR::null()).expect("Failed to create job object");

    let mut info = JOBOBJECT_EXTENDED_LIMIT_INFORMATION::default();
    info.BasicLimitInformation.LimitFlags = JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE;

    SetInformationJobObject(job, JobObjectExtendedLimitInformation, (&raw const info).cast(), u32::try_from(std::mem::size_of::<JOBOBJECT_EXTENDED_LIMIT_INFORMATION>()).expect("abc")).expect("Failed to set job object info");

    job
  }
}

fn assign_child_to_job(job: HANDLE, child: &Child) {
  unsafe { AssignProcessToJobObject(job, HANDLE(child.as_raw_handle().cast())).expect("Failed to assign child to job") }
}

use {
  common::time,
  std::{
    ffi::CStr,
    os::windows::io::AsRawHandle,
    process::{
      Child,
      Command,
    },
  },
  windows::{
    Win32::{
      Foundation::{
        HANDLE,
        HWND,
        LPARAM,
      },
      System::JobObjects::{
        AssignProcessToJobObject,
        CreateJobObjectW,
        JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE,
        JOBOBJECT_EXTENDED_LIMIT_INFORMATION,
        JobObjectExtendedLimitInformation,
        SetInformationJobObject,
      },
      UI::WindowsAndMessaging::{
        EnumWindows,
        FindWindowA,
        GetWindowTextA,
        IsWindowVisible,
        SW_HIDE,
        SW_SHOW,
        ShowWindow,
      },
    },
    core::{
      BOOL,
      PCWSTR,
      s,
    },
  },
};
