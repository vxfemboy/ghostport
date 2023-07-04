use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};

fn main() {
    let listener = TcpListener::bind("0.0.0.0:22").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let _ = stream.write_all(b"SSH-2.0-OpenSSH_7.2p2 Ubuntu-uwuntu\r\n");
                let _ = stream.flush();
                let mut buffer = [0; 1024];
                let _ = stream.read(&mut buffer);
                // log the connection and any commands sent by the attacker
            },
            Err(e) => {
                eprintln!("Error accepting connection: {}", e);
            }
        }
    }
}
