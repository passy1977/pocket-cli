use mac_address::get_mac_address;
use std::{error, fmt};
use crate::services::aes::Aes;

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


pub(crate) fn handle_passwd(passwd: &String, encrypt: bool) -> Result<String, String> {

    if passwd.len() < 16 {
        return Err("Passwd too short".to_string());
    }
    
    let key:  [u8; 32]= ['$' as u8; 32];
    let iv= b"\x00\x01\x02\x03\x04\x05\x06\x07\x08\x09\x0A\x0B\x0C\x0D\x0E\x0F";
    

    let mac_in_bytes  = match get_mac_address() {
        Ok(Some(ma)) => ma.bytes(),
        Ok(None) => return Err("Mac not found".to_string()),
        Err(_) => return Err("Error in translation".to_string()),
    };

    let mut aes = Aes::new(key, *iv);

    unsafe { 
        aes.encrypt(passwd); 
    }
    
    // key[..mac_in_bytes.len()].copy_from_slice(&mac_in_bytes);
    //
    // let v = passwd.trim().as_bytes();
    //
    // let mut block = GenericArray::<u8, typenum::U32>::from_slice(v);
    //
    // let z = GenericArray::<u8, typenum::U32>::from_slice(&key);
    //
    // let cipher = Aes256::new(&z);
    //
    // if encrypt {
    //     cipher.encrypt_block(&mut block);
    // } else {
    //     cipher.decrypt_block(&mut block);
    // }

    // match String::from_utf8(block.as_ref().to_vec()) {
    //     Ok(str) => Ok(str),
    //     Err(err) => Err(err.to_string())
    // }
    
    Ok("".to_ascii_lowercase())
}