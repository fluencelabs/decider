#[macro_export]
macro_rules! rpc_block_number {
    ($server:ident, $block_number:expr) => {{
        let (method, _params) = $server.receive_request().await.expect("rpc request");
        assert_eq!(method, "eth_blockNumber");
        $server.send_response(Ok(json!(to_hex($block_number))));
    }};
}

#[macro_export]
macro_rules! rpc_get_logs_empty {
    ($server:ident) => {{
        let (method, _params) = $server.receive_request().await.expect("rpc request");
        assert_eq!(method, "eth_getLogs");
        $server.send_response(Ok(json!([])));
    }};
}

#[macro_export]
macro_rules! rpc_get_logs_exact {
    ($server:ident, $logs:expr) => {{
        let (method, _params) = $server.receive_request().await.expect("rpc request");
        assert_eq!(method, "eth_getLogs");
        $server.send_response($logs);
    }};
}

#[macro_export]
macro_rules! rpc_deal_status {
    ($server:ident, $status:expr) => {{
        let (method, _params) = $server.receive_request().await.expect("rpc request");
        assert_eq!(method, "eth_call");
        $server.send_response(Ok(json!($status)));
    }};
}

#[macro_export]
macro_rules! rpc_deal_status_exact {
    ($server:ident, $status:expr) => {{
        let (method, _params) = $server.receive_request().await.expect("rpc request");
        assert_eq!(method, "eth_call");
        $server.send_response($status);
    }};
}
