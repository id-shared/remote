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

use {
  crate::{
    Xyloid,
    XyloidInput,
    XyloidType,
    io,
  },
  windows::Win32::{
    Devices::HumanInterfaceDevice::KEYBOARD_INPUT_DATA,
    Foundation::HANDLE,
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
};
