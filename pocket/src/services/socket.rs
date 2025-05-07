use std::io::{Error, Read};
use std::net::TcpStream;
use std::process::abort;
use std::thread;
use std::thread::JoinHandle;

pub struct Socket {
    pub address: String,
    pub writing : TcpStream,
    pub connected : bool,
    pub enable_reading_loop : bool
}

impl Socket {
    
    pub fn connect(address: String) -> Result<Self, Error> {

        let mut ret = Self {
            address: address.clone(),
            writing: match TcpStream::connect(&address) {
                Ok(stream) => stream,
                Err(e) => return Err(e)
            },
            connected: false,
            enable_reading_loop: true
        };

        ret.connected = true;
        
        let writing = match ret.writing.try_clone().ok() {
            Some(x) => x,
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
                    Ok(bytes_read) if *bytes_read > 0usize => {
                        let response = String::from_utf8_lossy(&buffer[..*bytes_read]);
                        println!("Risposta dal server: {}", response);
                    }
                    Ok(_) => {
                        println!("Nessun dato ricevuto.");
                    }
                    Err(e) => {
                        println!("Errore durante la lettura: {}", e);
                    }
                }
                
                if !ret.enable_reading_loop {
                    break 'reading;
                }
            }
        });

       Ok(ret)
    }
    
    pub fn is_connected(&self) -> bool {
        self.connected
    }
    
    pub fn shutdown(&mut self) -> Result<(), Error> {
        self.writing.shutdown(std::net::Shutdown::Both)?;
        Ok(())
    }
    
}