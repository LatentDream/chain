use std::process;
use clap::{App, Arg};
use reqwest;

mod server;
mod model;

#[tokio::main]
async fn main() {
    let matches = App::new("b")
        .version("1.0")
        .author("Guillaume Thibault")
        .about("The B blockchain")
        .subcommand(
            App::new("serve")
                .about("Launch the server")
        )
        .subcommand(
            App::new("request")
                .about("Make a request to the server")
        )
        .get_matches();

    match matches.subcommand_name() {
        Some("serve") => {
            server::launch_blockchain().await;
        }
        Some("request") => {
            let url = "http://127.0.0.1:8080";  // Update with the actual server address
            
            // Make a GET request to the server
            match reqwest::get(url).await {
                Ok(response) => {
                    // Print the status code and body of the response
                    println!("Status: {}", response.status());
                    println!("Body:\n{}", response.text().await.unwrap());
                }
                Err(err) => {
                    eprintln!("Error making request: {}", err);
                    process::exit(1);
                }
            }
        }
        _ => {
            // No subcommand or an unrecognized subcommand
            println!("Please provide a valid subcommand. Use --help for more information.");
        }
    }
}
