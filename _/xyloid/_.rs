pub fn device(guid: u128) -> Option<String> {
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

#[inline(always)]
pub fn z02(handle: HANDLE, up: bool) -> bool {
  zz(
    handle,
    if !up {
      MOUSE_BUTTON_2_DOWN
    }
    else {
      MOUSE_BUTTON_2_UP
    },
  )
}

#[inline(always)]
pub fn z01(handle: HANDLE, up: bool) -> bool {
  zz(
    handle,
    if !up {
      MOUSE_BUTTON_1_DOWN
    }
    else {
      MOUSE_BUTTON_1_UP
    },
  )
}

#[inline(always)]
pub fn zz(handle: HANDLE, n: u32) -> bool {
  let mut mi = MOUSE_INPUT_DATA {
    UnitId: 0,
    Flags: 0,
    Anonymous: Default::default(),
    RawButtons: 0,
    LastX: 0,
    LastY: 0,
    ExtraInformation: 0,
  };

  mi.Anonymous.Buttons = n;

  io1(handle, mi)
}

#[inline(always)]
pub fn xy(handle: HANDLE, x: i32, y: i32) -> bool {
  io1(handle, MOUSE_INPUT_DATA {
    UnitId: 0,
    Flags: 0,
    Anonymous: Default::default(),
    RawButtons: 0,
    LastX: x,
    LastY: -1 * y,
    ExtraInformation: 0,
  })
}

#[inline(always)]
pub fn key_arrow_r(handle: HANDLE, up: bool) -> bool {
  key_(handle, VK_RIGHT, up)
}

#[inline(always)]
pub fn key_arrow_l(handle: HANDLE, up: bool) -> bool {
  key_(handle, VK_LEFT, up)
}

#[inline(always)]
pub fn key_arrow_d(handle: HANDLE, up: bool) -> bool {
  key_(handle, VK_DOWN, up)
}

#[inline(always)]
pub fn key_arrow_u(handle: HANDLE, up: bool) -> bool {
  key_(handle, VK_UP, up)
}

#[inline(always)]
pub fn key_ctrl(handle: HANDLE, up: bool) -> bool {
  key_(handle, VK_CONTROL, up)
}

#[inline(always)]
pub fn key_h(handle: HANDLE, up: bool) -> bool {
  key_(handle, VK_H, up)
}

#[inline(always)]
pub fn key_(handle: HANDLE, key: VIRTUAL_KEY, up: bool) -> bool {
  k_(
    handle,
    key,
    if up {
      1
    }
    else {
      0
    },
  )
}

#[inline(always)]
pub fn k_(handle: HANDLE, key: VIRTUAL_KEY, flag: u16) -> bool {
  io2(handle, KEYBOARD_INPUT_DATA {
    UnitId: 0,
    MakeCode: mkcode(key),
    Flags: flag,
    Reserved: 0,
    ExtraInformation: 0,
  })
}

#[inline(always)]
pub fn mkcode(key: VIRTUAL_KEY) -> u16 {
  unsafe { MapVirtualKeyW(key.0 as u32, windows::Win32::UI::Input::KeyboardAndMouse::MAP_VIRTUAL_KEY_TYPE(0)) as u16 }
}

#[inline(always)]
pub fn io2(handle: HANDLE, ki: KEYBOARD_INPUT_DATA) -> bool {
  io(handle, Xyloid {
    unk1: 0,
    type_: XyloidType::Keyboard,
    input: XyloidInput {
      ki: ki,
    },
  })
}

#[inline(always)]
pub fn io1(handle: HANDLE, mi: MOUSE_INPUT_DATA) -> bool {
  io(handle, Xyloid {
    unk1: 0,
    type_: XyloidType::Mouse,
    input: XyloidInput {
      mi: mi,
    },
  })
}

#[inline(always)]
pub fn io(handle: HANDLE, mut xyloid: Xyloid) -> bool {
  let mut bytes_returned: u32 = 0;

  unsafe {
    DeviceIoControl(
      handle,
      0x88883020,
      Some(&mut xyloid as *mut _ as *mut _), // lpInBuffer
      std::mem::size_of::<Xyloid>() as u32,  // nInBufferSize
      None,                                  // lpOutBuffer
      0,                                     // nOutBufferSize
      Some(&mut bytes_returned),             // lpBytesReturned
      None,                                  // lpOverlapped
    )
  }
  .is_ok()
}

pub fn connect(path: String) -> Option<HANDLE> {
  // HINT: r"\\?\rzcontrol#vid_1532&pid_007b&mi_00#8&14018023&0#{e3be005d-d130-4910-88ff-09ae02f680e9}";
  let wide: Vec<u16> = OsStr::new(path.as_str()).encode_wide().chain(Some(0)).collect();

  let contact = unsafe { CreateFileW(PCWSTR(wide.as_ptr()), GENERIC_WRITE.0, FILE_SHARE_READ | FILE_SHARE_WRITE, None, OPEN_EXISTING, FILE_FLAGS_AND_ATTRIBUTES(0), None) };

  match contact {
    Ok(handle) => Some(handle),
    _ => None,
  }
}

pub fn type_1() -> Option<HANDLE> {
  match device(0xe3be005d_d130_4910_88ff_09ae02f680e9) {
    Some(path) => connect(path),
    _ => None,
  }
}

#[inline(always)]
pub fn is_mouse_r() -> bool {
  is_held(VK_RBUTTON)
}

#[inline(always)]
pub fn is_mouse_l() -> bool {
  is_held(VK_LBUTTON)
}

#[inline(always)]
pub fn is_ctrl() -> bool {
  is_held(VK_CONTROL)
}

#[inline(always)]
pub fn is_h() -> bool {
  is_held(VK_H)
}

#[inline(always)]
pub fn is_d() -> bool {
  is_held(VK_D)
}

#[inline(always)]
pub fn is_s() -> bool {
  is_held(VK_S)
}

#[inline(always)]
pub fn is_a() -> bool {
  is_held(VK_A)
}

#[inline(always)]
pub fn is_w() -> bool {
  is_held(VK_W)
}

#[inline(always)]
pub fn is_held(key: VIRTUAL_KEY) -> bool {
  unsafe { (GetAsyncKeyState(key.0 as i32) as u32 & 0x8000u32) != 0 }
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
          MOUSE_BUTTON_1_DOWN,
          MOUSE_BUTTON_1_UP,
          MOUSE_BUTTON_2_DOWN,
          MOUSE_BUTTON_2_UP,
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
      UI::Input::KeyboardAndMouse::{
        GetAsyncKeyState,
        MapVirtualKeyW,
        VIRTUAL_KEY,
        VK_A,
        VK_CONTROL,
        VK_D,
        VK_DOWN,
        VK_H,
        VK_LBUTTON,
        VK_LEFT,
        VK_RBUTTON,
        VK_RIGHT,
        VK_S,
        VK_UP,
        VK_W,
      },
    },
    core::PCWSTR,
  },
};
