#![feature(duration_millis_float)]
#![feature(stmt_expr_attributes)]
#![feature(trait_alias)]

#[inline]
pub fn ok<T, E: std::fmt::Debug>(z: Result<T, E>) -> T {
  z.unwrap_or_else(|err| panic!("called `ok()` on an `Err` value: {err:?}."))
}

#[inline]
pub fn so<T>(z: Option<T>) -> T {
  z.unwrap_or_else(|| panic!("called `so()` on a `None` value."))
}

pub mod time;

pub const F: bool = false;
pub const T: bool = true;
