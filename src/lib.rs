#![deny(clippy::all)]

use napi::Error;
use napi_derive::napi;

/// Extract links from a markdown string
#[napi]
pub fn extract_links(markdown: String) -> Result<Vec<String>, Error> {
  Ok(vec![])
}
