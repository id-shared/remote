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
pub fn xy(handle: HANDLE, x: f64, y: f64) -> bool {
  io1(handle, MOUSE_INPUT_DATA {
    UnitId: 0,
    Flags: 0,
    Anonymous: Default::default(),
    RawButtons: 0,
    LastX: (x.round()) as i32,
    LastY: (-1. * y.round()) as i32,
    ExtraInformation: 0,
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

use {
  crate::{
    Xyloid,
    XyloidInput,
    XyloidType,
    io,
  },
  windows::Win32::{
    Devices::HumanInterfaceDevice::{
      MOUSE_BUTTON_1_DOWN,
      MOUSE_BUTTON_1_UP,
      MOUSE_BUTTON_2_DOWN,
      MOUSE_BUTTON_2_UP,
      MOUSE_INPUT_DATA,
    },
    Foundation::HANDLE,
  },
};
