use client_sdk::connect_and_read;
use std::time::Duration;
use tokio::time::sleep;
use crate::opts::Opts;
use crate::server::server;

const TEST_SERVER_ADDR: &str = "127.0.0.1";
const TEST_SERVER_PORT: u16 = 9082;
async fn start_test_server() {
    let opts = Opts {
        host: TEST_SERVER_ADDR.to_string(),
        port: TEST_SERVER_PORT,
        verbose: 1,
    };

    tokio::spawn(async move {
        let _ = server::run(&opts).await;
    });

    // Give it a moment to bind and start
    sleep(Duration::from_millis(200)).await;
}

#[tokio::test]
async fn test_hello_from_server() {
    start_test_server().await;

    let addr = format!("{}:{}", TEST_SERVER_ADDR, TEST_SERVER_PORT);
    let msg = connect_and_read(&addr).await.unwrap();
    assert_eq!(msg.trim(), "Hello from server!");
}
