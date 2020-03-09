use specs::prelude::*;
use uuid::Uuid;
use std::net::TcpStream;

#[derive(Component)]
pub struct Player {
  pub id: Uuid,
  pub name: String,
  pub socket: TcpStream
}

#[derive(Component, Clone, Copy)]
pub struct Location {
  pub description: &'static str,
  pub address: (usize, usize)
}
