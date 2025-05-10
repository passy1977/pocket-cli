use std::fmt::Display;

#[derive(Clone)]
pub enum Response {
    Ok = 0,
    Error = 1,
    WrongParams = 2,
    UserAlreadyExist = 3,
    DeviceAlreadyExist = 4,
    UserNotExist = 5,
    DeviceNotExist = 6,
    WrongPasswd = 7
}
impl Display for Response {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Response::*;

        match *self {
            Ok => write!(f, "Ok({})", self.clone() as u8),
            Error => write!(f, "Error({})", self.clone() as u8),
            WrongParams => write!(f, "WrongParams({})", self.clone() as u8),
            UserAlreadyExist => write!(f, "UserAlreadyExist({})", self.clone() as u8),
            DeviceAlreadyExist => write!(f, "DeviceAlreadyExist({})", self.clone() as u8),
            UserNotExist => write!(f, "UserNotExist({})", self.clone() as u8),
            DeviceNotExist => write!(f, "DeviceNotExist({})", self.clone() as u8),
            WrongPasswd => write!(f, "WrongPasswd({})", self.clone() as u8),
        }
    }
}

impl PartialEq for Response {
    fn eq(&self, other: &Self) -> bool {
        self.clone() as u8 == other.clone() as u8
    }
}

impl Response {
    
    #[allow(dead_code)]
    pub fn check(&self, str: &String) -> bool {
        use Response::*;

        let str = str.trim();
        
        match self {
            x if *x == Ok && str == "0" => true,
            x if *x == Error && str == "1" => true,
            x if *x == WrongParams && str == "2" => true,
            x if *x == UserAlreadyExist && str == "3" => true,
            x if *x == DeviceAlreadyExist && str == "4" => true,
            x if *x == UserNotExist && str == "5" => true,
            x if *x == DeviceNotExist && str == "6" => true,
            x if *x == WrongPasswd && str == "7" => true,
            _ => false
        }
    }

    pub fn to_response(str: &String) -> Self {
        use Response::*;

        let str = str.trim();
        
        match str {
            _x @ "0" => Ok,
            _x @ "1" => Error,
            _x @ "2" => WrongParams,
            _x @ "3" => UserAlreadyExist,
            _x @ "4" => DeviceAlreadyExist,
            _x @ "5" => UserNotExist,
            _x @ "6" => DeviceNotExist,
            _x @ "7" => WrongPasswd,
            _ => Error
        }
    }
}