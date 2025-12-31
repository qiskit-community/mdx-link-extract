#![deny(clippy::all)]

use napi_derive::napi;

/// This is just a placeholder to check the tests are working. We'll replace
/// this with code soon.
#[napi]
pub fn plus_100(input: u32) -> u32 {
  input + 100
}
