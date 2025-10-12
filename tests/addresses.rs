mod common;

mod balance {
    use crate::{common::setup_node, perform_cmd_test};

    #[tokio::test]
    async fn test_one_param() {
        let (_container, url) = setup_node().await;

        perform_cmd_test!(
            "one_param",
            &url,
            &[
                "addresses",
                "balance",
                "1DrDyTr9RpRsQnDnXo2YRiPzPW4ooHX5LLoqXrqfMrpQH"
            ]
        );
    }

    #[tokio::test]
    async fn test_two_params() {
        let (_container, url) = setup_node().await;

        perform_cmd_test!(
            "two_params",
            &url,
            &[
                "addresses",
                "balance",
                "1DrDyTr9RpRsQnDnXo2YRiPzPW4ooHX5LLoqXrqfMrpQH",
                "--mem-pool"
            ]
        );
    }

    #[test]
    fn test_no_params() {
        perform_cmd_test!("no_params", &["addresses", "balance"]);
    }

    #[tokio::test]
    async fn test_bad_param() {
        let (_container, url) = setup_node().await;

        perform_cmd_test!("bad_param", &url, &["addresses", "balance", "foo"]);
    }
}

mod utxos {
    use crate::{common::setup_node, perform_cmd_test};

    #[tokio::test]
    async fn test_one_param() {
        let (_container, url) = setup_node().await;

        perform_cmd_test!(
            "one_param",
            &url,
            &[
                "addresses",
                "utxos",
                "1DrDyTr9RpRsQnDnXo2YRiPzPW4ooHX5LLoqXrqfMrpQH"
            ]
        );
    }

    #[tokio::test]
    async fn test_two_params() {
        let (_container, url) = setup_node().await;

        perform_cmd_test!(
            "two_params",
            &url,
            &[
                "addresses",
                "utxos",
                "1DrDyTr9RpRsQnDnXo2YRiPzPW4ooHX5LLoqXrqfMrpQH",
                "--error-if-exceed-max-utxos"
            ]
        );
    }

    #[test]
    fn test_no_params() {
        perform_cmd_test!("no_params", &["addresses", "utxos"]);
    }

    #[tokio::test]
    async fn test_bad_param() {
        let (_container, url) = setup_node().await;

        perform_cmd_test!("bad_param", &url, &["addresses", "utxos", "foo"]);
    }
}

mod group {
    use crate::{common::setup_node, perform_cmd_test};

    #[tokio::test]
    async fn test_one_param() {
        let (_container, url) = setup_node().await;

        perform_cmd_test!(
            "one_param",
            &url,
            &[
                "addresses",
                "group",
                "1DrDyTr9RpRsQnDnXo2YRiPzPW4ooHX5LLoqXrqfMrpQH"
            ]
        );
    }

    #[test]
    fn test_no_params() {
        perform_cmd_test!("no_params", &["addresses", "group"]);
    }

    #[tokio::test]
    async fn test_bad_param() {
        let (_container, url) = setup_node().await;

        perform_cmd_test!("bad_param", &url, &["addresses", "group", "foo"]);
    }
}
