use crate::world::World;
use uuid::Uuid;
use rand::prelude::*;
use std::sync::{Arc, Mutex};
use std::net::{TcpStream};
use std::io::{Write};

pub struct Player {
  pub id: Uuid,
  pub name: String,
  pub world: Arc<Mutex<World>>,
  pub location: (usize, usize),
  pub socket: TcpStream
}

impl Player {
    pub fn new(id: Uuid, name: String, world: Arc<Mutex<World>>, socket: TcpStream) -> Player {
        let mut rng = thread_rng();
        Player {
            id: id,
            name: name,
            world: world,
            location: (rng.gen_range(0, 9), rng.gen_range(0, 9)),
            socket: socket
        }
    }

    pub fn look(&self) {
        let world = self.world.lock().unwrap();
        let (x, y) = self.location;
        let loc = &world.world_map[x][y];
        &self.socket.try_clone().unwrap().write(loc.description.as_bytes()).unwrap();
        &self.socket.try_clone().unwrap().write(&[b'\n']).unwrap();
    }

    pub fn write(&self, string: String) {
        &self.socket.try_clone().unwrap().write(string.as_bytes()).unwrap();
    }
}
