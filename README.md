# chainx-sync-parse

The program is used to synchronize and parse sync-node data, 
providing 1:N external subscription service.

Follow the stage/V0.9.9 branch of ChainX.

## Usage

### 0. Requirement

- The latest Rust stable version.

### 1. Run the program

```bash
# compile
git clone https://github.com/chainpool/chainx-sync-parse.git
cd chainx-sync-parse
cargo build --release

# run
cp /target/release/chainx-sync-parse .
./start.sh
```

### 2. Register/Deregister

Subscribe to the prefixes of needed runtime storage by register api.

The structure of Runtime storage is consistent with the [ChainX - stage/V0.9.9](https://github.com/chainpool/ChainX/tree/stage/V0.9.9) and [substrate](https://github.com/chainpool/substrate).

**Register**:

Postman: `POST 0.0.0.0:3030`

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
that registrant receives the block data successfully, 
before sending a register request through Postman.

```bash
cd chainx-sync-parse
cargo run --example register
# please run `cargo run --example register -- -h` to see the specific usage.
```

**Deregister**:

Postman: `POST 0.0.0.0:3030`

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
git checkout stage/V0.9.9
cargo build --release --features msgbus-log
# or cargo build --release --features msgbus-redis

# run
cp target/release/chainx .
./sync-block.sh  # need to modify configuration manually.
```

## Feature - Sync strategy

### sync-log (Enable Default)

0. **Requirement**: None

1. **Usage**:

    ```bash
    # compile
    git clone https://github.com/chainpool/chainx-sync-parse.git
    cd chainx-sync-parse
    cargo build --release
    
    # run
    cp /target/release/chainx-sync-parse .
    ./start.sh
    ```

### sync-redis (Alternative)

0. **Requirement**: Redis (which supports the **keyspace notification** feature)

    ```
    redis-server's conf:
    
    ####### EVENT NOTIFICATION ######
    notify-keyspace-events "Ez"
    ```

1. **Usage**:

    ```bash
    # compile
    git clone https://github.com/chainpool/chainx-sync-parse.git
    cd chainx-sync-parse
    cargo build --release --no-default-features --features='std,msgbus-redis'
    
    # run
    cp /target/release/chainx-sync-parse .
    ./start.sh
    ```

## Feature/pgsql (Optional)

Add the feature for inserting syncing block information into PostgreSQL.
 
See the [up.sql](migrations/2019-02-12-082211_create_blocks/up.sql) file for details of the database table `blocks`.

0. **Requirement**: PostgreSQL, use your own PostgreSQL configuration in the [.env](./.env) file, like:

    ```bash
    DATABASE_URL=postgres://username:password@localhost/database_name
    ```

1. **Usage**:

    ```bash
    # compile
    git clone https://github.com/chainpool/chainx-sync-parse.git
    cd chainx-sync-parse
    cargo build --release --features pgsql
    
    # run
    cp /target/release/chainx-sync-parse .
    ./start.sh
    ```
