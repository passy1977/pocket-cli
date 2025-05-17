use crate::services::aes::Aes;
use mac_address::get_mac_address;
use openssl_sys::RAND_bytes;
use std::{error, fmt};
use libc::c_int;

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
        RAND_bytes(buffer.as_mut_ptr(), buffer.capacity() as c_int);
    }


    let mut result = String::with_capacity(length);
    for byte in buffer.iter_mut() {
        let byte =  *byte as char;
        
        if byte >= ' ' && byte <= '~' {
            result.push(byte);
        } else {
            result.push(Aes::PADDING as char);
        }
    }
    
    result
}
