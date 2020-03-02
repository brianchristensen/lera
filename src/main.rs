extern crate rand;
mod map;
mod server;

use map::gen_map;
use std::sync::mpsc::channel;
use std::thread;
use std::io;
use rand::prelude::*;

fn main() {
    //server::start();
    let mut rng = thread_rng();
    let m = gen_map()[rng.gen_range(0, 9)][rng.gen_range(0, 9)];
    println!("{}", m);

    let (tx, rx) = channel::<String>();

    thread::spawn(move|| {
        loop {
            let mut buffer = String::new();
            io::stdin().read_line(&mut buffer).unwrap();
            tx.send(buffer.clone()).unwrap();
        }
    });

    loop {
        let raw_cmd = rx.recv().unwrap().to_lowercase().replace("\n", "");
        let cmd = raw_cmd.as_str();

        match cmd {
            "quit" | "exit" => break,
            _ => println!("CMD: {}", cmd)
        }
    }
}
