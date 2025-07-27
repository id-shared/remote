pub fn device() -> Device {
  fn path(guid: u128) -> Option<String> {
    let device_guid = windows::core::GUID::from_u128(guid);

    match unsafe { SetupDiGetClassDevsW(Some(&device_guid), PCWSTR::null(), None, DIGCF_PRESENT | DIGCF_DEVICEINTERFACE) } {
      Ok(handle) => {
        let mut device_interface_data = SP_DEVICE_INTERFACE_DATA::default();
        let idx = 0;

        device_interface_data.cbSize = std::mem::size_of::<SP_DEVICE_INTERFACE_DATA>() as u32;

        match unsafe { SetupDiEnumDeviceInterfaces(handle, None, &device_guid, idx, &mut device_interface_data) } {
          Ok(_) => {
            let mut required_size = 0u32;
            let _ = unsafe { SetupDiGetDeviceInterfaceDetailW(handle, &device_interface_data, None, 0, Some(&mut required_size as *mut u32), None) };

            let mut detail_data = vec![0u8; required_size as usize];
            let detail_ptr = detail_data.as_mut_ptr() as *mut SP_DEVICE_INTERFACE_DETAIL_DATA_W;
            unsafe { (*detail_ptr).cbSize = std::mem::size_of::<SP_DEVICE_INTERFACE_DETAIL_DATA_W>() as u32 };

            match unsafe { SetupDiGetDeviceInterfaceDetailW(handle, &device_interface_data, Some(detail_ptr), required_size, None, None) } {
              Ok(_) => {
                let device_path_ptr = unsafe { &(*detail_ptr).DevicePath as *const u16 };
                let mut len = 0;

                while unsafe { *device_path_ptr.add(len) } != 0 {
                  len += 1;
                }

                let device_path_slice = unsafe { std::slice::from_raw_parts(device_path_ptr, len) };
                match OsString::from_wide(device_path_slice).to_str() {
                  Some(path_str) => {
                    let _ = unsafe { SetupDiDestroyDeviceInfoList(handle) };
                    return Some(path_str.to_string());
                  },
                  _ => {
                    let _ = unsafe { SetupDiDestroyDeviceInfoList(handle) };
                    return None;
                  },
                }
              },
              _ => {
                let _ = unsafe { SetupDiDestroyDeviceInfoList(handle) };
                return None;
              },
            }
          },
          _ => {
            let _ = unsafe { SetupDiDestroyDeviceInfoList(handle) };
            return None;
          },
        }
      },
      _ => None,
    }
  }

  fn io(path: String) -> Option<HANDLE> {
    // HINT: r"\\?\rzcontrol#vid_1532&pid_007b&mi_00#8&14018023&0#{e3be005d-d130-4910-88ff-09ae02f680e9}";
    let wide: Vec<u16> = OsStr::new(path.as_str()).encode_wide().chain(Some(0)).collect();

    let contact = unsafe { CreateFileW(PCWSTR(wide.as_ptr()), GENERIC_WRITE.0, FILE_SHARE_READ | FILE_SHARE_WRITE, None, OPEN_EXISTING, FILE_FLAGS_AND_ATTRIBUTES(0), None) };

    match contact {
      Ok(handle) => Some(handle),
      _ => None,
    }
  }

  let device = path(0xe3be005d_d130_4910_88ff_09ae02f680e9).unwrap();
  let handle = io(device).unwrap();
  Device {
    handle,
  }
}

#[inline(always)]
pub fn d2(x: KEYBOARD_INPUT_DATA, z: &Device) -> bool {
  io(
    Xyloid {
      unk1: 0,
      type_: XyloidType::Keyboard,
      input: XyloidInput {
        ki: x,
      },
    },
    z,
  )
}

#[inline(always)]
pub fn d1(x: MOUSE_INPUT_DATA, z: &Device) -> bool {
  io(
    Xyloid {
      unk1: 0,
      type_: XyloidType::Mouse,
      input: XyloidInput {
        mi: x,
      },
    },
    z,
  )
}

#[inline(always)]
pub fn io(mut x: Xyloid, z: &Device) -> bool {
  let mut bytes_returned: u32 = 0;

  unsafe {
    DeviceIoControl(
      z.handle,
      0x88883020,
      Some(&mut x as *mut _ as *mut _),     // lpInBuffer
      std::mem::size_of::<Xyloid>() as u32, // nInBufferSize
      None,                                 // lpOutBuffer
      0,                                    // nOutBufferSize
      Some(&mut bytes_returned),            // lpBytesReturned
      None,                                 // lpOverlapped
    )
  }
  .is_ok()
}

pub struct Device {
  handle: HANDLE,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum XyloidType {
  Keyboard = 1,
  Mouse    = 2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union XyloidInput {
  pub ki: KEYBOARD_INPUT_DATA,
  pub mi: MOUSE_INPUT_DATA,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Xyloid {
  pub unk1: u32,
  pub type_: XyloidType,
  pub input: XyloidInput,
}

// Compile-time size check
const _: () = assert!(std::mem::size_of::<Xyloid>() == 32);

pub mod d1;
pub mod d2;

pub use windows::Win32::Foundation::HANDLE;
use {
  std::{
    ffi::{
      OsStr,
      OsString,
    },
    os::windows::ffi::{
      OsStrExt,
      OsStringExt,
    },
  },
  windows::{
    Win32::{
      Devices::{
        DeviceAndDriverInstallation::{
          DIGCF_DEVICEINTERFACE,
          DIGCF_PRESENT,
          SP_DEVICE_INTERFACE_DATA,
          SP_DEVICE_INTERFACE_DETAIL_DATA_W,
          SetupDiDestroyDeviceInfoList,
          SetupDiEnumDeviceInterfaces,
          SetupDiGetClassDevsW,
          SetupDiGetDeviceInterfaceDetailW,
        },
        HumanInterfaceDevice::{
          KEYBOARD_INPUT_DATA,
          MOUSE_INPUT_DATA,
        },
      },
      Foundation::GENERIC_WRITE,
      Storage::FileSystem::{
        CreateFileW,
        FILE_FLAGS_AND_ATTRIBUTES,
        FILE_SHARE_READ,
        FILE_SHARE_WRITE,
        OPEN_EXISTING,
      },
      System::IO::DeviceIoControl,
    },
    core::PCWSTR,
  },
};
