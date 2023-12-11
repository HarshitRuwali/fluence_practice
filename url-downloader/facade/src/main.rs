use marine_rs_sdk::marine;

pub fn main() {}

#[marine]
pub fn get_n_save(url: String, file_name: String) -> String{
    
    let result = download(url.clone());
    file_put(file_name, result.into_bytes());

    String::from("OK")
}

#[marine]
#[link(wasm_import_module = "curl_adapter")]
extern "C"{
    pub fn download(url: String) -> String;
}

#[marine]
#[link(wasm_import_module = "local_storage")]
extern "C"{
    // this link_name attibute allows using "file_get" name
    // for function imoprted by "get" name

    #[link_name = "get"]
    pub fn file_get(file_name: String) -> Vec<u8>;
    
    #[link_name = "put"]
    pub fn file_put(name: String, file_content: Vec<u8>) -> String;
}

