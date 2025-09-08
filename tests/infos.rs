mod utils;

use crate::utils::perform_cmd_test;

#[test]
fn test_node() {
    perform_cmd_test("node", &["infos", "node"]);
}

#[test]
fn test_version() {
    perform_cmd_test("version", &["infos", "version"]);
}

#[test]
fn test_chain_params() {
    perform_cmd_test("chain_params", &["infos", "chain-params"]);
}

#[test]
fn test_self_clique() {
    perform_cmd_test("self_clique", &["infos", "self-clique"]);
}

#[test]
fn test_inter_clique_peer_info() {
    perform_cmd_test(
        "inter_clique_peer_info",
        &["infos", "inter-clique-peer-info"],
    );
}

#[test]
fn test_discovered_neighbors() {
    perform_cmd_test("discovered_neighbors", &["infos", "discovered-neighbors"]);
}

#[test]
fn test_misbehaviors() {
    perform_cmd_test("misbehaviors", &["infos", "misbehaviors"]);
}

#[test]
fn test_misbehaviors_ban_unban() {
    perform_cmd_test(
        "misbehaviors_ban_unban",
        &["infos", "misbehaviors-ban-unban", "Unban", "1.2.3.4"],
    );
}

#[test]
fn test_misbehaviors_ban_unban_no_params() {
    perform_cmd_test(
        "misbehaviors_ban_unban_no_params",
        &["infos", "misbehaviors-ban-unban"],
    );
}

#[test]
fn test_misbehaviors_ban_unban_bad_param() {
    perform_cmd_test(
        "misbehaviors_ban_unban_bad_param",
        &["infos", "misbehaviors-ban-unban", "UnknownType", "1.2.3.4"],
    );
}

#[test]
fn test_unreachable_brokers() {
    perform_cmd_test("unreachable_brokers", &["infos", "unreachable-brokers"]);
}

#[test]
fn test_discovery() {
    perform_cmd_test("discovery", &["infos", "discovery", "Reachable", "1.2.3.4"]);
}

#[test]
fn test_discovery_no_params() {
    perform_cmd_test("discovery_no_params", &["infos", "discovery"]);
}

#[test]
fn test_discovery_bad_param() {
    perform_cmd_test(
        "discovery_bad_param",
        &["infos", "discovery", "UnknownType", "1.2.3.4"],
    );
}

#[test]
fn test_history_hashrate() {
    perform_cmd_test(
        "history_hashrate",
        &["infos", "history-hashrate", "100", "200"],
    );
}

#[test]
fn test_history_hashrate_no_params() {
    perform_cmd_test("history_hashrate_no_params", &["infos", "history-hashrate"]);
}

#[test]
fn test_history_hashrate_bad_param() {
    perform_cmd_test(
        "history_hashrate_bad_param",
        &["infos", "history-hashrate", "foo"],
    );
}

#[test]
fn test_current_hashrate() {
    perform_cmd_test("current_hashrate", &["infos", "current-hashrate", "100"]);
}

#[test]
fn test_current_hashrate_no_params() {
    perform_cmd_test("current_hashrate_no_params", &["infos", "current-hashrate"]);
}

#[test]
fn test_current_hashrate_bad_param() {
    perform_cmd_test(
        "current_hashrate_bad_param",
        &["infos", "current-hashrate", "foo"],
    );
}

#[test]
fn test_current_difficulty() {
    perform_cmd_test("current_difficulty", &["infos", "current-difficulty"]);
}
