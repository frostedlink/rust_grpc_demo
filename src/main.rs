#[path = "../proto/chat.rs"]
mod chat;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client =
        chat::chat_service_client::ChatServiceClient::connect("http://localhost:3000").await.unwrap();

    let request = tonic::Request::new(chat::Message {
        body: String::from("Hello from client!"),
    });

    let response = client.send_message(request).await?;

    println!("RESPONSE={:?}", response.into_inner().body);
    Ok(())
}
