#[inline(always)]
pub fn rmenu(a: bool, z: &Device) -> bool {
  key(a, VK_RMENU, z)
}

#[inline(always)]
pub fn lmenu(a: bool, z: &Device) -> bool {
  key(a, VK_LMENU, z)
}

#[inline(always)]
pub fn ctrl(a: bool, z: &Device) -> bool {
  key(a, VK_CONTROL, z)
}

#[inline(always)]
pub fn ar(a: bool, z: &Device) -> bool {
  key(a, VK_RIGHT, z)
}

#[inline(always)]
pub fn al(a: bool, z: &Device) -> bool {
  key(a, VK_LEFT, z)
}

#[inline(always)]
pub fn ad(a: bool, z: &Device) -> bool {
  key(a, VK_DOWN, z)
}

#[inline(always)]
pub fn au(a: bool, z: &Device) -> bool {
  key(a, VK_UP, z)
}

#[inline(always)]
pub fn l(a: bool, z: &Device) -> bool {
  key(a, VK_L, z)
}

#[inline(always)]
pub fn k(a: bool, z: &Device) -> bool {
  key(a, VK_K, z)
}

#[inline(always)]
pub fn j(a: bool, z: &Device) -> bool {
  key(a, VK_J, z)
}

#[inline(always)]
pub fn i(a: bool, z: &Device) -> bool {
  key(a, VK_I, z)
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
pub fn key(a: bool, k: VIRTUAL_KEY, z: &Device) -> bool {
  match a {
    T => match is(k) {
      T => io(k, 1, z),
      _ => T,
    },
    _ => match is(k) {
      T => T,
      _ => io(k, 0, z),
    },
  }
}

#[inline(always)]
pub fn io(k: VIRTUAL_KEY, n: u16, z: &Device) -> bool {
  d2(
    KEYBOARD_INPUT_DATA {
      UnitId: 0,
      MakeCode: mkcode(k),
      Flags: n,
      Reserved: 0,
      ExtraInformation: 0,
    },
    z,
  )
}

#[inline(always)]
pub fn mkcode(key: VIRTUAL_KEY) -> u16 {
  unsafe { MapVirtualKeyW(key.0 as u32, windows::Win32::UI::Input::KeyboardAndMouse::MAP_VIRTUAL_KEY_TYPE(0)) as u16 }
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
    d2,
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
