#![feature(async_closure)]
#![feature(try_blocks)]

use std::str::FromStr;

use futures::FutureExt;

use crate::utils::{enable_decider_logs, TestApp};
use crate::utils::chain::{ChainReplies, Deal, random_tx};
use crate::utils::control::run_decider;
use crate::utils::default::{DEAL_IDS, DEAL_STATUS_ACTIVE};
use crate::utils::setup::setup_nox;
use crate::utils::state::deal;
use crate::utils::state::worker;
use crate::utils::test_rpc_server::run_test_server;


/// Test Scenario: Activate/Deactivate a deal
///
/// 1. Install an inactive deal (NOT_ENOUGH_WORKERS)
///     - Check the status
/// 2. Update the deal status to active
///     - Check the status
/// 3. Update the deal status to inactive again
///     - Check the status
/// 4. Update the deal status to ENDED
///     - The deal should be removed
///     - NOTE: this event shouldn't happen because when the deal is ended it's not
///       in the list of deals anymore, BUT we support it just in case
#[tokio::test]
async fn test_deal_status() {}