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
    rc::Rc,
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
        if let Err(_) = header {
            continue;
        } 

        let header = header.unwrap();

        if header.len() == 0 {
            break;
        }
    }

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

    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        json.len(),
        json
    );

    let write_result = stream.write_all(response.as_bytes());

    if let Err(_) = write_result {
        eprintln!("Failed to write response on stream");
        return;
    }

    stream.flush().unwrap();

    println!("Stream flushed, data sent to client");

    drop(players_manager_lock);
    drop(board_lock);

    wait_and_handle_messages(stream, Arc::clone(&players_manager));
}

fn wait_and_handle_messages(
    mut stream: TcpStream,
    players_manager: Arc<Mutex<RefCell<PlayersManager>>>,
) {
    let cloned_stream = stream.try_clone().unwrap();
    let cloned_stream = Rc::new(RefCell::new(cloned_stream));

    loop {
        let cloned_stream = Rc::clone(&cloned_stream);

        for line in BufReader::new(&mut stream).lines() {
            if let Err(e) = line {
                eprintln!("Error while trying to read stream content. Error: {e}");
                continue;
            }

            let line = line.unwrap();

            println!("line: {line}");

            if line.len() == 0 {
                continue;
            }

            let players_manager_lock = players_manager.lock();

            if let Err(_) = players_manager_lock {
                eprintln!("Failed to lock players_manager");
                continue;
            }

            let mut players_manager_lock = players_manager_lock.unwrap();
            let mut cloned_stream = cloned_stream.borrow_mut();

            parse_line_and_write_to_stream(&line, players_manager_lock.get_mut(), &mut cloned_stream);

            drop(players_manager_lock);
        }
    }
}

fn parse_line_and_write_to_stream(
    line: &str,
    players_manager: &mut PlayersManager,
    stream: &mut TcpStream,
) {
    let json = serde_json::from_str::<MovePlayerInfos>(&line);

    if let Err(_) = json {
        eprintln!("Failed to deserialize message");
    }

    let move_result = routes::player::move_player(&json.unwrap(), players_manager);

    if let Err(_) = move_result {
        eprintln!("Failed to move player");
    }

    let json = serde_json::to_string(&move_result.unwrap()).unwrap() + "\r\n";

    let write_result = stream.write_all(json.as_bytes());

    if let Err(_) = write_result {
        eprintln!("Failed to write bytes to stream");
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
