mod consts;
mod domain;
mod requests;
mod routes;

use std::{env, io::Read, io::Write, net::TcpListener, sync::RwLock, thread};

use consts::boards;
use domain::{board::board_creator::BoardCreator, player::players_manager::PlayersManager};

fn main() {
    let tile_symbols = boards::DEFAULT_BOARD_TILE_SYMBOLS
        .map(|slice| slice.to_vec())
        .to_vec();

    let board = BoardCreator::create_from(tile_symbols, "board0");
    let players_manager = PlayersManager::new(10, &board);

    let board = RwLock::new(board);
    let players_manager = RwLock::new(players_manager);

    let tcp_listener = TcpListener::bind("0.0.0.0:8000").unwrap();

    println!("Server listening on port 8000");

    for stream in tcp_listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                thread::spawn(move || {
                    let mut data = [0u8; 50];
                    while match stream.read(&mut data) {
                        Ok(size) => {
                            stream.write(&data[0..size]).unwrap();
                            true
                        }
                        Err(_) => {
                            println!("An error occurred with {}", stream.peer_addr().unwrap());
                            stream.shutdown(Shutdown::Both).unwrap();
                            false
                        }
                    } {}
                });
            }
            Err(e) => {
                println!("Error occurred: {e}");
            }
        }
    }
}
