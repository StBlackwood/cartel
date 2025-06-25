
# Cartel

Cartel is a modular message broker written in Rust, inspired by Kafka. It provides a TCP-based client-server architecture with blazing fast message delivery.

## Getting Started

### 1. Start the Broker

```bash
cargo run -p broker-server
````

Optional with CLI args:

```bash
cargo run -p broker-server -- --host 127.0.0.1 --port 9082 --verbose 2
```

### 2. Run Tests

```bash
cargo test
```

## Dependencies

* `tokio` – async runtime
* `tracing` – structured logging
* `clap` – command-line argument parsing
* `serde` + `bincode` – message serialization

## Development Tasks

* Set up a basic request/response protocol over TCP
* Implement producer publishing and consumer fetch APIs
* Add in-memory topic and log storage
* Add persistent log to disk
* Add consumer offset tracking and acknowledgement
* Plan for multi-node support and replication

## License

This project is licensed under the MIT license.
