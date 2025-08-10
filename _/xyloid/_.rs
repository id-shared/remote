pub fn device() -> Device {
  fn path(guid: u128) -> Option<String> {
    let device_guid = windows::core::GUID::from_u128(guid);

    unsafe { SetupDiGetClassDevsW(Some(&raw const device_guid), PCWSTR::null(), None, DIGCF_PRESENT | DIGCF_DEVICEINTERFACE) }.map_or(None, |handle| {
      let mut device_interface_data = SP_DEVICE_INTERFACE_DATA::default();
      let idx = 0;

      device_interface_data.cbSize = u32::try_from(std::mem::size_of::<SP_DEVICE_INTERFACE_DATA>()).unwrap();

      if unsafe { SetupDiEnumDeviceInterfaces(handle, None, &raw const device_guid, idx, &raw mut device_interface_data) }.is_ok() {
        let mut required_size = 0u32;
        let _ = unsafe { SetupDiGetDeviceInterfaceDetailW(handle, &raw const device_interface_data, None, 0, Some(&raw mut required_size), None) };

        // Calculate how many SP_DEVICE_INTERFACE_DETAIL_DATA_W structures we need
        let struct_size = std::mem::size_of::<SP_DEVICE_INTERFACE_DETAIL_DATA_W>();
        let num_structs = (required_size as usize).div_ceil(struct_size);

        // Allocate properly aligned memory using Vec
        let mut detail_buffer: Vec<SP_DEVICE_INTERFACE_DETAIL_DATA_W> = vec![unsafe { std::mem::zeroed() }; num_structs];
        let detail_ptr = detail_buffer.as_mut_ptr();
        unsafe { (*detail_ptr).cbSize = u32::try_from(std::mem::size_of::<SP_DEVICE_INTERFACE_DETAIL_DATA_W>()).unwrap() };

        if unsafe { SetupDiGetDeviceInterfaceDetailW(handle, &raw const device_interface_data, Some(detail_ptr), required_size, None, None) }.is_ok() {
          let device_path_ptr = unsafe { &(*detail_ptr).DevicePath as *const u16 };
          let mut len = 0;

          while unsafe { *device_path_ptr.add(len) } != 0 {
            len += 1;
          }

          let device_path_slice = unsafe { std::slice::from_raw_parts(device_path_ptr, len) };
          OsString::from_wide(device_path_slice).to_str().map_or_else(
            || {
              let _ = unsafe { SetupDiDestroyDeviceInfoList(handle) };
              None
            },
            |path_str| {
              let _ = unsafe { SetupDiDestroyDeviceInfoList(handle) };
              Some(path_str.to_string())
            },
          )
        }
        else {
          let _ = unsafe { SetupDiDestroyDeviceInfoList(handle) };
          None
        }
      }
      else {
        let _ = unsafe { SetupDiDestroyDeviceInfoList(handle) };
        None
      }
    })
  }

  fn io(path: &str) -> Option<HANDLE> {
    // HINT: r"\\?\rzcontrol#vid_1532&pid_007b&mi_00#8&14018023&0#{e3be005d-d130-4910-88ff-09ae02f680e9}";
    let wide: Vec<u16> = OsStr::new(path).encode_wide().chain(Some(0)).collect();

    let contact = unsafe { CreateFileW(PCWSTR(wide.as_ptr()), GENERIC_WRITE.0, FILE_SHARE_READ | FILE_SHARE_WRITE, None, OPEN_EXISTING, FILE_FLAGS_AND_ATTRIBUTES(0), None) };

    contact.ok()
  }

  let device = path(0xe3be005d_d130_4910_88ff_09ae02f680e9).unwrap();
  let handle = io(&device).unwrap();
  Device {
    handle,
  }
}

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

pub fn io(mut x: Xyloid, z: &Device) -> bool {
  let mut bytes_returned: u32 = 0;

  unsafe {
    DeviceIoControl(
      z.handle,
      0x8888_3020,
      Some((&raw mut x).cast()),                             // lpInBuffer
      u32::try_from(std::mem::size_of::<Xyloid>()).unwrap(), // nInBufferSize
      None,                                                  // lpOutBuffer
      0,                                                     // nOutBufferSize
      Some(&raw mut bytes_returned),                         // lpBytesReturned
      None,                                                  // lpOverlapped
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
