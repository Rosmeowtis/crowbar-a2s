/// test sync: cargo run --example info --no-default-features --features sync
/// test async: cargo run --example info --no-default-features --features async

#[cfg(all(not(feature = "async"), feature = "sync"))]
fn main() {
    let client = crowbar_a2s::A2SClient::new().unwrap();

    let result = client
        .info(&std::env::var("CARGO_TEST_SRCDS_ADDR").unwrap())
        .unwrap();

    println!("Sync: {:?}", result);
}

#[cfg(all(not(feature = "sync"), feature = "async"))]
#[tokio::main]
async fn main() {
    let client = crowbar_a2s::A2SClientAsync::new().await.unwrap();

    let result = client
        .info(&std::env::var("CARGO_TEST_SRCDS_ADDR").unwrap())
        .await
        .unwrap();

    println!("Async: {:?}", result);
}
