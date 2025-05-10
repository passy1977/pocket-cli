use std::{ error, fmt };
use mac_address::get_mac_address;

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


pub(crate) fn handle_passwd(passwd: &String, encode: bool) -> Result<String> {

    let mut aad :  [u8; 32]= ['$' as u8; 32];

    let mac_in_bytes  = match get_mac_address() {
        Ok(Some(ma)) => ma.bytes(),
        Ok(None) => return Err("Mac not found"),
        Err(_) => return Err("Error in translation"),
    };

    aad[..mac_in_bytes.len()].copy_from_slice(&mac_in_bytes);

    tink_aead::init();
    let kh = tink_core::keyset::Handle::new(&tink_aead::aes256_gcm_key_template()).ok().unwrap();
    if let Ok(a) = tink_aead::new(&kh) {
        Ok(
            if encode {
                if let Ok(ec) = a.encrypt(passwd.as_bytes(), &aad) {
                    hex::encode(&ec)
                } else {
                    return Err("Encode error")
                }
            } else {
                if let Ok(dc) = a.decrypt(passwd.as_bytes(), &aad) {
                    String::from_utf8(dc).expect("Unable convert password to utf8")
                } else {
                    return Err("Decode error")
                }
            }
        )
    } else {
        Err("Keyset error")
    }
}