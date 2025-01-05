/// test sync: cargo run --example info --no-default-features --features sync
/// test async: cargo run --example info --no-default-features --features async

#[cfg(all(not(feature = "async"), feature = "sync"))]
fn main() {
    use std::time::Duration;

    let client = crowbar_a2s::Builder::new()
        .timeout(Duration::new(5, 0))
        .build_sync()
        .unwrap();

    let result = client
        .info(&std::env::var("CARGO_TEST_SRCDS_ADDR").unwrap())
        .unwrap();

    println!("Sync: {:?}", result);
}

#[cfg(all(not(feature = "sync"), feature = "async"))]
#[tokio::main]
async fn main() {
    let client = crowbar_a2s::Builder::new()
        .timeout(Duration::new(5, 0))
        .build_async()
        .unwrap();

    let result = client
        .info(&std::env::var("CARGO_TEST_SRCDS_ADDR").unwrap())
        .await
        .unwrap();

    println!("Async: {:?}", result);
}
