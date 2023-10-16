use test_grpc::PTSL;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut _client = PTSL::Client::connect(
        String::from("http://[::1]:31416"),
        String::from("CuetesCuetos.com"),
        String::from("kikikiki.com"),
    )
    .await
    .unwrap();

    // client.request(command, payload)

    Ok(())
}
