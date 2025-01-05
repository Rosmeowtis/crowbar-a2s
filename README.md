[![Crates.io Version](https://img.shields.io/crates/v/a2s.svg)](https://crates.io/crates/a2s/)
[![Documentation](https://docs.rs/a2s/badge.svg)](https://docs.rs/a2s/)

## Crowbar A2S

An implementation of [Source A2S Queries](https://developer.valvesoftware.com/wiki/Server_queries)

**Note: Only supports Source engine and above, Goldsource is not supported**

## Develop Note

Before run test, set the env `CARGO_TEST_SRCDS_ADDR` to which you want to query from, such as:

```sh
# bash
CARGO_TEST_SRCDS_ADDR=localhost:27015 cargo test

# nushell
with-env {CARGO_TEST_SRCDS_ADDR: "localhost:27015"} { cargo test }

# PowerShell
$env:CARGO_TEST_SRCDS_ADDR = "localhost:27015"
cargo test
```