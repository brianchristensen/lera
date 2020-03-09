use crate::render::title;
use crate::channel::ChannelPayload;

use uuid::Uuid;
use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{BufRead, BufReader, Write};
use std::sync::mpsc::Sender;

pub fn start(tx: Sender<ChannelPayload>) {
    let listener = TcpListener::bind("0.0.0.0:23").unwrap();
    // accept connections and process them, spawning a new thread for each one
    println!("Server listening on port 23");
    for socket in listener.incoming() {
        let player_creation_tx = tx.clone();
        let cmd_tx = tx.clone();
        match socket {
            Ok(mut socket) => {
                // connection succeeded
                println!("New connection: {}", socket.peer_addr().unwrap());
                thread::spawn(move || {
                    // print game title
                    socket.write(title::print()).unwrap();
                    // wait for player to enter name
                    let id = get_player_name(socket.try_clone().unwrap(), player_creation_tx);
                    // player successfully joined game, wait for commands
                    wait_for_commands(id, socket, cmd_tx)
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

fn wait_for_commands(id: Uuid, mut socket: TcpStream, tx: Sender<ChannelPayload>) {
    // create a line reader to collect user input
    let reader_socket = socket.try_clone().unwrap();
    let mut reader = BufReader::new(reader_socket);
    let buf = &mut vec![];

    // loop the line reader on the socket to wait for user commands
    loop {
        let result = reader.read_until(b'\n', buf);
        match result {
            Ok(_data) => {
                let ascii = buf
                    .iter()
                    .filter(|&&x| x.is_ascii_alphanumeric() || x.is_ascii_digit() || x == b' ' || x == b'?')
                    .map(|&x| x).collect::<Vec<u8>>();
                let raw_cmd = String::from(std::str::from_utf8(&ascii).unwrap());
                let fmt_cmd = raw_cmd.replace("\n", "");
                let cmd = fmt_cmd.as_str().split(' ').filter(|&x| x != "").collect::<Vec<&str>>();
                if cmd.len() == 1 {
                    tx.send(ChannelPayload::Cmd((id, cmd[0].to_lowercase()))).unwrap();
                } else if cmd.len() == 2 {
                    tx.send(ChannelPayload::Target((id, cmd[0].to_lowercase(), cmd[1].to_lowercase()))).unwrap();
                } else {
                    socket.write(format!("Unknown command: {}", cmd[0]).as_bytes()).unwrap();
                }

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

fn get_player_name(mut socket: TcpStream, tx: Sender<ChannelPayload>) -> Uuid {
    // generate uuid for client
    let id = Uuid::new_v4();

    // create a line reader to collect user input
    let reader_socket = socket.try_clone().unwrap();
    let mut reader = BufReader::new(reader_socket);
    let buf = &mut vec![];

    // wait for a valid name
    'naming_game : loop {
        socket.write("Please enter your desired character name\n".as_bytes()).unwrap();
        socket.write("Your name must be at least two characters: ".as_bytes()).unwrap();
        let result = reader.read_until(b'\n', buf);
        match result {
            Ok(_data) => {
                let ascii = buf
                    .iter()
                    .filter(|&&x| x.is_ascii_alphabetic())
                    .map(|&x| x).collect::<Vec<u8>>();
                let cmd = String::from(std::str::from_utf8(&ascii).unwrap());

                if cmd.len() >= 2 && cmd != " " {
                    socket.write(format!("Welcome {}!\n\n", cmd).as_bytes()).unwrap();
                    tx.send(ChannelPayload::Join((id, cmd, socket))).unwrap();
                    break 'naming_game
                } else {
                    buf.drain(..);
                    continue 'naming_game
                }
            }
            Err(e) => {
                println!("An error occurred, terminating connection: {:?}", e);
                socket.shutdown(Shutdown::Both).unwrap();
                break 'naming_game
            }
        }
    }
    id
}
