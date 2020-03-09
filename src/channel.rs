use uuid::Uuid;
use std::sync::mpsc::channel;
use std::sync::mpsc::{Sender, Receiver};
use std::net::TcpStream;

pub enum ChannelPayload {
  Cmd((Uuid, String)),
  Target((Uuid, String, String)),
  Join((Uuid, String, TcpStream)),
}

pub fn start() -> (Sender<ChannelPayload>, Receiver<ChannelPayload>) {
  let (tx, rx) = channel::<ChannelPayload>();
  (tx, rx)
}