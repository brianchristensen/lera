use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};

pub fn start() {
  let listener = TcpListener::bind("0.0.0.0:23").unwrap();
  // accept connections and process them, spawning a new thread for each one
  println!("Server listening on port 23");
  for stream in listener.incoming() {
      match stream {
          Ok(stream) => {
              println!("New connection: {}", stream.peer_addr().unwrap());
              thread::spawn(move|| {
                  // connection succeeded
                  handle_client(stream)
              });
          }
          Err(e) => {
              println!("Error: {}", e);
              /* connection failed */
          }
      }
  }
  // close the socket server
  drop(listener);
}

fn handle_client(mut stream: TcpStream) {
  let mut data = [0 as u8; 32];
  while match stream.read(&mut data) {
      Ok(size) => {
          println!("{:?}", data);
          stream.write(&data[0..size]).unwrap();
          true
      },
      Err(_) => {
          println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
          stream.shutdown(Shutdown::Both).unwrap();
          false
      }
  } {}
}