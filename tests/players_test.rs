#[cfg(feature = "sync")]
#[test]
fn test_players() {
    let client = crowbar_a2s::Builder::new().build_sync().unwrap();

    let result = client
        .players(&std::env::var("CARGO_TEST_SRCDS_ADDR").unwrap())
        .unwrap();

    println!("{:?}", result);
}
