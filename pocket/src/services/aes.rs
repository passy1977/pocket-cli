use crate::utils::Result;
use openssl_sys::{EVP_CIPHER_CTX_new, EVP_DecryptFinal_ex, EVP_DecryptInit_ex, EVP_DecryptUpdate, EVP_EncryptFinal_ex, EVP_EncryptInit_ex, EVP_EncryptUpdate, EVP_aes_256_cbc, AES_BLOCK_SIZE, EVP_CIPHER_CTX};
use std::ffi::c_int;
use std::fmt::Write;
use std::ptr::null_mut;

pub struct Aes {
    key : [u8; Aes::KEY_SIZE],
    iv : [u8; AES_BLOCK_SIZE as usize],
    ctx : *mut EVP_CIPHER_CTX
}

#[warn(dead_code)]
pub fn bytes_to_hex(bytes: &[u8]) -> String {
    let mut hex = String::new();
    for &byte in bytes {
        write!(&mut hex, "{:02X}", byte).unwrap();
    }
    hex
}

#[warn(dead_code)]
pub fn hex_to_bytes(hex: &str) -> Result<Vec<u8>, String> {
    let mut bytes = Vec::new();

    for chunk in hex.as_bytes().chunks(2) {
        if chunk.len() == 2 {
            match u8::from_str_radix(std::str::from_utf8(chunk).unwrap(), 16) {
                Ok(byte) => bytes.push(byte),
                Err(e) => return Err(e.to_string())
            }
            
        } else {
            return Err("Odd value".to_string());
        }
    }

    Ok(bytes)
}

impl Aes {
    const KEY_SIZE : usize = 32;
    pub const PADDING : u8 = '$' as u8;

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

        unsafe {
            if EVP_EncryptInit_ex(self.ctx, EVP_aes_256_cbc(), null_mut(), self.key.as_ptr(), self.iv.as_ptr()) != 1 {
                return Err("EVP_EncryptInit_ex() issue")
            }

            if EVP_EncryptUpdate(self.ctx, cipher_text.as_mut_ptr(), &mut len, plain.as_ptr(), plain.len() as c_int ) != 1 {
                return Err("EVP_EncryptUpdate() issue")
            }
            
            if EVP_EncryptFinal_ex(self.ctx, cipher_text.as_mut_ptr().offset(len as isize), &mut len) != 1
            {
                return Err("EVP_EncryptFinal_ex() issue")
            }
            
        }
        
        Ok(bytes_to_hex(cipher_text.as_ref()))
    }

    pub fn decrypt(&mut self, encrypted : &String) -> Result<String> {
        
        let cipher = match hex_to_bytes(encrypted) {
            Ok(cipher_text) => cipher_text,
            Err(_) => return Err("hex_to_bytes() issue") 
        };

        let mut plain_text =  vec![0u8; encrypted.len()].into_boxed_slice();

        let mut len = 0;
        
        #[allow(unused_assignments)]
        let mut ret = String::new();
        
        unsafe {
            if EVP_DecryptInit_ex(self.ctx, EVP_aes_256_cbc(), null_mut(), self.key.as_ptr(), self.iv.as_ptr()) != 1
            {
                return Err("EVP_DecryptInit_ex() issue")
            }

            if EVP_DecryptUpdate(self.ctx, plain_text.as_mut_ptr(), &mut len, cipher.as_ptr(), cipher.len() as c_int) != 1
            {
                return Err("EVP_DecryptUpdate() issue")
            }
            // plain_text_len = len;

            let plain_text_sliced = &mut plain_text[len.try_into().unwrap_or(0) .. ];
            
            if EVP_DecryptFinal_ex(self.ctx, plain_text_sliced.as_mut_ptr(), &mut len) != 1
            {
                return Err("EVP_DecryptFinal_ex() issue")
            }
            // plain_text_len += len;

            ret =  String::from_utf8(plain_text.to_vec()).unwrap();
        }
        
        Ok(ret)
    }
    
}

