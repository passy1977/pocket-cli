use std::io::{Error, ErrorKind, Read, Write};
use std::net::TcpStream;
use std::process::abort;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub struct Socket {
    pub writing : TcpStream,
    pub rx : mpsc::Receiver<String>,
    pub connected : bool,
    pub enable_reading_loop : bool
}

impl Socket {
    
    const TIMEOUT : u64 = 1_000;
    
    pub fn connect(address: String) -> Result<Self, Error> {

        let (tx, rx) = mpsc::channel::<String>();
        
        let mut ret = Self {
            writing: match TcpStream::connect(&address) {
                Ok(stream) => stream,
                Err(e) => return Err(e)
            },
            connected: false,
            enable_reading_loop: true,
            rx
        };

        
        
        ret.connected = true;
        
        let writing = match ret.writing.try_clone().ok() {
            Some(stream) => stream,
            None => abort(),
        };
        
        thread::spawn(move || {

            let mut reading = match writing.try_clone() {
                Ok(stream) => stream,
                Err(e) => {
                    eprintln!("Failed to clone socket: {:?}", e);
                    return
                }
            };

            'reading: loop {
                let mut buffer = [0; 1024];
                match &reading.read(&mut buffer) {
                    Ok(bytes_read) if *bytes_read > 0usize => tx.send(String::from_utf8_lossy(&buffer[..*bytes_read]).to_string()).unwrap(),
                    Ok(_) => println!("No data received!"),
                    Err(e) => println!("Error during reading: {}", e)
                }
                
                if !ret.enable_reading_loop {
                    break 'reading;
                }
            }
        });

       Ok(ret)
    }
    
    pub fn write(&mut self, s: &String) -> Result<String, Error> {
        self.write_str(s.as_str())
    }

    pub fn write_str(&mut self, s: &str) -> Result<String, Error> {
        self.writing.write_all(s.as_bytes())?;
        self.writing.write_all("\n".as_bytes())?;
        self.writing.flush()?;
        
        match self.rx.recv_timeout(Duration::from_millis(Socket::TIMEOUT)) {
            Ok(str) => Ok(str),
            Err(e) => Err(Error::new(ErrorKind::ConnectionAborted, e.to_string()))
        }
    }
    
    pub fn is_connected(&self) -> bool {
        self.connected
    }
    
    pub fn shutdown(&mut self) -> Result<(), Error> {
        self.writing.shutdown(std::net::Shutdown::Both)?;
        Ok(())
    }
    
}