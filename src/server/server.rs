use mio::{Events, Poll, Token, Ready, PollOpt};
use std::io::{Read, Write};
use std::net::SocketAddr;
use std::time::{Duration, Instant};
use crate::model::Blockchain;
use anyhow::Result;


pub fn server_single_thread() {

    // Create the blockchain
    let mut blockchain = Blockchain::new();
    let address: SocketAddr = "127.0.0.1:8000".parse().unwrap();
    let listener = match mio::net::TcpListener::bind(&address) {
        Ok(listener) => {
            listener
        }
        Err(err) => {
            eprintln!("Error binding to {}: {}", address, err);
            return;
        }
    };


    let poll = Poll::new().unwrap();
    poll.register(
        &listener, 
        Token(0),
        Ready::readable(),
        PollOpt::edge()).unwrap();
        

    let mut events = Events::with_capacity(1024);
    let mut last_process_time = Instant::now();

    println!("Server listening on 127.0.0.1:8080");
    loop {

        poll.poll(&mut events, Some(Duration::from_millis(500))).unwrap();
        for event in &events {
            // handle the event
            println!("Event: {:?}", event);
            match event.token() {
                Token(0) => {
                    println!("event readines: {:?}", event.readiness().is_readable());
                    if event.readiness().is_readable() {
                        // Accept incoming connections
                        while let Ok((mut socket, _)) = listener.accept() {
                            let mut buffer = [0; 1024];
                            let mut _request: Option<usize> = None;
                            loop {
                                match socket.read(&mut buffer) {
                                    Ok(request) => {
                                        // Data successfully read, continue processing
                                        _request = Some(request);
                                        break;
                                    }
                                    Err(ref err) if err.kind() == std::io::ErrorKind::WouldBlock => {
                                        // Data not yet available, wait a bit
                                        std::thread::sleep(Duration::from_millis(10));
                                    }
                                    Err(err) => {
                                        eprintln!("Error reading from socket: {}", err);
                                        let response = format!("HTTP/1.1 400 BAD REQUEST\r\n\r\n{}", err);
                                        let _ = socket.write_all(response.as_bytes());
                                        return;
                                    }
                                }
                            }
                            let request = _request.unwrap();
                            let request_str = String::from_utf8_lossy(&buffer[..request]);

                            let balance_prefix = "GET /balance/";
                            let create_account_prefix = "POST /account/";
                            let transfer_prefix = "POST /transfer/";

                            if request_str.starts_with(balance_prefix) {
                                // Balance ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
                                let id_start: usize = balance_prefix.len();
                                let id_end = request_str.find(" HTTP/1.1").unwrap_or(request_str.len());
                                let id = request_str[id_start..id_end].to_string();
                                let balance = blockchain.get_balance(&id.to_string()); 
                                match balance {
                                    Ok(balance) => {
                                        let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", balance);
                                        let _ = socket.write_all(response.as_bytes());
                                    }
                                    Err(err) => {
                                        let response = format!("HTTP/1.1 404 NOT FOUND\r\n\r\n{}", err);
                                        let _ = socket.write_all(response.as_bytes());
                                    }
                                }
                                
                            } else if request_str.starts_with(create_account_prefix) {
                                // Transaction: Create account ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
                                // Process request
                                let body_start = request_str.find("\r\n\r\n").unwrap_or(request_str.len()) + "\r\n\r\n".len();
                                let body_str = &request_str[body_start..];
                                let json_body: serde_json::Value = serde_json::from_str(body_str).unwrap_or_default();
                                let id = json_body["id"].as_str().unwrap_or_default().to_string();
                                let balance = json_body["balance"].as_u64().unwrap_or_default() as u128;
                                // Build callback
                                fn callback(result: Result<()>, mut socket: mio::net::TcpStream) {
                                    match result {
                                        Ok(_) => {
                                            let response: String = format!("HTTP/1.1 200 OK\r\n\r\nAccount created");
                                            let _ = socket.write_all(response.as_bytes());
                                        }
                                        Err(err) => {
                                            let response = format!("HTTP/1.1 400 BAD REQUEST\r\n\r\n{}", err);
                                            let _ = socket.write_all(response.as_bytes());
                                        }
                                    }
                                }
                                // Convert the callback function to a closure
                                let closure: Box<dyn FnOnce(Result<(), anyhow::Error>) + Send + 'static> = Box::new(move |result| {            
                                    callback(result, socket);
                                });
                                // Add callback to queue
                                blockchain.create_account(id, balance, closure)


                            } else if request_str.starts_with(transfer_prefix) {
                                // Transaction: transfer ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
                                // Process request
                                let body_start = request_str.find("\r\n\r\n").unwrap_or(request_str.len()) + "\r\n\r\n".len();
                                let body_str = &request_str[body_start..];
                                let json_body: serde_json::Value = serde_json::from_str(body_str).unwrap_or_default();

                                let from = json_body["from"].as_str().unwrap_or_default().to_string();
                                let to = json_body["to"].as_str().unwrap_or_default().to_string();
                                let amount = json_body["amount"].as_u64().unwrap_or_default() as u128;

                                // Build callback
                                fn callback(result: Result<()>, mut socket: mio::net::TcpStream) {
                                    match result {
                                        Ok(_) => {
                                            let response = format!("HTTP/1.1 200 OK\r\n\r\nTransfer complete");
                                            let _ = socket.write_all(response.as_bytes());
                                        }
                                        Err(err) => {
                                            let response = format!("HTTP/1.1 400 BAD REQUEST\r\n\r\n{}", err);
                                            let _ = socket.write_all(response.as_bytes());
                                        }
                                    }
                                }
                                // Convert the callback function to a closure
                                let closure: Box<dyn FnOnce(Result<(), anyhow::Error>) + Send + 'static> = Box::new(move |result| {            
                                    callback(result, socket);
                                });
                                // Add callback to queue
                                blockchain.add_transfer(from, to, amount, closure);

                            } else {
                                let response = format!("HTTP/1.1 404 NOT FOUND\r\n\r\n");
                                let _ = socket.write_all(response.as_bytes());
                            }
                        }
                    }
                }
                _ => {}
            }
        }

        // Process blockchain every 2 seconds
        let elapsed = last_process_time.elapsed();
        if elapsed >= Duration::from_secs(1) {
            blockchain.mint();
            last_process_time = Instant::now();
        }
    }
}

