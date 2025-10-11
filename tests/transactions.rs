mod common;

mod build {
    use crate::{common::setup_node, perform_cmd_test};

    #[test]
    fn test_one_param() {
        perform_cmd_test!(
            "one_param",
            &[
                "transactions",
                "build",
                "02ff46d897b529927bd1006cf75fb9e005d4f47181ec37db9dc82548218ff88106"
            ]
        );
    }

    #[test]
    fn test_two_params() {
        perform_cmd_test!(
            "two_params",
            &[
                "transactions",
                "build",
                "02ff46d897b529927bd1006cf75fb9e005d4f47181ec37db9dc82548218ff88106",
                "17cBiTcWhung3WDLuc9ja5Y7BMus5Q7CD9wYBxS1r1P2R",
            ]
        );
    }

    #[tokio::test]
    async fn test_three_params() {
        let (_container, url) = setup_node().await;

        perform_cmd_test!(
            "three_params",
            &url,
            &[
                "transactions",
                "build",
                "02ff46d897b529927bd1006cf75fb9e005d4f47181ec37db9dc82548218ff88106",
                "17cBiTcWhung3WDLuc9ja5Y7BMus5Q7CD9wYBxS1r1P2R",
                "1000000000000000000"
            ]
        );
    }

    #[tokio::test]
    async fn test_bad_public_key() {
        let (_container, url) = setup_node().await;

        perform_cmd_test!(
            "bad_public_key",
            &url,
            &[
                "transactions",
                "build",
                "15hLd3PdALjHe9RmFsjpRiMGYrCchZSjdyRufMjyfFSCB",
                "17cBiTcWhung3WDLuc9ja5Y7BMus5Q7CD9wYBxS1r1P2R",
                "1000000000000000000"
            ]
        );
    }

    #[tokio::test]
    async fn test_bad_address() {
        let (_container, url) = setup_node().await;

        perform_cmd_test!(
            "bad_address",
            &url,
            &[
                "transactions",
                "build",
                "02ff46d897b529927bd1006cf75fb9e005d4f47181ec37db9dc82548218ff88106",
                "17cBiTcWhung3WDLuc9ja5Y7BMus5Q7CD9wYBxS1r1P",
                "1000000000000000000"
            ]
        );
    }

    #[tokio::test]
    async fn test_too_small_value() {
        let (_container, url) = setup_node().await;

        perform_cmd_test!(
            "too_small_value",
            &url,
            &[
                "transactions",
                "build",
                "02ff46d897b529927bd1006cf75fb9e005d4f47181ec37db9dc82548218ff88106",
                "17cBiTcWhung3WDLuc9ja5Y7BMus5Q7CD9wYBxS1r1P2R",
                "100"
            ]
        );
    }

    #[test]
    fn test_no_params() {
        perform_cmd_test!("no_params", &["transactions", "build"]);
    }
}

mod submit {
    use serde_json::{Value, json};

    use crate::{
        common::{post, setup_node},
        perform_cmd_test,
    };

    #[test]
    fn test_one_param() {
        perform_cmd_test!(
            "one_param",
            &[
                "transactions",
                "submit",
                "a46f5fb14ae87d3d776d5a76823b238e0ddad43d8e6f6f8f40299ba0ec85e230"
            ]
        );
    }

    #[test]
    fn test_two_params() {
        perform_cmd_test!(
            "two_params",
            &[
                "transactions",
                "submit",
                "a46f5fb14ae87d3d776d5a76823b238e0ddad43d8e6f6f8f40299ba0ec85e230",
                "00040080004e20c1174876e80001ccb4a237b769377b8ebe2326104daf6ee211daada428ee5e21c50\
                c8b63be65c1072555980002ff46d897b529927bd1006cf75fb9e005d4f47181ec37db9dc82548218ff\
                8810602c40de0b6b3a764000000622990ad7be0a3d163562c10fd7985ef40a3e41857e7a1583406a78\
                5efc9273a00000000000000000000c6d3c1d647b478ec4b00000045c42a82aa01b2b7399c26ee6450d\
                44f04e017aa3c44af5e6b451f32aa458b1c00000000000000000000"
            ]
        );
    }

    #[tokio::test]
    async fn test_three_params() {
        let (_container, url) = setup_node().await;

        let result: Value = post(
            &url,
            "/transactions/build",
            json!({
                "fromPublicKey": "02ff46d897b529927bd1006cf75fb9e005d4f47181ec37db9dc82548218ff88106",
                "destinations": vec![json!({
                    "address": "17cBiTcWhung3WDLuc9ja5Y7BMus5Q7CD9wYBxS1r1P2R",
                    "attoAlphAmount": "1000000000000000000",
                })]
            }),
        )
        .await.unwrap().unwrap();

        perform_cmd_test!(
            "three_params",
            &url,
            &[
                "transactions",
                "submit",
                result["txId"].as_str().unwrap(),
                result["unsignedTx"].as_str().unwrap(),
                "--private-key",
                "609abeb3455aef1b832da331d168e13dd8f565f6c053e6f00cbd15e50e0d6b76"
            ]
        );
    }

    #[test]
    fn test_bad_tx_id() {
        perform_cmd_test!(
            "bad_tx_id",
            &[
                "transactions",
                "submit",
                "a46f5fb14ae87d3d776d5a76823b238e0ddad43d8e6f6f8f40299ba0ec85",
                "00040080004e20c1174876e80001ccb4a237975e960d767ce9b550bc481fe3a883d142bc4454539223a135756a371f5f67720002ff46d897b529927bd1006cf75fb9e005d4f47181ec37db9dc82548218ff8810602c40de0b6b3a764000000622990ad7be0a3d163562c10fd7985ef40a3e41857e7a1583406a785efc9273a00000000000000000000c6d3c20de6fb3cb00f00000045c42a82aa01b2b7399c26ee6450d44f04e017aa3c44af5e6b451f32aa458b1c00000000000000000000",
                "--private-key",
                "609abeb3455aef1b832da331d168e13dd8f565f6c053e6f00cbd15e50e0d6b76"
            ]
        );
    }

    #[tokio::test]
    async fn test_bad_unsigned_tx() {
        let (_container, url) = setup_node().await;

        perform_cmd_test!(
            "bad_unsigned_tx",
            &url,
            &[
                "transactions",
                "submit",
                "a46f5fb14ae87d3d776d5a76823b238e0ddad43d8e6f6f8f40299ba0ec85e230",
                "00040080004e20c1174876e80001ccb4a237b769377b8ebe2326104daf6ee211daada428ee5e21c50\
                c8b63be65c1072555980002ff46d897b529927bd1006cf75fb9e005d4f47181ec37db9dc82548218ff\
                8810602c40de0b6b3a764000000622990ad7be0a3d163562c10fd7985ef40a3e41857e7a1583406a78\
                5efc9273a00000000000000000000c6d3c1d647b478ec",
                "--private-key",
                "609abeb3455aef1b832da331d168e13dd8f565f6c053e6f00cbd15e50e0d6b76"
            ]
        );
    }

    #[test]
    fn test_bad_private_key() {
        perform_cmd_test!(
            "bad_private_key",
            &[
                "transactions",
                "submit",
                "a46f5fb14ae87d3d776d5a76823b238e0ddad43d8e6f6f8f40299ba0ec85e230",
                "00040080004e20c1174876e80001ccb4a237b769377b8ebe2326104daf6ee211daada428ee5e21c50\
                c8b63be65c1072555980002ff46d897b529927bd1006cf75fb9e005d4f47181ec37db9dc82548218ff\
                8810602c40de0b6b3a764000000622990ad7be0a3d163562c10fd7985ef40a3e41857e7a1583406a78\
                5efc9273a00000000000000000000c6d3c1d647b478e",
                "--private-key",
                "609abeb3455aef1b832da331d168e13dd8f565f6c053e6f00cbd15e50e0d"
            ]
        );
    }

    #[test]
    fn test_no_params() {
        perform_cmd_test!("no_params", &["transactions", "submit"]);
    }
}

mod create {
    use crate::{common::setup_node, perform_cmd_test};

    #[test]
    fn test_one_param() {
        perform_cmd_test!(
            "one_param",
            &[
                "transactions",
                "create",
                "02ff46d897b529927bd1006cf75fb9e005d4f47181ec37db9dc82548218ff88106"
            ]
        );
    }

    #[test]
    fn test_two_params() {
        perform_cmd_test!(
            "two_params",
            &[
                "transactions",
                "create",
                "02ff46d897b529927bd1006cf75fb9e005d4f47181ec37db9dc82548218ff88106",
                "17cBiTcWhung3WDLuc9ja5Y7BMus5Q7CD9wYBxS1r1P2R",
            ]
        );
    }

    #[test]
    fn test_three_params() {
        perform_cmd_test!(
            "three_params",
            &[
                "transactions",
                "create",
                "02ff46d897b529927bd1006cf75fb9e005d4f47181ec37db9dc82548218ff88106",
                "17cBiTcWhung3WDLuc9ja5Y7BMus5Q7CD9wYBxS1r1P2R",
                "1000000000000000000"
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
                "transactions",
                "create",
                "02ff46d897b529927bd1006cf75fb9e005d4f47181ec37db9dc82548218ff88106",
                "17cBiTcWhung3WDLuc9ja5Y7BMus5Q7CD9wYBxS1r1P2R",
                "1000000000000000000",
                "--private-key",
                "609abeb3455aef1b832da331d168e13dd8f565f6c053e6f00cbd15e50e0d6b76"
            ]
        );
    }

    #[tokio::test]
    async fn test_bad_public_key() {
        let (_container, url) = setup_node().await;

        perform_cmd_test!(
            "bad_public_key",
            &url,
            &[
                "transactions",
                "create",
                "15hLd3PdALjHe9RmFsjpRiMGYrCchZSjdyRufMjyfFSCB",
                "17cBiTcWhung3WDLuc9ja5Y7BMus5Q7CD9wYBxS1r1P2R",
                "1000000000000000000",
                "--private-key",
                "609abeb3455aef1b832da331d168e13dd8f565f6c053e6f00cbd15e50e0d6b76"
            ]
        );
    }

    #[tokio::test]
    async fn test_bad_address() {
        let (_container, url) = setup_node().await;

        perform_cmd_test!(
            "bad_address",
            &url,
            &[
                "transactions",
                "create",
                "02ff46d897b529927bd1006cf75fb9e005d4f47181ec37db9dc82548218ff88106",
                "17cBiTcWhung3WDLuc9ja5Y7BMus5Q7CD9wYBxS1r1P",
                "1000000000000000000",
                "--private-key",
                "609abeb3455aef1b832da331d168e13dd8f565f6c053e6f00cbd15e50e0d6b76"
            ]
        );
    }

    #[tokio::test]
    async fn test_too_small_value() {
        let (_container, url) = setup_node().await;

        perform_cmd_test!(
            "too_small_value",
            &url,
            &[
                "transactions",
                "create",
                "02ff46d897b529927bd1006cf75fb9e005d4f47181ec37db9dc82548218ff88106",
                "17cBiTcWhung3WDLuc9ja5Y7BMus5Q7CD9wYBxS1r1P2R",
                "100",
                "--private-key",
                "609abeb3455aef1b832da331d168e13dd8f565f6c053e6f00cbd15e50e0d6b76"
            ]
        );
    }

    #[tokio::test]
    async fn test_bad_private_key() {
        let (_container, url) = setup_node().await;

        perform_cmd_test!(
            "bad_private_key",
            &url,
            &[
                "transactions",
                "create",
                "02ff46d897b529927bd1006cf75fb9e005d4f47181ec37db9dc82548218ff88106",
                "17cBiTcWhung3WDLuc9ja5Y7BMus5Q7CD9wYBxS1r1P2R",
                "1000000000000000000",
                "--private-key",
                "609abeb3455aef1b832da331d168e13dd8f565f6c053e6f00cbd15e50e0d"
            ]
        );
    }

    #[test]
    fn test_no_params() {
        perform_cmd_test!("no_params", &["transactions", "create"]);
    }
}

mod details {
    use crate::{common::setup_node, perform_cmd_test};

    #[tokio::test]
    async fn test_exists() {
        let (_container, url) = setup_node().await;

        perform_cmd_test!(
            "exists",
            &url,
            &[
                "transactions",
                "details",
                "482784abee65d18a09f457b8125f045580c12bfc462318b3f98337a8c77d2c66"
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
                "transactions",
                "details",
                "b2dcb7c16a2846a857568547b775db5217f243102285b73a7c1852f4495e7a36"
            ]
        );
    }

    #[tokio::test]
    async fn test_exists_with_groups() {
        let (_container, url) = setup_node().await;

        perform_cmd_test!(
            "exists_with_groups",
            &url,
            &[
                "transactions",
                "details",
                "482784abee65d18a09f457b8125f045580c12bfc462318b3f98337a8c77d2c66",
                "--from-group",
                "3",
                "--to-group",
                "3"
            ]
        );
    }

    #[tokio::test]
    async fn test_not_exists_with_groups() {
        let (_container, url) = setup_node().await;

        perform_cmd_test!(
            "not_exists_with_groups",
            &url,
            &[
                "transactions",
                "details",
                "482784abee65d18a09f457b8125f045580c12bfc462318b3f98337a8c77d2c66",
                "--from-group",
                "1",
                "--to-group",
                "1"
            ]
        );
    }

    #[test]
    fn test_no_params() {
        perform_cmd_test!("no_params", &["transactions", "details"]);
    }
}

mod rich_details {
    use crate::{common::setup_node, perform_cmd_test};

    #[tokio::test]
    async fn test_exists() {
        let (_container, url) = setup_node().await;

        perform_cmd_test!(
            "exists",
            &url,
            &[
                "transactions",
                "rich-details",
                "482784abee65d18a09f457b8125f045580c12bfc462318b3f98337a8c77d2c66"
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
                "transactions",
                "rich-details",
                "b2dcb7c16a2846a857568547b775db5217f243102285b73a7c1852f4495e7a36"
            ]
        );
    }

    #[tokio::test]
    async fn test_exists_with_groups() {
        let (_container, url) = setup_node().await;

        perform_cmd_test!(
            "exists_with_groups",
            &url,
            &[
                "transactions",
                "rich-details",
                "482784abee65d18a09f457b8125f045580c12bfc462318b3f98337a8c77d2c66",
                "--from-group",
                "3",
                "--to-group",
                "3"
            ]
        );
    }

    #[tokio::test]
    async fn test_not_exists_with_groups() {
        let (_container, url) = setup_node().await;

        perform_cmd_test!(
            "not_exists_with_groups",
            &url,
            &[
                "transactions",
                "rich-details",
                "482784abee65d18a09f457b8125f045580c12bfc462318b3f98337a8c77d2c66",
                "--from-group",
                "1",
                "--to-group",
                "1"
            ]
        );
    }

    #[test]
    fn test_no_params() {
        perform_cmd_test!("no_params", &["transactions", "rich-details"]);
    }
}

mod raw {
    use crate::{common::setup_node, perform_cmd_test};

    #[tokio::test]
    async fn test_exists() {
        let (_container, url) = setup_node().await;

        perform_cmd_test!(
            "exists",
            &url,
            &[
                "transactions",
                "raw",
                "482784abee65d18a09f457b8125f045580c12bfc462318b3f98337a8c77d2c66"
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
                "transactions",
                "raw",
                "b2dcb7c16a2846a857568547b775db5217f243102285b73a7c1852f4495e7a36"
            ]
        );
    }

    #[tokio::test]
    async fn test_exists_with_groups() {
        let (_container, url) = setup_node().await;

        perform_cmd_test!(
            "exists_with_groups",
            &url,
            &[
                "transactions",
                "raw",
                "482784abee65d18a09f457b8125f045580c12bfc462318b3f98337a8c77d2c66",
                "--from-group",
                "3",
                "--to-group",
                "3"
            ]
        );
    }

    #[tokio::test]
    async fn test_not_exists_with_groups() {
        let (_container, url) = setup_node().await;

        perform_cmd_test!(
            "not_exists_with_groups",
            &url,
            &[
                "transactions",
                "raw",
                "482784abee65d18a09f457b8125f045580c12bfc462318b3f98337a8c77d2c66",
                "--from-group",
                "1",
                "--to-group",
                "1"
            ]
        );
    }

    #[test]
    fn test_no_params() {
        perform_cmd_test!("no_params", &["transactions", "raw"]);
    }
}

mod status {
    use crate::{common::setup_node, perform_cmd_test};

    #[tokio::test]
    async fn test_exists() {
        let (_container, url) = setup_node().await;

        perform_cmd_test!(
            "exists",
            &url,
            &[
                "transactions",
                "status",
                "482784abee65d18a09f457b8125f045580c12bfc462318b3f98337a8c77d2c66"
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
                "transactions",
                "status",
                "b2dcb7c16a2846a857568547b775db5217f243102285b73a7c1852f4495e7a36"
            ]
        );
    }

    #[tokio::test]
    async fn test_exists_with_groups() {
        let (_container, url) = setup_node().await;

        perform_cmd_test!(
            "exists_with_groups",
            &url,
            &[
                "transactions",
                "status",
                "482784abee65d18a09f457b8125f045580c12bfc462318b3f98337a8c77d2c66",
                "--from-group",
                "3",
                "--to-group",
                "3"
            ]
        );
    }

    #[tokio::test]
    async fn test_not_exists_with_groups() {
        let (_container, url) = setup_node().await;

        perform_cmd_test!(
            "not_exists_with_groups",
            &url,
            &[
                "transactions",
                "status",
                "482784abee65d18a09f457b8125f045580c12bfc462318b3f98337a8c77d2c66",
                "--from-group",
                "1",
                "--to-group",
                "1"
            ]
        );
    }

    #[test]
    fn test_no_params() {
        perform_cmd_test!("no_params", &["transactions", "status"]);
    }
}

mod tx_id_from_outputref {
    use crate::{common::setup_node, perform_cmd_test};

    #[tokio::test]
    async fn test_exists() {
        let (_container, url) = setup_node().await;

        perform_cmd_test!(
            "exists",
            &url,
            &[
                "transactions",
                "tx-id-from-outputref",
                "1022114513",
                "5d892f71e43b262733eaa99188a14f017fe006188f76cc0cc6ff8a0eb1ebab47"
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
                "transactions",
                "tx-id-from-outputref",
                "1022114513",
                "5d892f71e43b262733eaa99188a14f017fe006188f76cc0cc6ff8a0eb1ebab46"
            ]
        );
    }

    #[test]
    fn test_missing_param() {
        perform_cmd_test!(
            "missing_param",
            &["transactions", "tx-id-from-outputref", "1900212619"]
        );
    }

    #[test]
    fn test_no_params() {
        perform_cmd_test!("no_params", &["transactions", "tx-id-from-outputref"]);
    }

    #[test]
    fn test_bad_param() {
        perform_cmd_test!(
            "bad_param",
            &[
                "transactions",
                "tx-id-from-outputref",
                "foo",
                "91079afe4f5d56d09fab2fa60a9369dcf145eb982aece94bf81d00c537e03231"
            ]
        );
    }
}
