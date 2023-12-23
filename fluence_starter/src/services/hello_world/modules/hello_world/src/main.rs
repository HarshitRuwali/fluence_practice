#![allow(non_snake_case)]
use marine_rs_sdk::marine;
use marine_rs_sdk::module_manifest;

module_manifest!();

#[marine]
pub struct Hello {
  pub response: String
}

pub fn main() {}

#[marine]
pub fn hello_world() -> Hello {
    let response = format!("Hello, Fluence!");
    Hello { response }
}