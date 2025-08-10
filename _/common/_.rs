#![feature(duration_millis_float)]
#![feature(stmt_expr_attributes)]
#![feature(trait_alias)]

pub fn abc<T, E: std::fmt::Debug>(result: Result<T, E>) -> T {
  match result {
    Ok(value) => value,
    Err(error) => panic!("Failed to  unwrap result: {error:#?}"),
  }
}

pub mod time;

pub const F: bool = false;
pub const T: bool = true;
