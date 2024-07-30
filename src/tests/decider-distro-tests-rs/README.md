# Decider Tests

To run tests:
- Install IPFS CLI and run the IPFS daemon using command `ipfs daemon`.
- Upload tests resources to IPFS `./resources/upload.sh`.
- Set the path to your IPFS CLI binary `export IPFS_CLI_PATH=<path_to_ipfs_binary>`.
- Compile Decider and Worker Spell Aqua via `../../../build.sh`.
- To run tests, you may call `cargo nextest run --release`.
- To see logs, you need to call `cargo nextest run --release --nocapture`.


If you want to debug a specific test, you may enable logs using `enable_decider_logs()` function at the start of the test.
You also can modify `enable_decider_logs` (`tests/utils/mod.rs`) to enable more logs.
