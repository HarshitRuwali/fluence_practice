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

#[cfg(test)]
mod tests {
    use marine_rs_sdk_test::marine_test;
    #[marine_test(config_path = "../../../../../../.fluence/tmp/Config.toml")]
    fn test_hello_world(hw: marine_test_env::hello_world::ModuleInterface) {
        let greeting = hw.hello_world();
        assert_eq!(greeting.response, "Hello, Fluence!".to_string());
    }
}

