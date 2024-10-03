use reqwest;
use tokio;

#[tokio::main]
async fn main() {
    let response = reqwest::get("https://jsonplaceholder.typicode.com/posts/1")
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    println!("your request {}", response);
}
