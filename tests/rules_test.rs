#[cfg(feature = "sync")]
#[test]
fn test_rules() {
    let client = crowbar_a2s::A2SClient::new().unwrap();

    let result = client
        .rules(&std::env::var("CARGO_TEST_SRCDS_ADDR").unwrap())
        .unwrap();

    println!("{:?}", result);
}

#[cfg(feature = "sync")]
#[test]
fn test_rules_multipacket() {
    let client = crowbar_a2s::A2SClient::new().unwrap();

    let result = client
        .rules(&std::env::var("CARGO_TEST_SRCDS_ADDR").unwrap())
        .unwrap();

    println!("{:?}", result);
}

#[cfg(feature = "sync")]
#[test]
fn test_rules_multipacket2() {
    let client = crowbar_a2s::A2SClient::new().unwrap();

    let result = client
        .rules(&std::env::var("CARGO_TEST_SRCDS_ADDR").unwrap())
        .unwrap();

    println!("{:?}", result);
}
