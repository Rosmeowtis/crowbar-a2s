#[cfg(feature = "sync")]
#[test]
fn test_rules() {
    let client = crowbar_a2s::Builder::new().build_sync().unwrap();

    let result = client
        .rules(&std::env::var("CARGO_TEST_SRCDS_ADDR").unwrap())
        .unwrap();

    println!("{:?}", result);
}

#[cfg(feature = "sync")]
#[test]
fn test_rules_multipacket() {
    let client = crowbar_a2s::Builder::new().build_sync().unwrap();

    // need a server with multipacket
    let result = client.rules("74.91.118.209:27015").unwrap();

    println!("{:?}", result);
}

#[cfg(feature = "sync")]
#[test]
fn test_rules_multipacket2() {
    let client = crowbar_a2s::Builder::new().build_sync().unwrap();

    // need a server with multipacket
    let result = client.rules("188.165.244.220:27175").unwrap();

    println!("{:?}", result);
}
