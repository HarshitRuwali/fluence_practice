use marine_rs_sdk::marine;
use std::fs;
use std::path::PathBuf;

const SITES_DIR: &str = "/sites/";

pub fn main(){}

#[marine]
pub fn put(name: String, file_content: Vec<u8>) -> String{
    let rpc_tmp_filepath = format!("{} {}", SITES_DIR, name);
    let _ = fs::write(PathBuf::from(rpc_tmp_filepath), file_content);

    String::from("OK")
}

#[marine]
pub fn get(file_name: String) -> Vec<u8>{
    let tmp_filepath = format!("{} {}", SITES_DIR, file_name);
    fs::read(tmp_filepath).unwrap_or_else(|_| b"error while reading file".to_vec())
}
