use crate::crypto;
use crate::json;
use crate::app;
use std::fs;

pub fn pre_process(key: String) -> Option<Vec<(String, String)>> {
    let len = key.len();
    let mut buf = [0u8; 16];
    buf[..len].copy_from_slice(key.as_bytes());

    let contents = fs::read_to_string("data/something_else.json").expect("Error Reading File!");
    let mut decoded_data = json::decode(contents);
    for (site, passwd) in &mut decoded_data {
        *site = crypto::decrypt(site.to_string(), buf);
        *passwd = crypto::decrypt(passwd.to_string(), buf);
    }
    Some(decoded_data)
}

pub fn post_process(json: String) {

}