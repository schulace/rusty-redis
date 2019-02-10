use std::io;
use std::io::prelude::*;
use std::net::TcpStream;

fn main() {
  loop {
    let mut s = String::new();
    match io::stdin().read_line(&mut s) {
      Ok(0) => break, //EOF
      Ok(_) => {
        let mut stream = TcpStream::connect("127.0.0.1:7878").unwrap();
        let mut read_buffer = Vec::<u8>::new();
        stream.write_all(s.as_bytes()).unwrap();
        stream.read_to_end(&mut read_buffer).unwrap();
        println!("{}", std::str::from_utf8(&read_buffer).unwrap());
      }
      Err(_) => break,
    }
  }
}
