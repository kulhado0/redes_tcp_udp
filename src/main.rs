use std::io::{stdin, stdout};
use std::io::{BufRead, BufReader};
use std::io::{Read, Write};
use std::net::TcpStream;
use std::str::from_utf8;

fn main() {
    match TcpStream::connect("localhost:8000") {
        Ok(mut stream) => {
            println!("Successfully connected to server in port 8000");

            for line in BufReader::new(&mut stream).lines() {
                match line {
                    Err(e) => println!("Failed to receive data: {}", e),
                    Ok(_) => {
                        let line = line.unwrap();

                        if line.len() == 0 { break }

                        println!("reply: {} with size {}", line, line.len());
                    }
                }
            }

            let mut s = String::new();

            print!("Enter player id: ");

            let _ = stdout().flush();

            stdin()
                .read_line(&mut s)
                .expect("Did not enter a correct string");

            if let Some('\n') = s.chars().next_back() {
                s.pop();
            }

            if let Some('\r') = s.chars().next_back() {
                s.pop();
            }

            println!("You typed: {}", s);

            let msg = format!("{{\"player_id\":\"{s}\",\"direction\":\"Down\"}}\r\n");

            println!("will send msg: {msg}");

            stream.write_all(msg.as_bytes()).unwrap();

            println!("Sent data, awaiting reply again...");

            for line in BufReader::new(&mut stream).lines() {
                match line {
                    Ok(_) => {
                        let line = line.unwrap();

                        if line.len() == 0 { break }

                        println!("reply: {} with size {}", line, line.len());
                    }
                    Err(e) => {
                        println!("Failed to receive data: {}", e);
                    }
                }
            }
        }
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
    println!("Terminated.");
}

// {"player_id":"e3eec3cc-09a7-4d2f-adce-e04d9084d58a","direction":"Up"}
