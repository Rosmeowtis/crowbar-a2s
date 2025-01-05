#[cfg(feature = "sync")]
#[test]
fn test_info() {
    let client = crowbar_a2s::A2SClient::new().unwrap();

    let result = client
        .info(&std::env::var("CARGO_TEST_SRCDS_ADDR").unwrap())
        .unwrap();

    println!("{:?}", result);
}
