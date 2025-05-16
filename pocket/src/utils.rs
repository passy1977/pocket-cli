use crate::services::aes::Aes;
use libc::size_t;
use mac_address::get_mac_address;
use std::ffi::c_void;
use std::{error, fmt};

pub type Result<T, E = &'static str> = std::result::Result<T, E>;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    Undefine,
    Msg(String)
}

impl fmt::Display for Error {
    
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        use crate::utils::Error::*;
        
        match self {
            Undefine => write!(f, "Undefine"),
            Msg(msg) => write!(f, "{}", msg)
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        use crate::utils::Error::*;
        
        match self {
            Undefine => "UndefineError",
            Msg(msg) => msg,
            //_ => "GenericError"
        }
    }
}


pub(crate) fn handle_passwd(passwd: &String, iv: &Option<Vec<u8>>, encrypt: bool) -> Result<String, String> {

    if passwd.len() < 16 {
        return Err("Passwd too short".to_string());
    }

    let iv = match iv {
        None => return Err("IV not valid".to_string()),
        Some(iv) => iv.as_slice()
    };

    let mut key:  [u8; 32]= [Aes::PADDING; 32];

    let mac_in_bytes  = match get_mac_address() {
        Ok(Some(ma)) => ma.bytes(),
        Ok(None) => return Err("Mac not found".to_string()),
        Err(_) => return Err("Error in translation".to_string()),
    };

    key[..mac_in_bytes.len()].copy_from_slice(&mac_in_bytes);

    let mut aes = Aes::new(key, iv.try_into().unwrap());
    
    if encrypt {
        match aes.encrypt(passwd) {
            Ok(cipher_text) => Ok(cipher_text),
            Err(e) => Err(e.to_string())
        }
    } else {
        match aes.decrypt(passwd) {
            Ok(plain_text) => Ok(plain_text),
            Err(e) => Err(e.to_string())
        }
    }
}

pub(crate) fn generate_random_string(length: usize) -> String {
    let mut buffer = vec![0u8; length];

    unsafe {
        // Chiamata di sistema getrandom
        libc::getrandom(buffer.as_mut_ptr() as *mut c_void, length as size_t, 0);
    }

    // Converti i byte casuali in caratteri ASCII (solo per semplicità)
    let mut result = String::with_capacity(length);
    for &byte in buffer.iter() {
        if (byte >= b'a' && byte <= b'z') || (byte >= b'A' && byte <= b'Z') || (byte >= b'0' && byte <= b'9') {
            result.push(byte as char);
        }
    }

    result
}
