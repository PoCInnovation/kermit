mod common;

use crate::common::{perform_cmd, setup_node};

#[tokio::test]
async fn test_list() {
    let (_container, url) = setup_node().await;

    perform_cmd(&url, &["wallets", "create", "wallet", "1234"]).unwrap();
    perform_cmd_test!("list", &url, &["wallets", "list"]);
}

mod restore {
    use crate::{common::setup_node, perform_cmd_test};

    #[test]
    fn test_one_param() {
        perform_cmd_test!("one_param", &["wallets", "restore", "wallet"]);
    }

    #[test]
    fn test_two_params() {
        perform_cmd_test!("two_params", &["wallets", "restore", "wallet", "1234"]);
    }

    #[tokio::test]
    async fn test_three_params() {
        let (_container, url) = setup_node().await;

        perform_cmd_test!(
            "three_params",
            &url,
            &[
                "wallets",
                "restore",
                "wallet",
                "1234",
                "history smoke among improve cruise kidney brief jealous carbon office will \
                gallery amateur kit motion ethics attack dignity example huge thought depth \
                funny this"
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
                "wallets",
                "restore",
                "wallet",
                "1234",
                "history smoke among improve cruise kidney brief jealous carbon office will \
                gallery amateur kit motion ethics attack dignity example huge thought depth \
                funny this",
                "--is-miner",
            ]
        );
    }

    #[test]
    fn test_no_params() {
        perform_cmd_test!("no_params", &["wallets", "restore"]);
    }

    #[tokio::test]
    async fn test_bad_param() {
        let (_container, url) = setup_node().await;

        perform_cmd_test!(
            "bad_param",
            &url,
            &["wallets", "restore", "wallet", "1234", "foo"]
        );
    }
}

mod create {
    use crate::{
        common::{perform_cmd, setup_node},
        perform_cmd_test,
    };

    #[tokio::test]
    async fn test_create() {
        let (_container, url) = setup_node().await;

        perform_cmd_test!(
            "create",
            &url,
            &["wallets", "create", "wallet", "1234"],
            &[(
                r#"mnemonic":\s*"(?:[a-z]+(?: [a-z]+){23})"#,
                r#"mnemonic": "<MNEMONIC>"#
            )]
        );
    }

    #[tokio::test]
    async fn test_create_with_miner() {
        let (_container, url) = setup_node().await;

        perform_cmd_test!(
            "create_with_miner",
            &url,
            &["wallets", "create", "wallet", "1234", "--is-miner"],
            &[(
                r#"mnemonic":\s*"(?:[a-z]+(?: [a-z]+){23})"#,
                r#"mnemonic": "<MNEMONIC>"#
            )]
        );
    }

    #[tokio::test]
    async fn test_already_exists() {
        let (_container, url) = setup_node().await;

        perform_cmd(&url, &["wallets", "create", "wallet", "1234"]).unwrap();
        perform_cmd_test!(
            "already_exists",
            &url,
            &["wallets", "create", "wallet", "1234"]
        );
    }

    #[test]
    fn test_missing_param() {
        perform_cmd_test!("missing_param", &["wallets", "create", "wallet"]);
    }

    #[test]
    fn test_no_params() {
        perform_cmd_test!("no_params", &["wallets", "create"]);
    }
}

mod status {
    use crate::{
        common::{perform_cmd, setup_node},
        perform_cmd_test,
    };

    #[tokio::test]
    async fn test_exists() {
        let (_container, url) = setup_node().await;

        perform_cmd(&url, &["wallets", "create", "wallet", "1234"]).unwrap();
        perform_cmd_test!("exists", &url, &["wallets", "status", "wallet"]);
    }

    #[tokio::test]
    async fn test_not_exists() {
        let (_container, url) = setup_node().await;

        perform_cmd_test!("not_exists", &url, &["wallets", "status", "foo"]);
    }

    #[test]
    fn test_no_params() {
        perform_cmd_test!("no_params", &["wallets", "status"]);
    }
}

mod delete {
    use crate::{
        common::{perform_cmd, setup_node},
        perform_cmd_test,
    };

    #[tokio::test]
    async fn test_exists() {
        let (_container, url) = setup_node().await;

        perform_cmd(&url, &["wallets", "create", "wallet", "1234"]).unwrap();
        perform_cmd_test!("exists", &url, &["wallets", "delete", "wallet", "1234"]);
    }

    #[tokio::test]
    async fn test_not_exists() {
        let (_container, url) = setup_node().await;

        perform_cmd_test!("not_exists", &url, &["wallets", "delete", "foo", "1234"]);
    }

    #[test]
    fn test_missing_param() {
        perform_cmd_test!("missing_param", &["wallets", "delete", "wallet"]);
    }

    #[test]
    fn test_no_params() {
        perform_cmd_test!("no_params", &["wallets", "delete"]);
    }
}

mod lock {
    use crate::{
        common::{perform_cmd, setup_node},
        perform_cmd_test,
    };

    #[tokio::test]
    async fn test_one_param() {
        let (_container, url) = setup_node().await;

        perform_cmd(&url, &["wallets", "create", "wallet", "1234"]).unwrap();
        perform_cmd_test!("exists", &url, &["wallets", "lock", "wallet"]);
    }

    #[test]
    fn test_no_params() {
        perform_cmd_test!("no_params", &["wallets", "lock"]);
    }
}

mod unlock {
    use crate::{
        common::{perform_cmd, setup_node},
        perform_cmd_test,
    };

    #[tokio::test]
    async fn test_exists() {
        let (_container, url) = setup_node().await;

        perform_cmd(&url, &["wallets", "create", "wallet", "1234"]).unwrap();
        perform_cmd(&url, &["wallets", "lock", "wallet"]).unwrap();
        perform_cmd_test!("exists", &url, &["wallets", "unlock", "wallet", "1234"]);
    }

    #[tokio::test]
    async fn test_not_exists() {
        let (_container, url) = setup_node().await;

        perform_cmd_test!("not_exists", &url, &["wallets", "unlock", "foo", "1234"]);
    }

    #[test]
    fn test_missing_param() {
        perform_cmd_test!("missing_param", &["wallets", "unlock", "wallet"]);
    }

    #[test]
    fn test_no_params() {
        perform_cmd_test!("no_params", &["wallets", "unlock"]);
    }
}

mod balances {
    use crate::{
        common::{perform_cmd, setup_node},
        perform_cmd_test,
    };

    #[tokio::test]
    async fn test_exists() {
        let (_container, url) = setup_node().await;

        perform_cmd(
            &url,
            &[
                "wallets",
                "restore",
                "wallet",
                "1234",
                "history smoke among improve cruise kidney brief jealous carbon office will \
                gallery amateur kit motion ethics attack dignity example huge thought depth \
                funny this",
            ],
        )
        .unwrap();
        perform_cmd_test!("exists", &url, &["wallets", "balances", "wallet"]);
    }

    #[tokio::test]
    async fn test_not_exists() {
        let (_container, url) = setup_node().await;

        perform_cmd_test!("not_exists", &url, &["wallets", "balances", "foo"]);
    }

    #[test]
    fn test_no_params() {
        perform_cmd_test!("no_params", &["wallets", "balances"]);
    }
}

mod reveal_mnemonic {
    use crate::{
        common::{perform_cmd, setup_node},
        perform_cmd_test,
    };

    #[tokio::test]
    async fn test_exists() {
        let (_container, url) = setup_node().await;

        perform_cmd(
            &url,
            &[
                "wallets",
                "restore",
                "wallet",
                "1234",
                "history smoke among improve cruise kidney brief jealous carbon office will \
                gallery amateur kit motion ethics attack dignity example huge thought depth \
                funny this",
            ],
        )
        .unwrap();
        perform_cmd_test!(
            "exists",
            &url,
            &["wallets", "reveal-mnemonic", "wallet", "1234"]
        );
    }

    #[tokio::test]
    async fn test_not_exists() {
        let (_container, url) = setup_node().await;

        perform_cmd_test!(
            "not_exists",
            &url,
            &["wallets", "reveal-mnemonic", "foo", "1234"]
        );
    }

    #[test]
    fn test_missing_param() {
        perform_cmd_test!("missing_param", &["wallets", "reveal-mnemonic", "wallet"]);
    }

    #[test]
    fn test_no_params() {
        perform_cmd_test!("no_params", &["wallets", "reveal-mnemonic"]);
    }
}

mod transfer {
    use crate::{
        common::{perform_cmd, setup_node},
        perform_cmd_test,
    };

    #[tokio::test]
    async fn test_exists() {
        let (_container, url) = setup_node().await;

        perform_cmd(
            &url,
            &[
                "wallets",
                "restore",
                "wallet",
                "1234",
                "history smoke among improve cruise kidney brief jealous carbon office will \
                gallery amateur kit motion ethics attack dignity example huge thought depth \
                funny this",
            ],
        )
        .unwrap();
        perform_cmd(
            &url,
            &[
                "transactions",
                "create",
                "02ff46d897b529927bd1006cf75fb9e005d4f47181ec37db9dc82548218ff88106",
                "1F8hPMrEHKhKqpScWJjeJGiQSaBJE6VnqEBAHb5rVFfoP",
                "1000000000000000000",
                "--private-key",
                "609abeb3455aef1b832da331d168e13dd8f565f6c053e6f00cbd15e50e0d6b76",
            ],
        )
        .unwrap();
        perform_cmd_test!(
            "exists",
            &url,
            &[
                "wallets",
                "transfer",
                "wallet",
                "1DrDyTr9RpRsQnDnXo2YRiPzPW4ooHX5LLoqXrqfMrpQH",
                "500000000000000000"
            ]
        );
    }

    #[tokio::test]
    async fn test_not_exists() {
        let (_container, url) = setup_node().await;

        perform_cmd_test!(
            "not_exists",
            &url,
            &[
                "wallets",
                "transfer",
                "foo",
                "1DrDyTr9RpRsQnDnXo2YRiPzPW4ooHX5LLoqXrqfMrpQH",
                "1000000000000000000"
            ]
        );
    }

    #[tokio::test]
    async fn test_bad_address() {
        let (_container, url) = setup_node().await;

        perform_cmd(
            &url,
            &[
                "wallets",
                "restore",
                "wallet",
                "1234",
                "history smoke among improve cruise kidney brief jealous carbon office will \
                gallery amateur kit motion ethics attack dignity example huge thought depth \
                funny this",
            ],
        )
        .unwrap();
        perform_cmd(
            &url,
            &[
                "transactions",
                "create",
                "02ff46d897b529927bd1006cf75fb9e005d4f47181ec37db9dc82548218ff88106",
                "1F8hPMrEHKhKqpScWJjeJGiQSaBJE6VnqEBAHb5rVFfoP",
                "1000000000000000000",
                "--private-key",
                "609abeb3455aef1b832da331d168e13dd8f565f6c053e6f00cbd15e50e0d6b76",
            ],
        )
        .unwrap();
        perform_cmd_test!(
            "bad_address",
            &url,
            &[
                "wallets",
                "transfer",
                "wallet",
                "1DrDyTr9RpRsQnDnXo2YRiPzPW4ooHX5LLoqXrqfM",
                "1000000000000000000"
            ]
        );
    }

    #[tokio::test]
    async fn test_too_small_value() {
        let (_container, url) = setup_node().await;

        perform_cmd(
            &url,
            &[
                "wallets",
                "restore",
                "wallet",
                "1234",
                "history smoke among improve cruise kidney brief jealous carbon office will \
                gallery amateur kit motion ethics attack dignity example huge thought depth \
                funny this",
            ],
        )
        .unwrap();
        perform_cmd(
            &url,
            &[
                "transactions",
                "create",
                "02ff46d897b529927bd1006cf75fb9e005d4f47181ec37db9dc82548218ff88106",
                "1F8hPMrEHKhKqpScWJjeJGiQSaBJE6VnqEBAHb5rVFfoP",
                "1000000000000000000",
                "--private-key",
                "609abeb3455aef1b832da331d168e13dd8f565f6c053e6f00cbd15e50e0d6b76",
            ],
        )
        .unwrap();
        perform_cmd_test!(
            "too_small_value",
            &url,
            &[
                "wallets",
                "transfer",
                "wallet",
                "1DrDyTr9RpRsQnDnXo2YRiPzPW4ooHX5LLoqXrqfMrpQH",
                "100"
            ]
        );
    }

    #[test]
    fn test_missing_one_param() {
        perform_cmd_test!(
            "missing_one_param",
            &[
                "wallets",
                "transfer",
                "wallet",
                "1DrDyTr9RpRsQnDnXo2YRiPzPW4ooHX5LLoqXrqfMrpQH"
            ]
        );
    }

    #[test]
    fn test_missing_two_params() {
        perform_cmd_test!("missing_two_params", &["wallets", "transfer", "wallet"]);
    }

    #[test]
    fn test_no_params() {
        perform_cmd_test!("no_params", &["wallets", "transfer"]);
    }
}

mod sweep_active_address {
    use crate::{
        common::{perform_cmd, setup_node},
        perform_cmd_test,
    };

    #[tokio::test]
    async fn test_exists() {
        let (_container, url) = setup_node().await;

        perform_cmd(
            &url,
            &[
                "wallets",
                "restore",
                "wallet",
                "1234",
                "history smoke among improve cruise kidney brief jealous carbon office will \
                gallery amateur kit motion ethics attack dignity example huge thought depth \
                funny this",
            ],
        )
        .unwrap();
        perform_cmd(
            &url,
            &[
                "transactions",
                "create",
                "02ff46d897b529927bd1006cf75fb9e005d4f47181ec37db9dc82548218ff88106",
                "1F8hPMrEHKhKqpScWJjeJGiQSaBJE6VnqEBAHb5rVFfoP",
                "1000000000000000000",
                "--private-key",
                "609abeb3455aef1b832da331d168e13dd8f565f6c053e6f00cbd15e50e0d6b76",
            ],
        )
        .unwrap();
        perform_cmd_test!(
            "exists",
            &url,
            &[
                "wallets",
                "sweep-active-address",
                "wallet",
                "1DrDyTr9RpRsQnDnXo2YRiPzPW4ooHX5LLoqXrqfMrpQH"
            ]
        );
    }

    #[tokio::test]
    async fn test_not_exists() {
        let (_container, url) = setup_node().await;

        perform_cmd_test!(
            "not_exists",
            &url,
            &[
                "wallets",
                "sweep-active-address",
                "foo",
                "1DrDyTr9RpRsQnDnXo2YRiPzPW4ooHX5LLoqXrqfMrpQH"
            ]
        );
    }

    #[test]
    fn test_missing_param() {
        perform_cmd_test!(
            "missing_param",
            &["wallets", "sweep-active-address", "wallet"]
        );
    }

    #[test]
    fn test_no_params() {
        perform_cmd_test!("no_params", &["wallets", "sweep-active-address"]);
    }
}

mod sweep_all_addresses {
    use crate::{
        common::{perform_cmd, setup_node},
        perform_cmd_test,
    };

    #[tokio::test]
    async fn test_exists() {
        let (_container, url) = setup_node().await;

        perform_cmd(
            &url,
            &[
                "wallets",
                "restore",
                "wallet",
                "1234",
                "history smoke among improve cruise kidney brief jealous carbon office will \
                gallery amateur kit motion ethics attack dignity example huge thought depth \
                funny this",
            ],
        )
        .unwrap();
        perform_cmd(&url, &["wallets", "derive-next-address", "wallet"]).unwrap();
        perform_cmd(
            &url,
            &[
                "transactions",
                "create",
                "02ff46d897b529927bd1006cf75fb9e005d4f47181ec37db9dc82548218ff88106",
                "1F8hPMrEHKhKqpScWJjeJGiQSaBJE6VnqEBAHb5rVFfoP",
                "1000000000000000000",
                "--private-key",
                "609abeb3455aef1b832da331d168e13dd8f565f6c053e6f00cbd15e50e0d6b76",
            ],
        )
        .unwrap();
        perform_cmd(
            &url,
            &[
                "transactions",
                "create",
                "02ff46d897b529927bd1006cf75fb9e005d4f47181ec37db9dc82548218ff88106",
                "15LmMkBiK2QXyWfsbMQB5gSmCXzuJsdwoCBhmrNwDfrQ9",
                "1000000000000000000",
                "--private-key",
                "609abeb3455aef1b832da331d168e13dd8f565f6c053e6f00cbd15e50e0d6b76",
            ],
        )
        .unwrap();
        perform_cmd_test!(
            "exists",
            &url,
            &[
                "wallets",
                "sweep-all-addresses",
                "wallet",
                "1DrDyTr9RpRsQnDnXo2YRiPzPW4ooHX5LLoqXrqfMrpQH"
            ]
        );
    }

    #[tokio::test]
    async fn test_not_exists() {
        let (_container, url) = setup_node().await;

        perform_cmd_test!(
            "not_exists",
            &url,
            &[
                "wallets",
                "sweep-all-addresses",
                "foo",
                "1DrDyTr9RpRsQnDnXo2YRiPzPW4ooHX5LLoqXrqfMrpQH"
            ]
        );
    }

    #[test]
    fn test_missing_param() {
        perform_cmd_test!(
            "missing_param",
            &["wallets", "sweep-all-addresses", "wallet"]
        );
    }

    #[test]
    fn test_no_params() {
        perform_cmd_test!("no_params", &["wallets", "sweep-all-addresses"]);
    }
}

mod sign {
    use crate::{
        common::{perform_cmd, setup_node},
        perform_cmd_test,
    };

    #[tokio::test]
    async fn test_exists() {
        let (_container, url) = setup_node().await;

        perform_cmd(
            &url,
            &[
                "wallets",
                "restore",
                "wallet",
                "1234",
                "history smoke among improve cruise kidney brief jealous carbon office will \
                gallery amateur kit motion ethics attack dignity example huge thought depth \
                funny this",
            ],
        )
        .unwrap();
        perform_cmd_test!(
            "exists",
            &url,
            &[
                "wallets",
                "sign",
                "wallet",
                "bdaf9dc514ce7d34b6474b8ca10a3dfb93ba997cb9d5ff1ea724ebe2af48abe5"
            ]
        );
    }

    #[tokio::test]
    async fn test_not_exists() {
        let (_container, url) = setup_node().await;

        perform_cmd_test!(
            "not_exists",
            &url,
            &[
                "wallets",
                "sign",
                "foo",
                "bdaf9dc514ce7d34b6474b8ca10a3dfb93ba997cb9d5ff1ea724ebe2af48abe5"
            ]
        );
    }

    #[test]
    fn test_missing_param() {
        perform_cmd_test!("missing_param", &["wallets", "sign", "wallet"]);
    }

    #[test]
    fn test_no_params() {
        perform_cmd_test!("no_params", &["wallets", "sign"]);
    }
}

mod addresses {
    use crate::{
        common::{perform_cmd, setup_node},
        perform_cmd_test,
    };

    #[tokio::test]
    async fn test_exists() {
        let (_container, url) = setup_node().await;

        perform_cmd(
            &url,
            &[
                "wallets",
                "restore",
                "wallet",
                "1234",
                "history smoke among improve cruise kidney brief jealous carbon office will \
                gallery amateur kit motion ethics attack dignity example huge thought depth \
                funny this",
            ],
        )
        .unwrap();
        perform_cmd_test!("exists", &url, &["wallets", "addresses", "wallet"]);
    }

    #[tokio::test]
    async fn test_not_exists() {
        let (_container, url) = setup_node().await;

        perform_cmd_test!("not_exists", &url, &["wallets", "addresses", "foo"]);
    }

    #[test]
    fn test_no_params() {
        perform_cmd_test!("no_params", &["wallets", "addresses"]);
    }
}

mod address_info {
    use crate::{
        common::{perform_cmd, setup_node},
        perform_cmd_test,
    };

    #[tokio::test]
    async fn test_exists() {
        let (_container, url) = setup_node().await;

        perform_cmd(
            &url,
            &[
                "wallets",
                "restore",
                "wallet",
                "1234",
                "history smoke among improve cruise kidney brief jealous carbon office will \
                gallery amateur kit motion ethics attack dignity example huge thought depth \
                funny this",
            ],
        )
        .unwrap();
        perform_cmd_test!(
            "exists",
            &url,
            &[
                "wallets",
                "address-info",
                "wallet",
                "1F8hPMrEHKhKqpScWJjeJGiQSaBJE6VnqEBAHb5rVFfoP"
            ]
        );
    }

    #[tokio::test]
    async fn test_not_exists() {
        let (_container, url) = setup_node().await;

        perform_cmd_test!(
            "not_exists",
            &url,
            &[
                "wallets",
                "address-info",
                "foo",
                "1F8hPMrEHKhKqpScWJjeJGiQSaBJE6VnqEBAHb5rVFfoP"
            ]
        );
    }

    #[test]
    fn test_missing_param() {
        perform_cmd_test!("missing_param", &["wallets", "address-info", "wallet"]);
    }

    #[test]
    fn test_no_params() {
        perform_cmd_test!("no_params", &["wallets", "address-info"]);
    }
}

mod derive_next_address {
    use crate::{
        common::{perform_cmd, setup_node},
        perform_cmd_test,
    };

    #[tokio::test]
    async fn test_exists() {
        let (_container, url) = setup_node().await;

        perform_cmd(
            &url,
            &[
                "wallets",
                "restore",
                "wallet",
                "1234",
                "history smoke among improve cruise kidney brief jealous carbon office will \
                gallery amateur kit motion ethics attack dignity example huge thought depth \
                funny this",
            ],
        )
        .unwrap();
        perform_cmd_test!(
            "exists",
            &url,
            &["wallets", "derive-next-address", "wallet"]
        );
    }

    #[tokio::test]
    async fn test_not_exists() {
        let (_container, url) = setup_node().await;

        perform_cmd_test!(
            "not_exists",
            &url,
            &["wallets", "derive-next-address", "foo"]
        );
    }

    #[tokio::test]
    async fn test_exists_with_group() {
        let (_container, url) = setup_node().await;

        perform_cmd(
            &url,
            &[
                "wallets",
                "restore",
                "wallet",
                "1234",
                "history smoke among improve cruise kidney brief jealous carbon office will \
                gallery amateur kit motion ethics attack dignity example huge thought depth \
                funny this",
            ],
        )
        .unwrap();
        perform_cmd_test!(
            "exists_with_group",
            &url,
            &["wallets", "derive-next-address", "wallet", "--group", "2"]
        );
    }

    #[tokio::test]
    async fn test_not_exists_with_group() {
        let (_container, url) = setup_node().await;

        perform_cmd_test!(
            "not_exists_with_group",
            &url,
            &["wallets", "derive-next-address", "foo", "--group", "2"]
        );
    }

    #[test]
    fn test_no_params() {
        perform_cmd_test!("no_params", &["wallets", "derive-next-address"]);
    }
}

mod change_active_address {
    use crate::{
        common::{perform_cmd, setup_node},
        perform_cmd_test,
    };

    #[tokio::test]
    async fn test_exists() {
        let (_container, url) = setup_node().await;

        perform_cmd(
            &url,
            &[
                "wallets",
                "restore",
                "wallet",
                "1234",
                "history smoke among improve cruise kidney brief jealous carbon office will \
                gallery amateur kit motion ethics attack dignity example huge thought depth \
                funny this",
            ],
        )
        .unwrap();
        perform_cmd_test!(
            "exists",
            &url,
            &[
                "wallets",
                "change-active-address",
                "wallet",
                "1F8hPMrEHKhKqpScWJjeJGiQSaBJE6VnqEBAHb5rVFfoP"
            ]
        );
    }

    #[tokio::test]
    async fn test_not_exists() {
        let (_container, url) = setup_node().await;

        perform_cmd_test!(
            "not_exists",
            &url,
            &[
                "wallets",
                "change-active-address",
                "foo",
                "1F8hPMrEHKhKqpScWJjeJGiQSaBJE6VnqEBAHb5rVFfoP"
            ]
        );
    }

    #[test]
    fn test_missing_param() {
        perform_cmd_test!(
            "missing_param",
            &["wallets", "change-active-address", "wallet"]
        );
    }

    #[test]
    fn test_no_params() {
        perform_cmd_test!("no_params", &["wallets", "change-active-address"]);
    }
}

mod miner_addresses {
    use crate::{
        common::{perform_cmd, setup_node},
        perform_cmd_test,
    };

    #[tokio::test]
    async fn test_exists() {
        let (_container, url) = setup_node().await;

        perform_cmd(
            &url,
            &[
                "wallets",
                "restore",
                "wallet",
                "1234",
                "history smoke among improve cruise kidney brief jealous carbon office will \
                gallery amateur kit motion ethics attack dignity example huge thought depth \
                funny this",
                "--is-miner",
            ],
        )
        .unwrap();
        perform_cmd_test!("exists", &url, &["wallets", "miner-addresses", "wallet"]);
    }

    #[tokio::test]
    async fn test_not_exists() {
        let (_container, url) = setup_node().await;

        perform_cmd_test!("not_exists", &url, &["wallets", "miner-addresses", "foo"]);
    }

    #[test]
    fn test_no_params() {
        perform_cmd_test!("no_params", &["wallets", "miner-addresses"]);
    }
}

mod derive_next_miner_addresses {
    use crate::{
        common::{perform_cmd, setup_node},
        perform_cmd_test,
    };

    #[tokio::test]
    async fn test_exists() {
        let (_container, url) = setup_node().await;

        perform_cmd(
            &url,
            &[
                "wallets",
                "restore",
                "wallet",
                "1234",
                "history smoke among improve cruise kidney brief jealous carbon office will \
                gallery amateur kit motion ethics attack dignity example huge thought depth \
                funny this",
                "--is-miner",
            ],
        )
        .unwrap();
        perform_cmd_test!(
            "exists",
            &url,
            &["wallets", "derive-next-miner-addresses", "wallet"]
        );
    }

    #[tokio::test]
    async fn test_not_exists() {
        let (_container, url) = setup_node().await;

        perform_cmd_test!(
            "not_exists",
            &url,
            &["wallets", "derive-next-miner-addresses", "foo"]
        );
    }

    #[test]
    fn test_no_params() {
        perform_cmd_test!("no_params", &["wallets", "derive-next-miner-addresses"]);
    }
}
