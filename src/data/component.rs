use specs::prelude::*;
use std::net::TcpStream;

#[derive(Component)]
pub struct Player {
  pub socket: TcpStream
}

#[derive(Component)]
pub struct Name {
  pub val: String
}

#[derive(Debug, Component, Clone, Copy)]
pub struct Location {
  pub description: &'static str,
  pub address: (usize, usize)
}

#[derive(Component)]
pub struct Speaking {
  pub msg: String
}

#[derive(Component)]
pub struct Moving {
  pub loc: Location,
  pub to: String,
  pub from: String
}