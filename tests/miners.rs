mod common;

use crate::common::setup_node;

mod cpu_mining {
    use crate::{common::setup_node, perform_cmd_test};

    #[tokio::test]
    async fn test_start_mining() {
        let (_container, url) = setup_node().await;

        perform_cmd_test!(
            "start_mining",
            &url,
            &["miners", "cpu-mining", "start-mining"]
        );
    }

    #[tokio::test]
    async fn test_stop_mining() {
        let (_container, url) = setup_node().await;

        perform_cmd_test!(
            "stop_mining",
            &url,
            &["miners", "cpu-mining", "stop-mining"]
        );
    }

    #[test]
    fn test_no_params() {
        perform_cmd_test!("no_params", &["miners", "cpu-mining"]);
    }

    #[tokio::test]
    async fn test_bad_param() {
        let (_container, url) = setup_node().await;

        perform_cmd_test!("bad_param", &url, &["miners", "cpu-mining", "foo"]);
    }
}

mod mine_one_block {
    use crate::{common::setup_node, perform_cmd_test};

    #[tokio::test]
    async fn test_one_param() {
        perform_cmd_test!("one_param", &["miners", "mine-one-block", "1"]);
    }

    #[tokio::test]
    async fn test_two_params() {
        let (_container, url) = setup_node().await;

        perform_cmd_test!("two_params", &url, &["miners", "mine-one-block", "1", "1"]);
    }

    #[test]
    fn test_no_params() {
        perform_cmd_test!("no_params", &["miners", "mine-one-block"]);
    }

    #[test]
    fn test_bad_param() {
        perform_cmd_test!("bad_param", &["miners", "mine-one-block", "foo", "1"]);
    }
}

#[tokio::test]
async fn test_addresses() {
    let (_container, url) = setup_node().await;

    perform_cmd_test!("addresses", &url, &["miners", "addresses"]);
}

mod update_addresses {
    use crate::{common::setup_node, perform_cmd_test};

    #[tokio::test]
    async fn test_one_param() {
        let (_container, url) = setup_node().await;

        perform_cmd_test!(
            "one_param",
            &url,
            &[
                "miners",
                "update-addresses",
                "1FsroWmeJPBhcPiUr37pWXdojRBe6jdey9uukEXk1TheA"
            ]
        );
    }

    #[tokio::test]
    async fn test_four_params() {
        let (_container, url) = setup_node().await;

        perform_cmd_test!(
            "four_params",
            &url,
            &[
                "miners",
                "update-addresses",
                "1FsroWmeJPBhcPiUr37pWXdojRBe6jdey9uukEXk1TheA",
                "1CQvSXsmM5BMFKguKDPpNUfw1idiut8UifLtT8748JdHc",
                "193maApeJWrz9GFwWCfa982ccLARVE9Y1WgKSJaUs7UAx",
                "16fZKYPCZJv2TP3FArA9FLUQceTS9U8xVnSjxFG9MBKyY"
            ]
        );
    }

    #[tokio::test]
    async fn test_no_params() {
        let (_container, url) = setup_node().await;

        perform_cmd_test!("no_params", &url, &["miners", "update-addresses"]);
    }

    #[tokio::test]
    async fn test_bad_param() {
        let (_container, url) = setup_node().await;

        perform_cmd_test!("bad_param", &url, &["miners", "update-addresses", "foo"]);
    }
}
