use std::ffi::{c_int, c_uchar};
use std::ptr::{null, null_mut};
use openssl_sys::{EVP_CIPHER_CTX_new, EVP_EncryptFinal_ex, EVP_EncryptInit_ex, EVP_EncryptUpdate, EVP_aes_256_cbc, AES_BLOCK_SIZE, ENGINE, EVP_CIPHER_CTX};
use crate::utils::Result;

pub struct Aes {
    key : [u8; Aes::KEY_SIZE],
    iv : [u8; AES_BLOCK_SIZE as usize],
    ctx : *mut EVP_CIPHER_CTX
}


impl Aes {
    const KEY_SIZE : usize = 32;
    const PADDING : u8 = '$' as u8;

    pub fn new(key : [u8; Aes::KEY_SIZE], iv: [u8; AES_BLOCK_SIZE as usize]) -> Self { 
        unsafe {
            Self {
                key,
                iv,
                ctx: EVP_CIPHER_CTX_new()
            } 
        }
    }

    pub fn encrypt(&mut self, plain : &String) -> Result<String> {

        let cipher_text_len = (((plain.len() as c_int + AES_BLOCK_SIZE) / AES_BLOCK_SIZE) * AES_BLOCK_SIZE) as usize;
        let mut cipher_text =  vec![0u8; cipher_text_len].into_boxed_slice();

        let mut len = 0 as c_int;
        let mut cipher_text_len = 0;

        unsafe {
            if EVP_EncryptInit_ex(self.ctx, EVP_aes_256_cbc(), null_mut(), self.key.as_ptr(), self.iv.as_ptr()) != 1 {
                return Err("1")
            }

            if EVP_EncryptUpdate(self.ctx, cipher_text.as_mut_ptr(), &mut len, plain.as_ptr(), plain.len() as c_int ) != 1 {
                return Err("1")
            }
            cipher_text_len = len;

            if EVP_EncryptFinal_ex(self.ctx, cipher_text.as_mut_ptr() + len, &len) != 1
            {
                throw runtime_error(get_open_ssl_error());
            }
            cipher_text_len += len;
            
        }
        

        
        Err("")
    }
}