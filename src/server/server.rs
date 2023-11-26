use std::time::Duration;
use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use crate::model::Blockchain;
use anyhow::Result;


pub async fn launch_blockchain() {

    // Create the blockchain
    let mut blockchain = Blockchain::new();
    
    // Bind the server's socket
    let addr = "127.0.0.1:8080";
    let listener = match TcpListener::bind(addr).await {
        Ok(listener) => listener,
        Err(err) => {
            eprintln!("Error binding to address {}: {}", addr, err);
            return;
        }
    };

    // List of prefixes for the different routes
    let balance_prefix = "GET /balance/";
    let create_account_prefix = "POST /account/";
    let transfer_prefix = "POST /transfer/";

    println!("Server listening on: {}", addr);

    loop {

        tokio::select! {
            _ = async {
                blockchain.mint();
                tokio::time::sleep(Duration::from_secs(1)).await;
            } => {}

            result = listener.accept() => {
                match result {
                    Ok((mut socket, _)) => {
                        // Handle the connection asynchronously
                        // tokio::spawn(handle_connection(socket));
                        println!("New connection from: {}", socket.peer_addr().unwrap());
                        let mut buffer = [0; 1024];
                        let request = match socket.read(&mut buffer).await {
                            Ok(request) => request,
                            Err(err) => {
                                eprintln!("Error reading from socket: {}", err);
                                let response = format!("HTTP/1.1 400 BAD REQUEST\r\n\r\n{}", err);
                                let _ = socket.write_all(response.as_bytes());
                                continue;
                            }
                        };
                        let request_str = String::from_utf8_lossy(&buffer[..request]);

                        if request_str.starts_with(balance_prefix) {
                            // Balance ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
                            let id_start: usize = balance_prefix.len();
                            let id_end = request_str.find(" HTTP/1.1").unwrap_or(request_str.len());
                            let id = request_str[id_start..id_end].to_string();
                            let balance = blockchain.get_balance(&id.to_string()); 
                            match balance {
                                Ok(balance) => {
                                    let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", balance);
                                    let _ = socket.write_all(response.as_bytes()).await.unwrap();
                                }
                                Err(err) => {
                                    let response = format!("HTTP/1.1 404 NOT FOUND\r\n\r\n{}", err);
                                    let _ = socket.write_all(response.as_bytes()).await.unwrap();
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
                            async fn callback(result: Result<()>, mut socket: TcpStream) {
                                match result {
                                    Ok(_) => {
                                        let response: String = format!("HTTP/1.1 200 OK\r\n\r\nAccount created");
                                        let _ = socket.write_all(response.as_bytes()).await.unwrap();
                                    }
                                    Err(err) => {
                                        let response = format!("HTTP/1.1 400 BAD REQUEST\r\n\r\n{}", err);
                                        let _ = socket.write_all(response.as_bytes()).await.unwrap();
                                    }
                                }
                            }
                            // Convert the callback function to a closure
                            let closure: Box<dyn FnOnce(Result<(), anyhow::Error>) + Send + 'static> = Box::new(move |result| {
                                tokio::spawn(async move {
                                    callback(result, socket).await;
                                });
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
                            async fn callback(result: Result<()>, mut socket: TcpStream) {
                                match result {
                                    Ok(_) => {
                                        let response = format!("HTTP/1.1 200 OK\r\n\r\nTransfer complete");
                                        let _ = socket.write_all(response.as_bytes()).await.unwrap();
                                    }
                                    Err(err) => {
                                        let response = format!("HTTP/1.1 400 BAD REQUEST\r\n\r\n{}", err);
                                        let _ = socket.write_all(response.as_bytes()).await.unwrap();
                                    }
                                }
                            }
                            // Convert the callback function to a closure
                            let closure: Box<dyn FnOnce(Result<(), anyhow::Error>) + Send + 'static> = Box::new(move |result| {
                                tokio::spawn(async move {
                                    callback(result, socket).await;
                                });
                            });
                            // Add callback to queue
                            blockchain.add_transfer(from, to, amount, closure);

                        } else {
                            let response = format!("HTTP/1.1 404 NOT FOUND\r\n\r\n");
                            let _ = socket.write_all(response.as_bytes()).await.unwrap();
                        }
                    }
                    Err(err) => {
                        eprintln!("Error accepting connection: {}", err);
                    }
                }
            }
        }

    }
    
}
