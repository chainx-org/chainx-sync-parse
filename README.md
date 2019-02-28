# chainx-sub-parse

Follow the stage/testnet-v0.9.5 branch of ChainX.

## Usage

### 0. Requirement

- Redis (that supports the **keyspace notification** feature)

    redis-server's conf:

    ```
    ####### EVENT NOTIFICATION ######
    notify-keyspace-events "Ez"
    ```

- Rust stable

### 1. Run the program

```bash
# compile
git clone https://github.com/chainpool/chainx-sub-parse.git
cd chainx-sub-parse
cargo build --release

# run
cp /target/release/chainx-sub-parse .
./start.sh
```

### 2. Register

Subscribe to the prefixes of needed runtime storage by registering api.

The structure of Runtime storage is consistent with the [ChainX - stage/testnet-v0.9.5](https://github.com/chainpool/ChainX/tree/stage/testnet-v0.9.5) and [substrate](https://github.com/chainpool/substrate).

**For example**:

Postman: `POST 0.0.0.0:3030`

```
Headers:
Content-Type: application/json

Body: raw JSON (application/json)
{"jsonrpc":"2.0","id":1,"method":"register","params":["System BlockHash", "http://127.0.0.1:12345/write","1"]}
```

You can run the example (a simple http server) to simulate the situation 
that registrant receives the block data successfully, 
before sending a register request through Postman.

```bash
cd chainx-sub-parse
cargo run --example register
# please run `cargo run --example register -- -h` to see the specific usage.
```

### 3. Sync block

```bash
# compile
cd ChainX
git checkout stage/testnet-v0.9.5
cargo build --release --features msgbus-redis

# run
cp target/release/chainx .
./sync-block.sh  # need to modify configuration manually.
```

## Feature/pgsql

Add the feature for inserting syncing block information into PostgreSQL.
 
See the [up.sql](migrations/2019-02-12-082211_create_blocks/up.sql) file for details of the database table `blocks`.

### 0. Requirement

- PostgreSQL, use your own PostgreSQL configuration in the [.env](./.env) file, like:
    ```bash
    DATABASE_URL=postgres://username:password@localhost/database_name
    ```

### 1. Usage

```bash
# compile
git clone https://github.com/chainpool/chainx-sub-parse.git
cd chainx-sub-parse
cargo build --release --features pgsql

# run
cp /target/release/chainx-sub-parse .
./start.sh
```
