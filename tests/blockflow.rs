mod common;

mod blocks {
    use crate::{common::setup_node, perform_cmd_test};

    #[tokio::test]
    async fn test_one_param() {
        let (_container, url) = setup_node().await;

        perform_cmd_test!("one_param", &url, &["blockflow", "blocks", "100"]);
    }

    #[tokio::test]
    async fn test_two_params() {
        let (_container, url) = setup_node().await;

        perform_cmd_test!("two_params", &url, &["blockflow", "blocks", "100", "200"]);
    }

    #[test]
    fn test_no_params() {
        perform_cmd_test!("no_params", &["blockflow", "blocks"]);
    }

    #[test]
    fn test_bad_param() {
        perform_cmd_test!("bad_param", &["blockflow", "blocks", "foo"]);
    }
}

mod blocks_with_events {
    use crate::{common::setup_node, perform_cmd_test};

    #[tokio::test]
    async fn test_one_param() {
        let (_container, url) = setup_node().await;

        perform_cmd_test!(
            "one_param",
            &url,
            &["blockflow", "blocks-with-events", "100"]
        );
    }

    #[tokio::test]
    async fn test_two_params() {
        let (_container, url) = setup_node().await;

        perform_cmd_test!(
            "two_params",
            &url,
            &["blockflow", "blocks-with-events", "100", "200"]
        );
    }

    #[test]
    fn test_no_params() {
        perform_cmd_test!("no_params", &["blockflow", "blocks-with-events"]);
    }

    #[test]
    fn test_bad_param() {
        perform_cmd_test!("bad_param", &["blockflow", "blocks-with-events", "foo"]);
    }
}

mod rich_blocks {
    use crate::{common::setup_node, perform_cmd_test};

    #[tokio::test]
    async fn test_one_param() {
        let (_container, url) = setup_node().await;

        perform_cmd_test!("one_param", &url, &["blockflow", "rich-blocks", "100"]);
    }

    #[tokio::test]
    async fn test_two_params() {
        let (_container, url) = setup_node().await;

        perform_cmd_test!(
            "two_params",
            &url,
            &["blockflow", "rich-blocks", "100", "200"]
        );
    }

    #[test]
    fn test_no_params() {
        perform_cmd_test!("no_params", &["blockflow", "rich-blocks"]);
    }

    #[test]
    fn test_bad_param() {
        perform_cmd_test!("bad_param", &["blockflow", "rich-blocks", "foo"]);
    }
}

mod block {
    use crate::{common::setup_node, perform_cmd_test};

    #[tokio::test]
    async fn test_one_param() {
        let (_container, url) = setup_node().await;

        perform_cmd_test!(
            "one_param",
            &url,
            &[
                "blockflow",
                "block",
                "f344b3dd45f39d62cfd200cfa3312c080018102908787c5565b8c8af3647368f",
            ]
        );
    }

    #[test]
    fn test_no_params() {
        perform_cmd_test!("no_params", &["blockflow", "block"]);
    }

    #[tokio::test]
    async fn test_bad_param() {
        let (_container, url) = setup_node().await;

        perform_cmd_test!("bad_param", &url, &["blockflow", "block", "foo"]);
    }
}

mod main_chain_block_by_ghost_uncle {
    use crate::{common::setup_node, perform_cmd_test};

    #[tokio::test]
    async fn test_not_exists() {
        let (_container, url) = setup_node().await;

        perform_cmd_test!(
            "not_exists",
            &url,
            &[
                "blockflow",
                "main-chain-block-by-ghost-uncle",
                "bdaf9dc514ce7d34b6474b8ca10a3dfb93ba997cb9d5ff1ea724ebe2af48abe5",
            ]
        );
    }

    #[test]
    fn test_no_params() {
        perform_cmd_test!(
            "no_params",
            &["blockflow", "main-chain-block-by-ghost-uncle"]
        );
    }

    #[tokio::test]
    async fn test_bad_param() {
        let (_container, url) = setup_node().await;

        perform_cmd_test!(
            "bad_param",
            &url,
            &["blockflow", "main-chain-block-by-ghost-uncle", "foo"]
        );
    }
}

mod block_with_events {
    use crate::{common::setup_node, perform_cmd_test};

    #[tokio::test]
    async fn test_one_param() {
        let (_container, url) = setup_node().await;

        perform_cmd_test!(
            "one_param",
            &url,
            &[
                "blockflow",
                "block-with-events",
                "f344b3dd45f39d62cfd200cfa3312c080018102908787c5565b8c8af3647368f",
            ]
        );
    }

    #[test]
    fn test_no_params() {
        perform_cmd_test!("no_params", &["blockflow", "block-with-events"]);
    }

    #[tokio::test]
    async fn test_bad_param() {
        let (_container, url) = setup_node().await;

        perform_cmd_test!(
            "bad_param",
            &url,
            &["blockflow", "block-with-events", "foo"]
        );
    }
}

mod rich_block {
    use crate::{common::setup_node, perform_cmd_test};

    #[tokio::test]
    async fn test_one_param() {
        let (_container, url) = setup_node().await;

        perform_cmd_test!(
            "one_param",
            &url,
            &[
                "blockflow",
                "rich-block",
                "f344b3dd45f39d62cfd200cfa3312c080018102908787c5565b8c8af3647368f",
            ]
        );
    }

    #[test]
    fn test_no_params() {
        perform_cmd_test!("no_params", &["blockflow", "rich-block"]);
    }

    #[tokio::test]
    async fn test_bad_param() {
        let (_container, url) = setup_node().await;

        perform_cmd_test!("bad_param", &url, &["blockflow", "rich-block", "foo"]);
    }
}

mod is_block_in_main_chain {
    use crate::{common::setup_node, perform_cmd_test};

    #[tokio::test]
    async fn test_one_param() {
        let (_container, url) = setup_node().await;

        perform_cmd_test!(
            "one_param",
            &url,
            &[
                "blockflow",
                "is-block-in-main-chain",
                "f344b3dd45f39d62cfd200cfa3312c080018102908787c5565b8c8af3647368f",
            ]
        );
    }

    #[test]
    fn test_no_params() {
        perform_cmd_test!("no_params", &["blockflow", "is-block-in-main-chain"]);
    }

    #[tokio::test]
    async fn test_bad_param() {
        let (_container, url) = setup_node().await;

        perform_cmd_test!(
            "bad_param",
            &url,
            &["blockflow", "is-block-in-main-chain", "foo"]
        );
    }
}

mod hashes {
    use crate::{common::setup_node, perform_cmd_test};

    #[test]
    fn test_one_param() {
        perform_cmd_test!("one_param", &["blockflow", "hashes", "1"]);
    }

    #[test]
    fn test_two_params() {
        perform_cmd_test!("two_params", &["blockflow", "hashes", "1", "1"]);
    }

    #[tokio::test]
    async fn test_three_params() {
        let (_container, url) = setup_node().await;

        perform_cmd_test!(
            "three_params",
            &url,
            &["blockflow", "hashes", "1", "1", "0"]
        );
    }

    #[test]
    fn test_no_params() {
        perform_cmd_test!("no_params", &["blockflow", "hashes"]);
    }

    #[test]
    fn test_bad_param() {
        perform_cmd_test!("bad_param", &["blockflow", "hashes", "foo", "1", "0"]);
    }
}

mod chain_info {
    use crate::{common::setup_node, perform_cmd_test};

    #[test]
    fn test_one_param() {
        perform_cmd_test!("one_param", &["blockflow", "chain-info", "1"]);
    }

    #[tokio::test]
    async fn test_two_params() {
        let (_container, url) = setup_node().await;

        perform_cmd_test!("two_params", &url, &["blockflow", "chain-info", "1", "1"]);
    }

    #[test]
    fn test_no_params() {
        perform_cmd_test!("no_params", &["blockflow", "chain-info"]);
    }

    #[test]
    fn test_bad_param() {
        perform_cmd_test!("bad_param", &["blockflow", "chain-info", "foo", "1"]);
    }
}

mod header {
    use crate::{common::setup_node, perform_cmd_test};

    #[tokio::test]
    async fn test_one_param() {
        let (_container, url) = setup_node().await;

        perform_cmd_test!(
            "one_param",
            &url,
            &[
                "blockflow",
                "header",
                "f344b3dd45f39d62cfd200cfa3312c080018102908787c5565b8c8af3647368f",
            ]
        );
    }

    #[test]
    fn test_no_params() {
        perform_cmd_test!("no_params", &["blockflow", "header"]);
    }

    #[tokio::test]
    async fn test_bad_param() {
        let (_container, url) = setup_node().await;

        perform_cmd_test!("bad_param", &url, &["blockflow", "header", "foo"]);
    }
}

mod raw_block {
    use crate::{common::setup_node, perform_cmd_test};

    #[tokio::test]
    async fn test_one_param() {
        let (_container, url) = setup_node().await;

        perform_cmd_test!(
            "one_param",
            &url,
            &[
                "blockflow",
                "raw-block",
                "f344b3dd45f39d62cfd200cfa3312c080018102908787c5565b8c8af3647368f",
            ]
        );
    }

    #[test]
    fn test_no_params() {
        perform_cmd_test!("no_params", &["blockflow", "raw-block"]);
    }

    #[tokio::test]
    async fn test_bad_param() {
        let (_container, url) = setup_node().await;

        perform_cmd_test!("bad_param", &url, &["blockflow", "raw-block", "foo"]);
    }
}
