mod common;

use crate::common::{perform_cmd, setup_node};

#[tokio::test]
async fn test_node() {
    let (_container, url) = setup_node().await;

    perform_cmd_test!("node", &url, &["infos", "node"]);
}

#[tokio::test]
async fn test_version() {
    let (_container, url) = setup_node().await;

    perform_cmd_test!("version", &url, &["infos", "version"]);
}

#[tokio::test]
async fn test_chain_params() {
    let (_container, url) = setup_node().await;

    perform_cmd_test!("chain_params", &url, &["infos", "chain-params"]);
}

#[tokio::test]
async fn test_self_clique() {
    let (_container, url) = setup_node().await;

    perform_cmd_test!(
        "self_clique",
        &url,
        &["infos", "self-clique"],
        &[(
            r#""cliqueId":\s*"[0-9a-f]+""#,
            r#""cliqueId": "<CLIQUE_ID>""#,
        )]
    );
}

#[tokio::test]
async fn test_inter_clique_peer_info() {
    let (_container, url) = setup_node().await;

    perform_cmd_test!(
        "inter_clique_peer_info",
        &url,
        &["infos", "inter-clique-peer-info"]
    );
}

#[tokio::test]
async fn test_discovered_neighbors() {
    let (_container, url) = setup_node().await;

    perform_cmd_test!(
        "discovered_neighbors",
        &url,
        &["infos", "discovered-neighbors"]
    );
}

#[tokio::test]
async fn test_misbehaviors() {
    let (_container, url) = setup_node().await;

    perform_cmd(&url, &["infos", "misbehaviors-ban-unban", "Ban", "1.2.3.4"]).unwrap();
    perform_cmd_test!(
        "misbehaviors",
        &url,
        &["infos", "misbehaviors"],
        &[(r#""until":\s*[0-9]+"#, r#""until": "<UNTIL>""#)]
    );
}

mod misbehaviors_ban_unban {
    use crate::{common::setup_node, perform_cmd_test};

    #[tokio::test]
    async fn test_ban() {
        let (_container, url) = setup_node().await;

        perform_cmd_test!(
            "ban",
            &url,
            &["infos", "misbehaviors-ban-unban", "Ban", "1.2.3.4"]
        );
    }

    #[tokio::test]
    async fn test_unban() {
        let (_container, url) = setup_node().await;

        perform_cmd_test!(
            "unban",
            &url,
            &["infos", "misbehaviors-ban-unban", "Unban", "1.2.3.4"]
        );
    }

    #[tokio::test]
    async fn test_ban_several_peers() {
        let (_container, url) = setup_node().await;

        perform_cmd_test!(
            "ban_several_peers",
            &url,
            &[
                "infos",
                "misbehaviors-ban-unban",
                "Ban",
                "1.2.3.4",
                "5.6.7.8"
            ]
        );
    }

    #[tokio::test]
    async fn test_unban_several_peers() {
        let (_container, url) = setup_node().await;

        perform_cmd_test!(
            "unban_several_peers",
            &url,
            &[
                "infos",
                "misbehaviors-ban-unban",
                "Unban",
                "1.2.3.4",
                "5.6.7.8"
            ]
        );
    }

    #[tokio::test]
    async fn test_ban_no_peers() {
        let (_container, url) = setup_node().await;

        perform_cmd_test!(
            "ban_no_peers",
            &url,
            &["infos", "misbehaviors-ban-unban", "Ban"]
        );
    }

    #[tokio::test]
    async fn test_unban_no_peers() {
        let (_container, url) = setup_node().await;

        perform_cmd_test!(
            "unban_no_peers",
            &url,
            &["infos", "misbehaviors-ban-unban", "Unban"]
        );
    }

    #[test]
    fn test_no_params() {
        perform_cmd_test!("no_params", &["infos", "misbehaviors-ban-unban"]);
    }

    #[tokio::test]
    async fn test_bad_param() {
        let (_container, url) = setup_node().await;

        perform_cmd_test!(
            "bad_param",
            &url,
            &["infos", "misbehaviors-ban-unban", "UnknownType", "1.2.3.4"]
        );
    }
}

#[tokio::test]
async fn test_unreachable_brokers() {
    let (_container, url) = setup_node().await;

    perform_cmd(&url, &["infos", "discovery", "Unreachable", "1.2.3.4"]).unwrap();
    perform_cmd_test!(
        "unreachable_brokers",
        &url,
        &["infos", "unreachable-brokers"]
    );
}

mod discovery {
    use crate::{common::setup_node, perform_cmd_test};

    #[tokio::test]
    async fn test_reachable() {
        let (_container, url) = setup_node().await;

        perform_cmd_test!(
            "reachable",
            &url,
            &["infos", "discovery", "Reachable", "1.2.3.4"]
        );
    }

    #[tokio::test]
    async fn test_unreachable() {
        let (_container, url) = setup_node().await;

        perform_cmd_test!(
            "unreachable",
            &url,
            &["infos", "discovery", "Unreachable", "1.2.3.4"]
        );
    }

    #[tokio::test]
    async fn test_reachable_several_peers() {
        let (_container, url) = setup_node().await;

        perform_cmd_test!(
            "reachable_several_peers",
            &url,
            &["infos", "discovery", "Reachable", "1.2.3.4", "5.6.7.8"]
        );
    }

    #[tokio::test]
    async fn test_unreachable_several_peers() {
        let (_container, url) = setup_node().await;

        perform_cmd_test!(
            "unreachable_several_peers",
            &url,
            &["infos", "discovery", "Unreachable", "1.2.3.4", "5.6.7.8"]
        );
    }

    #[tokio::test]
    async fn test_reachable_no_peers() {
        let (_container, url) = setup_node().await;

        perform_cmd_test!(
            "reachable_no_peers",
            &url,
            &["infos", "discovery", "Reachable"]
        );
    }

    #[tokio::test]
    async fn test_unreachable_no_peers() {
        let (_container, url) = setup_node().await;

        perform_cmd_test!(
            "unreachable_no_peers",
            &url,
            &["infos", "discovery", "Unreachable"]
        );
    }

    #[test]
    fn test_no_params() {
        perform_cmd_test!("no_params", &["infos", "discovery"]);
    }

    #[tokio::test]
    async fn test_bad_param() {
        let (_container, url) = setup_node().await;

        perform_cmd_test!(
            "bad_param",
            &url,
            &["infos", "discovery", "UnknownType", "1.2.3.4"]
        );
    }
}

mod history_hashrate {
    use crate::{common::setup_node, perform_cmd_test};

    #[tokio::test]
    async fn test_one_param() {
        let (_container, url) = setup_node().await;

        perform_cmd_test!("one_param", &url, &["infos", "history-hashrate", "100"]);
    }

    #[tokio::test]
    async fn test_two_params() {
        let (_container, url) = setup_node().await;

        perform_cmd_test!(
            "two_params",
            &url,
            &["infos", "history-hashrate", "100", "200"]
        );
    }

    #[test]
    fn test_no_params() {
        perform_cmd_test!("no_params", &["infos", "history-hashrate"]);
    }

    #[test]
    fn test_bad_param() {
        perform_cmd_test!("bad_param", &["infos", "history-hashrate", "foo"]);
    }
}

mod current_hashrate {
    use crate::{common::setup_node, perform_cmd_test};

    #[tokio::test]
    async fn test_one_param() {
        let (_container, url) = setup_node().await;

        perform_cmd_test!("one_param", &url, &["infos", "current-hashrate", "100"]);
    }

    #[tokio::test]
    async fn test_no_params() {
        let (_container, url) = setup_node().await;

        perform_cmd_test!("no_params", &url, &["infos", "current-hashrate"]);
    }

    #[test]
    fn test_bad_param() {
        perform_cmd_test!("bad_param", &["infos", "current-hashrate", "foo"]);
    }
}

#[tokio::test]
async fn test_current_difficulty() {
    let (_container, url) = setup_node().await;

    perform_cmd_test!("current_difficulty", &url, &["infos", "current-difficulty"]);
}
