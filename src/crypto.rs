use std::array;
use std::ops::Rem;
use std::str;
use std::iter;
use aes::cipher::block_padding::Padding;
use base64;

use aes::Aes128;
use aes::cipher::{
    KeyIvInit, block_padding::{Pkcs7, ZeroPadding}, BlockDecryptMut, BlockEncryptMut,
};

type Aes128CbcEnc = cbc::Encryptor<aes::Aes128>;
type Aes128CbcDec = cbc::Decryptor<aes::Aes128>;

const IV: [u8; 16] = [0u8; 16];
const BUFFER_SIZE: usize = 64;

pub fn encrypt(text: String, key: [u8; 16]) -> String {
    let mut buf = [0u8; BUFFER_SIZE];
    let text_len = text.len();

    buf[..text_len].copy_from_slice(text.as_bytes());
    let ct = Aes128CbcEnc::new(&key.into(), &IV.into())
        .encrypt_padded_mut::<ZeroPadding>(&mut buf, text_len)
        .unwrap();

    //println!("{:?}", &ct);
    base64::encode(ct)
}

pub fn decrypt(encrypted: String, key: [u8; 16]) -> String {
    let mut encrypted_array = base64::decode(encrypted).unwrap();
    //println!("{:?}", encrypted_array);
    let text_len = encrypted_array.len();
    let mut buf = [0u8; BUFFER_SIZE];

    buf[..text_len].copy_from_slice(&encrypted_array);
    //Pkcs7::pad(buf.into(), text_len);

    let pt = Aes128CbcDec::new(&key.into(), &IV.into())
        .decrypt_padded_mut::<ZeroPadding>(&mut buf)
        .unwrap();

    unsafe {str::from_utf8_unchecked(&pt[..text_len]).replace("\0", "").to_string()}
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_encrypt() {
        println!("{}", encrypt("yee".to_string(), [0; 16]))
    }

    #[test]
    fn able_to_decrypt_to_original() {
        let original = "yee".to_string();
        let key = [1u8; 16];
        let after = decrypt(encrypt(original.clone(), key), key);
        //println!("{:?}", after)
        assert_eq!(original, after)
    }
}