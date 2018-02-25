extern crate blackjack;
extern crate hellohttp;

use blackjack::GameMap;
use hellohttp::ThreadPool;

use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
// use std::fs::File;
use std::str;
use std::sync::{Mutex, Arc};
// use std::thread;
// use std::time::Duration;

fn main() {
    let mut game = GameMap::new();
    let s1 = game.create_game();
    println!("{:?}", s1);
    let s2 = game.hit_game(1);
    println!("{:?}", s2);
    println!("{:?}", game.hit_game(1));
    let game_mutex = Arc::new(Mutex::new(game));
    start_listener(&game_mutex);
}

fn start_listener(game_mutex: &Arc<Mutex<GameMap>>) {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let game_mutex = Arc::clone(&game_mutex);
        pool.execute(move || {
            handle_connection(stream, game_mutex);
        });
    }

    println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream, game_mutex: Arc<Mutex<GameMap>>) {
    let mut buffer = [0; 512];
    let len = stream.read(&mut buffer).unwrap();
    let buf_string = str::from_utf8(&buffer[..len]).unwrap();
    let mut split_n = buf_string.splitn(3, " ");
    let resource_path = split_n.nth(1).unwrap();
    let path_segments: Vec<&str> = resource_path.split("/").filter(|s| *s != "").collect();
    println!("Got path segments: {:?}", path_segments);
    if path_segments.is_empty() {
        let new_game = game_mutex.lock().unwrap().create_game();
        stream.write(new_game.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
    else {
        let action = path_segments[0];
        let id = String::from(path_segments[1]).parse::<usize>().unwrap();
        let response = 
            if action == "hit" {
                game_mutex.lock().unwrap().hit_game(id)
            } else if action == "stand" {
                game_mutex.lock().unwrap().stand_game(id)
            } else { // "finish"
                game_mutex.lock().unwrap().finish_game(id)
            };
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }

    // let get = b"GET / HTTP/1.1\r\n";

    // let (status_line, filename) = if buffer.starts_with(get) {
    //     ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    // } else {
    //     ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    // };

    // let mut contents = String::new();
    // let response = format!("{}{}", status_line, contents);

    // stream.write(response.as_bytes()).unwrap();
    // stream.flush().unwrap();
}
