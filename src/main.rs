mod consts;
mod domain;
mod requests;
mod responses;
mod routes;

use std::{
    cell::RefCell,
    io::Read,
    io::Write,
    net::{Shutdown, TcpListener, TcpStream},
    sync::{Arc, Mutex},
    thread,
};

use consts::boards;
use domain::{
    board::{board::Board, board_creator::BoardCreator},
    player::{player::Player, players_manager::PlayersManager},
};

use crate::{
    requests::on_move_message::MovePlayerInfos, responses::on_connection::ConnectionEstablished,
};

fn handle_new_connection(
    stream: &mut TcpStream,
    players_manager: Arc<Mutex<RefCell<PlayersManager>>>,
    board: Arc<Mutex<Board>>,
) {
    let players_manager_lock = players_manager
        .lock()
        .expect("Failed to lock manager in new connection");

    let board_lock = board
        .lock()
        .expect("Failed to lock board in new connection");

    let player_name = &stream.peer_addr().unwrap().to_string();

    println!("on new connection: new player name: {}", player_name);

    let players = (*players_manager_lock).borrow().players().to_vec();
    let new_player = Player::new(player_name);

    players_manager_lock
        .borrow_mut()
        .add_player(new_player.clone());

    let response = ConnectionEstablished::new((*board_lock).clone(), players, new_player.clone());
    let json = serde_json::to_string(&response).expect("Failed to serialize response");

    println!("response: {}", json);

    stream
        .write(json.as_bytes())
        .expect("Failed to write response on stream");
    
    drop(players_manager_lock);
    drop(board_lock);

    wait_and_handle_messages(stream, Arc::clone(&players_manager));
}

fn wait_and_handle_messages(
    stream: &mut TcpStream,
    players_manager: Arc<Mutex<RefCell<PlayersManager>>>,
) {
    loop {
        let mut data = [0u8; 1024];

        match stream.read(&mut data) {
            Ok(size) => {
                if size == 0 {
                    return;
                }

                let stream_data = String::from_utf8_lossy(&data);

                let json = serde_json::from_str::<MovePlayerInfos>(&stream_data)
                    .expect("Failed to deserialize message");

                let mut players_manager = players_manager
                    .lock()
                    .expect("Failed to lock players_manager");

                routes::player::move_player(&json, players_manager.get_mut())
                    .expect("Failed to move player");

                stream.write(&data[0..size])
                    .expect("Failed to write bytes to stream");
            }
            Err(_) => {
                println!("An error occurred with {}", stream.peer_addr().unwrap());

                stream.shutdown(Shutdown::Both).unwrap();
            }
        }
    }
}

fn main() {
    let tile_symbols = boards::DEFAULT_BOARD_TILE_SYMBOLS
        .map(|slice| slice.to_vec())
        .to_vec();

    let board = BoardCreator::create_from(tile_symbols, "board0");
    let players_manager = PlayersManager::new(&board);

    let board = Arc::new(Mutex::new(board));
    let players_manager = Arc::new(Mutex::new(RefCell::new(players_manager)));

    let tcp_listener = TcpListener::bind("0.0.0.0:8000")
        .expect("Failed to bind TCP listener to localhost on port 8000");

    println!("Server listening on port 8000");

    for stream in tcp_listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());

                let players_manager = Arc::clone(&players_manager);
                let board = Arc::clone(&board);

                thread::spawn(move || {
                    handle_new_connection(&mut stream, players_manager, board);
                });
            }
            Err(e) => {
                println!("Error occurred: {e}");
            }
        }
    }
}
