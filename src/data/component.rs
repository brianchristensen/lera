use specs::prelude::*;
use std::net::TcpStream;

#[derive(Component)]
pub struct Player {
  pub socket: TcpStream
}

#[derive(Component)]
pub struct Name {
  pub name: String
}

#[derive(Debug, Component, Clone, Copy)]
pub struct Location {
  pub description: &'static str,
  pub address: (usize, usize)
}
