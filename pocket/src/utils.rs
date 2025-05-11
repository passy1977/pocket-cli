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
    aad.split(mac_in_bytes.len());
    
    
    
    
    
    
    
    Ok("$".to_owned() + &encode.to_string())
}