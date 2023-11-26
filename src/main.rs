use clap::{App, Arg};

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
            App::new("balance")
                .about("Find the balance of an account")
                .arg(
                    Arg::with_name("account")
                        .help("ID of the account")
                        .takes_value(true)
                        .required(true),
                )
        )
        .subcommand(
            App::new("create-account")
                .about("Create a new account")
                .arg(
                    Arg::with_name("id-of-account")
                        .help("ID of the account")
                        .takes_value(true)
                        .required(true),
                )
                .arg(
                    Arg::with_name("starting-balance")
                        .help("Starting balance of the account")
                        .takes_value(true)
                        .required(true),
                ),
        )
        .subcommand(
            App::new("transfer")
                .about("Transfer tokens between accounts")
                .arg(
                    Arg::with_name("from-account")
                        .help("ID of the sender")
                        .takes_value(true)
                        .required(true),
                )
                .arg(
                    Arg::with_name("to-account")
                        .help("ID of the recipient")
                        .takes_value(true)
                        .required(true),
                )
                .arg(
                    Arg::with_name("amount")
                        .help("Amount to transfer")
                        .takes_value(true)
                        .required(true),
                ),
        )
        .get_matches();

        match matches.subcommand() {
            ("serve", _) => {
                    server::launch_blockchain().await;
                }
            ("balance", Some(sub_matches)) => {
                    let id_of_account = sub_matches.value_of("account").expect("Please provide an account id").to_string();
                    server::balance(id_of_account).await;
                }
            ("create-account", Some(sub_matches)) => {
                let id_of_account = sub_matches.value_of("id-of-account").expect("Please provide an account id").to_string();
                let starting_balance = sub_matches.value_of("starting-balance").expect("Please provide an initial balance").parse::<u128>().expect("Invalid starting balance");
                println!("Creating account with id {} and starting balance {}", id_of_account, starting_balance);
                server::create_account(id_of_account, starting_balance).await;
            }
            ("transfer", Some(sub_matches)) => {
                let from_account = sub_matches.value_of("from-account").expect("Please provide an account id").to_string();
                let to_account = sub_matches.value_of("to-account").expect("Please provide an account id").to_string();
                let amount = sub_matches.value_of("amount").expect("Please provide an amount").parse::<u128>().expect("Invalid amount");
                println!("Transferring {} from {} to {}", amount, from_account, to_account);
                server::transfer(from_account, to_account, amount).await;
            }
            _ => {
                println!("{}", matches.usage());
            }
    }
}
