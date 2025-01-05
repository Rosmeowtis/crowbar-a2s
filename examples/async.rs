/// test async: cargo run --example async --no-default-features --features async

#[cfg(feature = "async")]
#[tokio::main]
async fn main() {
    use std::time::Duration;

    let client = crowbar_a2s::Builder::new()
        .timeout(Duration::new(5, 0))
        .build_async()
        .unwrap();
    let addr = std::env::var("CARGO_TEST_SRCDS_ADDR").unwrap();
    let info = client.info(&addr);
    let players = client.players(&addr);
    let result = tokio::try_join!(info, players).unwrap();
    eprintln!("[log] result {:?}", &result);
}
