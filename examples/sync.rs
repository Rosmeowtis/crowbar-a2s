/// test sync: cargo run --example sync --no-default-features --features sync
#[cfg(feature = "sync")]
fn main() {
    use std::time::Duration;

    let client = crowbar_a2s::Builder::new()
        .timeout(Duration::new(5, 0))
        .build_sync()
        .unwrap();

    let addr = std::env::var("CARGO_TEST_SRCDS_ADDR").unwrap();
    let info = client.info(&addr).unwrap();
    let players = client.players(&addr).unwrap();
    let result = (info, players);
    eprintln!("[log] result {:?}", &result);
}
