#![feature(async_closure)]
#![feature(try_blocks)]
#![feature(async_fn_in_trait)]

pub mod utils;

// Deploy a deal, assuming that there are no other deals installed
async fn deploy_deal(server: &mut ServerHandle, init_block: u32, deal_id: &str, block_number: u32) {
    let expected_reqs = 6;
    for _step in 0..expected_reqs {
        let (method, params) = server.receive_request().await.unwrap();
        let response = match method.as_str() {
            "eth_blockNumber" => json!(to_hex(init_block)),
            "eth_getLogs" => {
                let log = serde_json::from_value::<LogsReq>(params[0].clone()).unwrap();
                json!([TestApp::log_test_app2(
                    deal_id,
                    block_number,
                    log.topics[1].as_str()
                )])
            }
            "eth_sendRawTransaction" => {
                json!("0x55bfec4a4400ca0b09e075e2b517041cd78b10021c51726cb73bcba52213fa05")
            }
            "eth_getTransactionCount" => json!("0x1"),
            "eth_gasPrice" => json!("0x3b9aca07"),
            "eth_getTransactionReceipt" => default_receipt(),
            _ => panic!("mock http got an unexpected rpc method: {}", method),
        };
        server.send_response(Ok(json!(response)));
    }
}

/// Test plan:
/// - Use standard nox setup.
/// - On the first run, deploy a deal to create a worker. The deal is considered ACTIVE.
/// - On the second run,
#[tokio::test]
async fn test_activate() {
    const BLOCK_INIT: u32 = 33;
    const DEAL_ID: &'static str = DEAL_IDS[0];
    const BLOCK_NUMBER: u32 = 32;

    let mut server = run_test_server();
    let url = server.url.clone();

    let distro = make_distro_with_api(url);
    let (swarm, mut client) = setup_nox(distro.clone()).await;

    update_decider_script_for_tests(&mut client, swarm.tmp_dir.clone()).await;
    update_config(&mut client, &oneshot_config()).await.unwrap();
    deploy_deal(&mut server, BLOCK_INIT, DEAL_ID, BLOCK_NUMBER).await;
    wait_decider_stopped(&mut client).await;
}
