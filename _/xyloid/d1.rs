pub fn z02(a: bool, z: &Device) -> bool {
  io(
    match a {
      T => MOUSE_BUTTON_2_UP,
      _ => MOUSE_BUTTON_2_DOWN,
    },
    z,
  )
}

pub fn z01(a: bool, z: &Device) -> bool {
  io(
    match a {
      T => MOUSE_BUTTON_1_UP,
      _ => MOUSE_BUTTON_1_DOWN,
    },
    z,
  )
}

pub fn xy(x: i64, y: i64, z: &Device) -> bool {
  d1(
    MOUSE_INPUT_DATA {
      UnitId: 0,
      Flags: 0,
      Anonymous: MOUSE_INPUT_DATA_0::default(),
      RawButtons: 0,
      LastX: common::it(i32::try_from(x)),
      LastY: -common::it(i32::try_from(y)),
      ExtraInformation: 0,
    },
    z,
  )
}

pub fn io(n: u32, z: &Device) -> bool {
  let mut mi = MOUSE_INPUT_DATA {
    UnitId: 0,
    Flags: 0,
    Anonymous: MOUSE_INPUT_DATA_0::default(),
    RawButtons: 0,
    LastX: 0,
    LastY: 0,
    ExtraInformation: 0,
  };

  mi.Anonymous.Buttons = n;

  d1(mi, z)
}

use {
  crate::{
    Device,
    d1,
  },
  common::T,
  windows::Win32::Devices::HumanInterfaceDevice::{
    MOUSE_BUTTON_1_DOWN,
    MOUSE_BUTTON_1_UP,
    MOUSE_BUTTON_2_DOWN,
    MOUSE_BUTTON_2_UP,
    MOUSE_INPUT_DATA,
    MOUSE_INPUT_DATA_0,
  },
};
