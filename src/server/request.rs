use reqwest;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let url = "http://127.0.0.1:8080";  // Update with the actual server address

    // Make a GET request to the server
    let response = reqwest::get(url).await?;

    // Print the status code and body of the response
    println!("Status: {}", response.status());
    println!("Body:\n{}", response.text().await?);

    Ok(())
}
