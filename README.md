# chainx-sync-parse

The program is used to synchronize and parse sync-node data, 
providing 1:N external subscription service.

Follow the mainnet branch of ChainX.

## Usage

### 0. Requirement

- The latest Rust stable version.

### 1. Run the program

```bash
# compile
git clone https://github.com/chainx-org/chainx-sync-parse.git
cd chainx-sync-parse
cargo build --release

# run
# -h or --help for usage details
./target/release/chainx-sync-parse -h
```

### 2. Register/Deregister

Subscribe to the prefixes of needed runtime storage by register api.

The structure of Runtime storage is consistent with the [ChainX](https://github.com/chainx-org/ChainX) 
and [Substrate](https://github.com/chainpool/substrate).

**Register**:

Request: `POST 0.0.0.0:3030`

```
Headers:
Content-Type: application/json

Body: raw JSON (application/json)
{"jsonrpc":"2.0","id":1,"method":"register","params":[["XAssets AssetInfo", "XAssets AssetBalance"], "http://127.0.0.1:12345/write","1"]}
```

Parameter description:

- prefixes: 
    - type: JsonArray with JsonString
    - example: ["XAssets AssetInfo"], ["XAssets AssetInfo", "XAssets AssetBalance"]
- url: 
    - type: JsonString
    - example: "http://127.0.0.1:12345/write"
- version: 
    - type: JsonString
    - note: Semantic version (major.minor.patch), see [details](https://github.com/semver/semver)
    - example: "1.2.3"

You can run the example (a simple http server) to simulate the situation 
that registrant receives the block data successfully, before sending a register request.

```bash
cd chainx-sync-parse
cargo run --example register
# please run `cargo run --example register -- -h` to see the specific usage.
```

**Deregister**:

Request: `POST 0.0.0.0:3030`

```
Headers:
Content-Type: application/json

Body: raw JSON (application/json)
{"jsonrpc":"2.0","id":1,"method":"deregister","params":["http://127.0.0.1:12345/write"]}
```

Parameter description:

- url: 
    - type: JsonString
    - example: "http://127.0.0.1:12345/write"

### 3. Sync block

```bash
# compile
cd ChainX
git checkout mainnet
cargo build --release --features msgbus-log

# run
cp target/release/chainx .
# features msgbus-log, `sync.log` is the param specified by `chainx-sync-parse --sync-log=<PATH>`
nohup ./chainx --base-path <PATH> --name <NAME> --port <PORT> --pruning archive --rpc-port <PORT> --ws-port <PORT> --rpc-external --ws-external --no-grandpa &>> sync.log &
```

## Feature - Sync strategy

### sync-log (Enable by default, recommended)

0. **Requirement**: None

1. **Usage**:

    ```bash
    # compile
    git clone https://github.com/chainx-org/chainx-sync-parse.git
    cd chainx-sync-parse
    cargo build --release
    
    # run
    ./target/release/chainx-sync-parse -h
    # -h or --help for usage details
    chainx-sync-parse 1.1.0
    ChainX <https://chainx.org>
    Synchronize and parse ChainX sync data
    
    USAGE:
        chainx-sync-parse [OPTIONS]
    
    FLAGS:
        -h, --help       Prints help information
        -V, --version    Prints version information
    
    OPTIONS:
            --log-rotate-interval <SECOND>    Specify the sync log rotate interval, unit: SECOND [default: 30]
            --parse-log <PATH>                Specify the parse log file path [default: log/parse.log]
            --parse-roll-count <COUNT>        Specify the roll count of parse log [default: 5]
            --parse-roll-size <SIZE>          Specify the roll size of parse log, unit: MB [default: 200]
        -p, --port <PORT>                     Specify the port of register service [default: 3030]
            --start-height <HEIGHT>           Specify the starting block height to scan, range: [start,stop) [default: 0]
            --stop-height <HEIGHT>            Specify the stopping block height to scan [default: 18446744073709551615]
            --sync-log <PATH>                 Specify the sync log path [default: log/sync.log]
    ```
