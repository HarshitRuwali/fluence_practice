use marine_rs_sdk::marine;
use marine_rs_sdk::MountedBinaryResult;

pub fn main() {}

#[marine]
pub fn download(url: String) -> String {
    let result = curl(vec![url]);

    String::from_utf8(result.stdout).unwrap()
}

#[marine]
#[link(wasm_import_module = "host")]
extern "C" {
    fn curl(cmd: Vec<String>) -> MountedBinaryResult;
}
