#[inline(always)]
pub fn rmenu(device: &Device, up: bool) -> bool {
  key_(device, VK_RMENU, up)
}

#[inline(always)]
pub fn lmenu(device: &Device, up: bool) -> bool {
  key_(device, VK_LMENU, up)
}

#[inline(always)]
pub fn ctrl(device: &Device, up: bool) -> bool {
  key_(device, VK_CONTROL, up)
}

#[inline(always)]
pub fn ar(device: &Device, up: bool) -> bool {
  key_(device, VK_RIGHT, up)
}

#[inline(always)]
pub fn al(device: &Device, up: bool) -> bool {
  key_(device, VK_LEFT, up)
}

#[inline(always)]
pub fn ad(device: &Device, up: bool) -> bool {
  key_(device, VK_DOWN, up)
}

#[inline(always)]
pub fn au(device: &Device, up: bool) -> bool {
  key_(device, VK_UP, up)
}

#[inline(always)]
pub fn l(device: &Device, up: bool) -> bool {
  key_(device, VK_L, up)
}

#[inline(always)]
pub fn k(device: &Device, up: bool) -> bool {
  key_(device, VK_K, up)
}

#[inline(always)]
pub fn j(device: &Device, up: bool) -> bool {
  key_(device, VK_J, up)
}

#[inline(always)]
pub fn i(device: &Device, up: bool) -> bool {
  key_(device, VK_I, up)
}

#[inline(always)]
pub fn is_rmenu() -> bool {
  is(VK_RMENU)
}

#[inline(always)]
pub fn is_lmenu() -> bool {
  is(VK_LMENU)
}

#[inline(always)]
pub fn is_ctrl() -> bool {
  is(VK_CONTROL)
}

#[inline(always)]
pub fn is_mr() -> bool {
  is(VK_RBUTTON)
}

#[inline(always)]
pub fn is_ml() -> bool {
  is(VK_LBUTTON)
}

#[inline(always)]
pub fn is_l() -> bool {
  is(VK_L)
}

#[inline(always)]
pub fn is_k() -> bool {
  is(VK_K)
}

#[inline(always)]
pub fn is_j() -> bool {
  is(VK_J)
}

#[inline(always)]
pub fn is_i() -> bool {
  is(VK_I)
}

#[inline(always)]
pub fn is_d() -> bool {
  is(VK_D)
}

#[inline(always)]
pub fn is_s() -> bool {
  is(VK_S)
}

#[inline(always)]
pub fn is_a() -> bool {
  is(VK_A)
}

#[inline(always)]
pub fn is_w() -> bool {
  is(VK_W)
}

#[inline(always)]
pub fn key_(device: &Device, key: VIRTUAL_KEY, up: bool) -> bool {
  match up {
    T => match is(key) {
      T => k_(device, key, 1),
      _ => F,
    },
    _ => match is(key) {
      T => T,
      _ => k_(device, key, 0),
    },
  }
}

#[inline(always)]
pub fn k_(device: &Device, key: VIRTUAL_KEY, flag: u16) -> bool {
  io2(device, KEYBOARD_INPUT_DATA {
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
pub fn io2(device: &Device, ki: KEYBOARD_INPUT_DATA) -> bool {
  io(device, Xyloid {
    unk1: 0,
    type_: XyloidType::Keyboard,
    input: XyloidInput {
      ki: ki,
    },
  })
}

#[inline(always)]
pub fn is(key: VIRTUAL_KEY) -> bool {
  unsafe { (GetAsyncKeyState(key.0 as i32) as u32 & 0x8000u32) != 0 }
}

pub const F: bool = false;
pub const T: bool = true;

use {
  crate::{
    Device,
    Xyloid,
    XyloidInput,
    XyloidType,
    io,
  },
  windows::Win32::{
    Devices::HumanInterfaceDevice::KEYBOARD_INPUT_DATA,
    UI::Input::KeyboardAndMouse::{
      GetAsyncKeyState,
      MapVirtualKeyW,
      VIRTUAL_KEY,
      VK_A,
      VK_CONTROL,
      VK_D,
      VK_DOWN,
      VK_I,
      VK_J,
      VK_K,
      VK_L,
      VK_LBUTTON,
      VK_LEFT,
      VK_LMENU,
      VK_RBUTTON,
      VK_RIGHT,
      VK_RMENU,
      VK_S,
      VK_UP,
      VK_W,
    },
  },
};
