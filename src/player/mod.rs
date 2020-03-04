use uuid::Uuid;
use rand::prelude::*;
use std::net::{TcpStream};
use std::io::{Write};

pub struct Player {
  pub id: Uuid,
  pub name: String,
  pub location: (usize, usize),
  pub socket: TcpStream
}

impl Player {
    pub fn new(id: Uuid, name: String, socket: TcpStream) -> Player {
        let mut rng = thread_rng();
        Player {
            id: id,
            name: name,
            location: (rng.gen_range(0, 9), rng.gen_range(0, 9)),
            socket: socket
        }
    }

    pub fn write(&self, string: String) {
        &self.socket.try_clone().unwrap().write(string.as_bytes()).unwrap();
    }

    pub fn setName(&mut self, name: String) {
        self.name = name;
    }
}
