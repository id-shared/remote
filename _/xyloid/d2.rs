#[must_use]
pub fn rmenu(a: bool, z: &Device) -> bool {
  key(a, VK_RMENU, z)
}

#[must_use]
pub fn lmenu(a: bool, z: &Device) -> bool {
  key(a, VK_LMENU, z)
}

#[must_use]
pub fn ctrl(a: bool, z: &Device) -> bool {
  key(a, VK_CONTROL, z)
}

#[must_use]
pub fn ar(a: bool, z: &Device) -> bool {
  key(a, VK_RIGHT, z)
}

#[must_use]
pub fn al(a: bool, z: &Device) -> bool {
  key(a, VK_LEFT, z)
}

#[must_use]
pub fn ad(a: bool, z: &Device) -> bool {
  key(a, VK_DOWN, z)
}

#[must_use]
pub fn au(a: bool, z: &Device) -> bool {
  key(a, VK_UP, z)
}

#[must_use]
pub fn l(a: bool, z: &Device) -> bool {
  key(a, VK_L, z)
}

#[must_use]
pub fn k(a: bool, z: &Device) -> bool {
  key(a, VK_K, z)
}

#[must_use]
pub fn j(a: bool, z: &Device) -> bool {
  key(a, VK_J, z)
}

#[must_use]
pub fn i(a: bool, z: &Device) -> bool {
  key(a, VK_I, z)
}

#[must_use]
pub fn is_rmenu() -> bool {
  is(VK_RMENU)
}

#[must_use]
pub fn is_lmenu() -> bool {
  is(VK_LMENU)
}

#[must_use]
pub fn is_ctrl() -> bool {
  is(VK_CONTROL)
}

#[must_use]
pub fn is_mr() -> bool {
  is(VK_RBUTTON)
}

#[must_use]
pub fn is_ml() -> bool {
  is(VK_LBUTTON)
}

#[must_use]
pub fn is_au() -> bool {
  is(VK_UP)
}

#[must_use]
pub fn is_ad() -> bool {
  is(VK_DOWN)
}

#[must_use]
pub fn is_ar() -> bool {
  is(VK_RIGHT)
}

#[must_use]
pub fn is_al() -> bool {
  is(VK_LEFT)
}

#[must_use]
pub fn is_l() -> bool {
  is(VK_L)
}

#[must_use]
pub fn is_k() -> bool {
  is(VK_K)
}

#[must_use]
pub fn is_j() -> bool {
  is(VK_J)
}

#[must_use]
pub fn is_i() -> bool {
  is(VK_I)
}

#[must_use]
pub fn is_s() -> bool {
  is(VK_S)
}

#[must_use]
pub fn is_w() -> bool {
  is(VK_W)
}

#[must_use]
pub fn is_a() -> bool {
  is(VK_A)
}

#[must_use]
pub fn is_d() -> bool {
  is(VK_D)
}

#[must_use]
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

#[must_use]
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

#[must_use]
pub fn mkcode(key: VIRTUAL_KEY) -> u16 {
  unsafe { MapVirtualKeyW(key.0 as u32, windows::Win32::UI::Input::KeyboardAndMouse::MAP_VIRTUAL_KEY_TYPE(0)) as u16 }
}

#[must_use]
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
