use client_sdk::client::Client;
use common::protocol::ClientRole;
use std::time::Duration;
use tokio::time::sleep;
use broker_server::opts::Opts;
use broker_server::server;

const TEST_SERVER_ADDR: &str = "127.0.0.1";
const TEST_SERVER_PORT: u16 = 9082;

async fn start_test_server() {
    let opts = Opts {
        host: TEST_SERVER_ADDR.to_string(),
        port: TEST_SERVER_PORT,
        verbose: 1,
    };

    tokio::spawn(async move {
        let _ = server::server::run(&opts).await;
    });

    sleep(Duration::from_millis(200)).await;
}

#[tokio::test]
async fn test_handshake_from_client() {
    start_test_server().await;

    let addr = format!("{}:{}", TEST_SERVER_ADDR, TEST_SERVER_PORT);
    let result = Client::connect("test-client".into(), ClientRole::Producer, &addr).await;

    assert!(
        result.is_ok(),
        "Expected successful handshake, but got error: {:?}",
        result.err()
    );
}
