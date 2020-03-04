use crate::world;

use uuid::Uuid;
use std::thread;
use std::sync::{Arc, Mutex};
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{BufRead, BufReader};
use std::sync::mpsc::Sender;

pub fn start(world: Arc<Mutex<world::World>>, tx: Sender<(Uuid, String)>) {
  let listener = TcpListener::bind("0.0.0.0:23").unwrap();
  // accept connections and process them, spawning a new thread for each one
  println!("Server listening on port 23");
  for socket in listener.incoming() {
      let worldc = world.clone();
      let txc = tx.clone();
      match socket {
          Ok(socket) => {
              println!("New connection: {}", socket.peer_addr().unwrap());
              thread::spawn(move || {
                  // connection succeeded
                  new_client(worldc, socket, txc)
              });
          }
          Err(e) => {
              println!("Connection error: {}", e);
          }
      }
  }
  // close the socket server
  drop(listener);
}

// join a new client to the game
fn new_client(world: Arc<Mutex<world::World>>, socket: TcpStream, tx: Sender<(Uuid, String)>) {
    // generate uuid for client
    let id = Uuid::new_v4();
    // join the game world
    world::join(world, id, socket.try_clone().unwrap());

    // loop a line reader on the socket to read user commands
    let mut reader = BufReader::new(socket.try_clone().unwrap());
    let buf = &mut vec![];
    loop {
        let result = reader.read_until(b'\n', buf);
        match result {
            Ok(_data) => {
                let ascii = buf
                    .iter()
                    .filter(|&&x| x.is_ascii_alphanumeric() || x.is_ascii_digit() || x == b' ' || x == b'?')
                    .map(|&x| x).collect::<Vec<u8>>();
                let cmd = String::from(std::str::from_utf8(&ascii).unwrap());
                tx.send((id, cmd)).unwrap();
                buf.drain(..);
                ()
            }
            Err(e) => {
                println!("An error occurred, terminating connection: {:?}", e);
                socket.shutdown(Shutdown::Both).unwrap();
                break
            }
        }
    }
}
