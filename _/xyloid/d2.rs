pub fn rmenu(a: bool, z: &Device) -> bool {
  key(a, VK_RMENU, z)
}

pub fn lmenu(a: bool, z: &Device) -> bool {
  key(a, VK_LMENU, z)
}

pub fn ctrl(a: bool, z: &Device) -> bool {
  key(a, VK_CONTROL, z)
}

pub fn ar(a: bool, z: &Device) -> bool {
  key(a, VK_RIGHT, z)
}

pub fn al(a: bool, z: &Device) -> bool {
  key(a, VK_LEFT, z)
}

pub fn ad(a: bool, z: &Device) -> bool {
  key(a, VK_DOWN, z)
}

pub fn au(a: bool, z: &Device) -> bool {
  key(a, VK_UP, z)
}

pub fn l(a: bool, z: &Device) -> bool {
  key(a, VK_L, z)
}

pub fn k(a: bool, z: &Device) -> bool {
  key(a, VK_K, z)
}

pub fn j(a: bool, z: &Device) -> bool {
  key(a, VK_J, z)
}

pub fn i(a: bool, z: &Device) -> bool {
  key(a, VK_I, z)
}

pub fn is_rmenu() -> bool {
  is(VK_RMENU)
}

pub fn is_lmenu() -> bool {
  is(VK_LMENU)
}

pub fn is_ctrl() -> bool {
  is(VK_CONTROL)
}

pub fn is_mr() -> bool {
  is(VK_RBUTTON)
}

pub fn is_ml() -> bool {
  is(VK_LBUTTON)
}

pub fn is_au() -> bool {
  is(VK_UP)
}

pub fn is_ad() -> bool {
  is(VK_DOWN)
}

pub fn is_ar() -> bool {
  is(VK_RIGHT)
}

pub fn is_al() -> bool {
  is(VK_LEFT)
}

pub fn is_l() -> bool {
  is(VK_L)
}

pub fn is_k() -> bool {
  is(VK_K)
}

pub fn is_j() -> bool {
  is(VK_J)
}

pub fn is_i() -> bool {
  is(VK_I)
}

pub fn is_s() -> bool {
  is(VK_S)
}

pub fn is_w() -> bool {
  is(VK_W)
}

pub fn is_a() -> bool {
  is(VK_A)
}

pub fn is_d() -> bool {
  is(VK_D)
}

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

pub fn mkcode(key: VIRTUAL_KEY) -> u16 {
  common::ok(u16::try_from(unsafe { MapVirtualKeyW(u32::from(key.0), windows::Win32::UI::Input::KeyboardAndMouse::MAP_VIRTUAL_KEY_TYPE(0)) }))
}

pub fn is(key: VIRTUAL_KEY) -> bool {
  u32::from(unsafe { GetAsyncKeyState(i32::from(key.0)) }.unsigned_abs()) & 0x8000u32 != 0
}

pub const F: bool = false;
pub const T: bool = true;

use {
  crate::{
    Device,
    d2,
  },
  common,
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
