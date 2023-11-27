use std::process;
use reqwest;

const B_CHAIN_URL: &str = "http://127.0.0.1:8000";

pub fn balance(id: String) {
    let url: String = format!("{}/balance/{}", B_CHAIN_URL, id); 
    match reqwest::blocking::Client::new()
        .get(url)
        .send()
    {
        Ok(response) => {
            println!("Status: {}", response.status());
            println!("Body: {}", response.text().unwrap());
        }
        Err(err) => {
            eprintln!("Error making request: {}", err);
            process::exit(1);
        }
    }
}

pub fn create_account(id: String, balance: u128) {
    let url: String = format!("{}/account/", B_CHAIN_URL); 

    let body = serde_json::json!({
        "id": id,
        "balance": balance,
    });

    match reqwest::blocking::Client::new()
        .post(url)
        .json(&body)
        .send()
    {
        Ok(response) => {
            println!("Status: {}", response.status());
            println!("Body: {}", response.text().unwrap());
        }
        Err(err) => {
            eprintln!("Error making request: {}", err);
            process::exit(1);
        }
    }
}

pub fn transfer(from_account: String, to_account: String, amount: u128) {
    let url: String = format!("{}/transfer/", B_CHAIN_URL); 
    let body = serde_json::json!({
        "from": from_account,
        "to": to_account,
        "amount": amount,
    });

    match reqwest::blocking::Client::new()
        .post(url)
        .json(&body)
        .send()
    {
        Ok(response) => {
            println!("Status: {}", response.status());
            println!("Body: {}", response.text().unwrap());
        }
        Err(err) => {
            eprintln!("Error making request: {}", err);
            process::exit(1);
        }
    }

}
