use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use crate::model::Blockchain;

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
    println!("Server listening on: {}", addr);

    while let Ok((mut socket, _)) = listener.accept().await {

        // Handle each connection in a new task
        tokio::spawn(async move {
            println!("New connection from: {}", socket.peer_addr().unwrap());
            // Read the incoming data
            let mut buffer = [0; 1024];
            let request = socket.read(&mut buffer).await.unwrap();

            println!("Received {} bytes", request);
            
            // Process the request (you can implement your own logic here)
            let response = "HTTP/1.1 200 OK\r\n\r\nHello, World!";
            
            // Write the response back to the client
            let _ = socket.write_all(response.as_bytes()).await.unwrap();

        });
    }
}
