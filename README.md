## Crowbar A2S

An implementation of [Source A2S Queries](https://developer.valvesoftware.com/wiki/Server_queries)

Forked from <https://github.com/rumblefrog/a2s-rs>

**Note: Only supports Source engine and above, Goldsource is not supported**

## Usage

```rust
use std::time::Duration;
use crowbar_a2s::Builder;

fn main() {
    let client = Builder::new()
        .timeout(Duration::new(5, 0))
        .build_sync()
        .unwrap();

    let addr = std::env::var("CARGO_TEST_SRCDS_ADDR").unwrap();
    let info = client.info(&addr).unwrap();
    let players = client.players(&addr).unwrap();
    let result = (info, players);
    eprintln!("[log] result {:?}", &result);
}
```

```rust
use std::time::Duration;

#[tokio::main]
async fn main() {

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
```

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