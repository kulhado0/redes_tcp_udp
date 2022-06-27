mod consts;
mod domain;
mod requests;
mod responses;
mod routes;

use std::{
    cell::RefCell,
    io::Write,
    io::{BufRead, BufReader},
    net::{TcpListener, TcpStream},
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
    mut stream: TcpStream,
    players_manager: Arc<Mutex<RefCell<PlayersManager>>>,
    board: Arc<Mutex<Board>>,
) {
    let players_manager_lock = players_manager.lock();

    if let Err(_) = players_manager_lock {
        eprintln!("Failed to lock players manager");
        return;
    }

    let board_lock = board.lock();

    if let Err(_) = board_lock {
        eprintln!("Failed to lock board");
        return;
    }

    for header in BufReader::new(&mut stream).lines() {
        let header = header.unwrap();
        if header == "\r" {
            break;
        }
        println!("header: {header}");
    }

    stream.write_all(b"HTTP/1.1 200 OK\r\n\r\n").unwrap();

    let player_name = &stream.peer_addr().unwrap().to_string();

    let players_manager_lock = players_manager_lock.unwrap();
    let board_lock = board_lock.unwrap();

    let players = (*players_manager_lock).borrow().players().to_vec();
    let new_player = Player::new(player_name);

    players_manager_lock
        .borrow_mut()
        .add_player(new_player.clone());

    let response = ConnectionEstablished::new((*board_lock).clone(), players, new_player.clone());
    let json = serde_json::to_string(&response);

    if let Err(_) = json {
        eprintln!("Failed to serialize response");
        return;
    }

    let json = json.unwrap();

    let write_result = stream.write_all(json.as_bytes());

    if let Err(_) = write_result {
        eprintln!("Failed to write response on stream");
        return;
    }

    drop(players_manager_lock);
    drop(board_lock);

    wait_and_handle_messages(stream, Arc::clone(&players_manager));
}

fn wait_and_handle_messages(
    stream: TcpStream,
    players_manager: Arc<Mutex<RefCell<PlayersManager>>>,
) {
    let stream = Arc::new(Mutex::new(RefCell::new(stream)));

    loop {
        let stream_lock = stream.lock();

        if let Err(_) = stream_lock {
            eprintln!("Failed to lock stream");
            continue;
        }

        let stream = stream_lock.unwrap();

        for line in BufReader::new(&*stream.borrow_mut()).lines() {
            if let Err(e) = line {
                eprintln!("Error while trying to read stream content. Error: {e}");
                continue;
            }

            let line = line.unwrap();

            if line.len() == 0 {
                continue;
            }

            println!("line: {}", line);

            let json = serde_json::from_str::<MovePlayerInfos>(&line);

            if let Err(_) = json {
                eprintln!("Failed to deserialize message");
                continue;
            }

            let players_manager_lock = players_manager.lock();

            if let Err(_) = players_manager_lock {
                eprintln!("Failed to lock players_manager");
                continue;
            }

            let mut players_manager_lock = players_manager_lock.unwrap();

            let move_result = routes::player::move_player(
                &json.unwrap(),
                players_manager_lock.get_mut(),
            );

            if let Err(_) = move_result {
                eprintln!("Failed to move player");
                continue;
            }

            let json = serde_json::to_string(&move_result.unwrap()).unwrap();

            let write_result = stream.borrow_mut()
                .write_all(json.as_bytes());

            if let Err(_) = write_result {
                eprintln!("Failed to write bytes to stream");
                continue;
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
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());

                let players_manager = Arc::clone(&players_manager);
                let board = Arc::clone(&board);

                thread::spawn(move || {
                    handle_new_connection(stream, players_manager, board);
                });
            }
            Err(e) => {
                println!("Error occurred: {e}");
            }
        }
    }
}
