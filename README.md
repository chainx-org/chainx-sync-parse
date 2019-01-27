# chainx-sub-parse

Follow the stage/test branch of ChainX.

## Usage

### 0. Requirement

- Redis (that supports the **keyspace notification** feature)

    redis-server's conf:

    ```
    ####### EVENT NOTIFICATION ######
    notify-keyspace-events "Ez"
    ```

- Latest version of Rust.

### 1. Run the program

```bash
git clone https://github.com/chainpool/chainx-sub-parse.git
cd chainx-sub-parse
cargo run
```

### 2. Register

Subscribe to the prefixes of needed runtime storage by registering api.

The structure of Runtime storage is consistent with the [ChainX stage/test](https://github.com/chainpool/ChainX/tree/stage/test) and [substrate](https://github.com/chainpool/substrate).

### 3. Sync block

```bash
cd ChainX
git checkout stage/test.
cargo run --features msgbus-redis -- --dev -d .sub --port 20001 --bootnodes=/ip4/127.0.0.1/tcp/20000/p2p/QmbQFPV5kfteEAFjWnaKpHh446AgPtaAY1cyyim3F5KV8i
```
