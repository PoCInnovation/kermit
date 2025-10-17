mod common;

use crate::common::{perform_cmd_output, setup_node};

const CONTRACT_FILTERS: [(&str, &str); 4] = [
    (
        r#""contractId":\s*"[A-Za-z0-9]+""#,
        r#""contractId": <contractID>"#,
    ),
    (
        r#""address":\s*"[A-Za-z0-9]+""#,
        r#""address": <contractID>"#,
    ),
    (r#""txId":\s*"[0-9a-f]+""#, r#""txId": <txId>"#),
    (r#""id":\s*"[0-9a-f]+""#, r#""id": <id>"#),
];

const DEFAULT_PRIVATE_KEY: &str =
    "a642942e67258589cd2b1822c631506632db5a12aabcf413604e785300d762a5";

const TEST_CONFIG: &str = "./tests/contracts/test.config.yaml";

async fn perform_cmd_test_contract(
    name: &str,
    args: &[&str],
    config_name: Option<&str>,
    custom_filter: Option<Vec<(&str, &str)>>,
    url: Option<&str>,
) -> () {
    let _container; // keep the container up until oos

    let url = if let Some(str_url) = url {
        str_url.to_string()
    } else {
        let (container, url) = setup_node().await;
        _container = container;
        url
    };

    let config_path = config_name.unwrap_or("./alephium.config.yaml");

    let mut new_args = vec!["contracts", "-n", "dev", "-c", config_path];
    new_args.extend(args);

    let custom_filter = custom_filter.unwrap_or_else(|| vec![]);

    perform_cmd_test!(name, &url, &new_args, &custom_filter);
}

/* Run the command and write the outputed result (if success) inside a temporary file */
pub fn perform_cmd_dev(name: &str, args: &[&str], config_name: Option<&str>, url: &str) -> String {
    let config_path = config_name.unwrap_or("./alephium.config.yaml");

    let mut new_args = vec!["contracts", "-n", "dev", "-c", config_path];
    new_args.extend(args);

    perform_cmd_output(name, &new_args, url)
}

///////////
///
/// Compile
///
///////////

mod compile {
    use crate::perform_cmd_test_contract;

    #[tokio::test]
    async fn test_compile_simple() {
        perform_cmd_test_contract(
            "compile",
            &["compile", "tests/contracts/test_dir/sub_contract.ral"],
            None,
            None,
            None,
        )
        .await;
    }

    #[tokio::test]
    async fn test_compile_folder() {
        perform_cmd_test_contract(
            "compile_folder",
            &["compile", "tests/contracts/test_dir"],
            None,
            None,
            None,
        )
        .await;
    }

    #[tokio::test]
    async fn test_compile_invalid_file() {
        perform_cmd_test_contract(
            "compile_invalid_file",
            &["compile", "tests/contracts/impossible.ral"],
            None,
            None,
            None,
        )
        .await;
    }

    #[tokio::test]
    async fn test_compile_invalid_import() {
        perform_cmd_test_contract(
            "compile_invalid_import",
            &["compile", "tests/contracts/fail/wrong_import.ral"],
            None,
            None,
            None,
        )
        .await;
    }

    #[tokio::test]
    async fn test_compile_no_import() {
        perform_cmd_test_contract(
            "compile_no_import",
            &["compile", "tests/contracts/fail/sub_contract_no_import.ral"],
            None,
            None,
            None,
        )
        .await;
    }

    #[tokio::test]
    async fn test_compile_single_recurse_import() {
        perform_cmd_test_contract(
            "compile_single_recurse",
            &["compile", "tests/contracts/import_tests/single/1.ral"],
            None,
            None,
            None,
        )
        .await;
    }

    #[tokio::test]
    async fn test_compile_multiple_recurse_import() {
        perform_cmd_test_contract(
            "compile_multiple_recurse",
            &["compile", "tests/contracts/import_tests/multiple/1.ral"],
            None,
            None,
            None,
        )
        .await;
    }

    #[tokio::test]
    async fn test_compile_multiple_recurse_dir_import() {
        perform_cmd_test_contract(
            "compile_multiple_recurse_dir",
            &["compile", "tests/contracts/import_tests/multiple"],
            None,
            None,
            None,
        )
        .await;
    }
}

///////////
///
/// Deploy
///
///////////

mod deploy {
    use crate::{
        CONTRACT_FILTERS, DEFAULT_PRIVATE_KEY, TEST_CONFIG, common::setup_node, perform_cmd_dev,
        perform_cmd_test_contract,
    };

    #[tokio::test]
    async fn test_deploy_contract() {
        let config = TEST_CONFIG.into();
        let (_container, url) = setup_node().await;

        std::env::set_var("PRIVATE_KEY", DEFAULT_PRIVATE_KEY);

        let path = perform_cmd_dev(
            "deploy_sub_contract",
            &["compile", "tests/contracts/test_dir/sub_contract.ral"],
            config,
            &url,
        );

        perform_cmd_test_contract(
            "deploy_sub_contract",
            &["deploy", "Sub", &path],
            config,
            CONTRACT_FILTERS.to_vec().into(),
            Some(&url),
        )
        .await;
    }

    #[tokio::test]
    async fn test_deploy_contract_debug() {
        let config = TEST_CONFIG.into();
        let (_container, url) = setup_node().await;

        std::env::set_var("PRIVATE_KEY", DEFAULT_PRIVATE_KEY);

        let path = perform_cmd_dev(
            "deploy_sub_contract_debug",
            &["compile", "tests/contracts/test_dir/token_faucet.ral"],
            config,
            &url,
        );

        perform_cmd_test_contract(
            "deploy_sub_contract_debug",
            &["deploy", "TokenFaucet", &path],
            config,
            CONTRACT_FILTERS.to_vec().into(),
            Some(&url),
        )
        .await;
    }

    #[tokio::test]
    async fn test_deploy_contract_debug_types() {
        let config = TEST_CONFIG.into();
        let (_container, url) = setup_node().await;

        let path = perform_cmd_dev(
            "deploy_sub_contract_debug_types",
            &["compile", "tests/contracts/all_types.ral"],
            config,
            &url,
        );

        std::env::set_var("PRIVATE_KEY", DEFAULT_PRIVATE_KEY);

        perform_cmd_test_contract(
            "deploy_sub_contract_debug_types",
            &["deploy", "TestTypes", &path],
            config,
            CONTRACT_FILTERS.to_vec().into(),
            Some(&url),
        )
        .await;
    }
}

///////////
///
/// Call Contract
///
///////////

mod call {
    use crate::{
        DEFAULT_PRIVATE_KEY, TEST_CONFIG,
        common::{get_json_str_field_from_file, setup_node},
        perform_cmd_dev, perform_cmd_test_contract,
    };

    // only return the result
    const CONTRACT_CALL_FILTERS: [(&str, &str); 1] =
        [(r#"(?s)^.*("returns":\s*\[[^\]]*\]).*$"#, "$1")];
    const CONTRACT_ASSERT_CALL_FILTER: [(&str, &str); 1] =
        [(r#"(?s)(?m).*?^\s*("type":\s*"CallContract.*)$.*"#, "$1\n")];

    #[tokio::test]
    async fn test_call_contract_debug_no_args() {
        let config = TEST_CONFIG.into();
        let (_container, url) = setup_node().await;

        std::env::set_var("PRIVATE_KEY", DEFAULT_PRIVATE_KEY);

        let compile_path = perform_cmd_dev(
            "call_debug_types_no_args",
            &["compile", "tests/contracts/all_types.ral"],
            config,
            &url,
        );

        let deploy_path = perform_cmd_dev(
            "call_debug_types_no_args_deploy",
            &["deploy", "TestTypes", &compile_path],
            config,
            &url,
        );

        let contract_id = get_json_str_field_from_file(&deploy_path, "contractId");

        perform_cmd_test_contract(
            "call_debug_types_no_args",
            &["call", "TestTypes", &contract_id, &compile_path, "test"],
            config,
            CONTRACT_ASSERT_CALL_FILTER.to_vec().into(),
            Some(&url),
        )
        .await;
    }

    #[tokio::test]
    async fn test_call_contract_debug_arg() {
        let config = TEST_CONFIG.into();
        let (_container, url) = setup_node().await;

        std::env::set_var("PRIVATE_KEY", DEFAULT_PRIVATE_KEY);

        let compile_path = perform_cmd_dev(
            "call_debug_types_arg",
            &["compile", "tests/contracts/test_dir/sub_contract.ral"],
            config,
            &url,
        );

        let deploy_path = perform_cmd_dev(
            "call_debug_types_arg_deploy",
            &["deploy", "Sub", &compile_path],
            config,
            &url,
        );

        let contract_id = get_json_str_field_from_file(&deploy_path, "contractId");

        perform_cmd_test_contract(
            "call_debug_types_arg",
            &[
                "call",
                "Sub",
                &contract_id,
                &compile_path,
                "sub",
                "--args",
                "array=[2,1]",
            ],
            config,
            CONTRACT_CALL_FILTERS.to_vec().into(),
            Some(&url),
        )
        .await;
    }

    #[tokio::test]
    async fn test_call_contract_debug_all_args() {
        let config = TEST_CONFIG.into();
        let (_container, url) = setup_node().await;

        std::env::set_var("PRIVATE_KEY", DEFAULT_PRIVATE_KEY);

        let compile_path = perform_cmd_dev(
            "call_debug_types_all_args",
            &["compile", "tests/contracts/all_types.ral"],
            config,
            &url,
        );

        let deploy_path = perform_cmd_dev(
            "call_debug_types_all_args_deploy",
            &["deploy", "TestTypes", &compile_path],
            config,
            &url,
        );

        let contract_id = get_json_str_field_from_file(&deploy_path, "contractId");

        perform_cmd_test_contract(
            "call_debug_types_all_args",
            &[
                "call",
                "TestTypes",
                &contract_id,
                &compile_path,
                "test_args",
                "--args",
                "arg_vec=Coucou",
                "arg_addr=1DrDyTr9RpRsQnDnXo2YRiPzPW4ooHX5LLoqXrqfMrpQH",
                "arg_i256=-898",
                "arg_u256=78678576576",
                "arg_bool=false",
                "arg_array=[[0,1], [2,3]]",
                "arg_struct.counter=98",
                "arg_struct.nested_struct.val=10000",
            ],
            config,
            CONTRACT_ASSERT_CALL_FILTER.to_vec().into(),
            Some(&url),
        )
        .await;
    }

    #[tokio::test]
    async fn test_call_contract_debug_all_args_unordered() {
        let config = TEST_CONFIG.into();
        let (_container, url) = setup_node().await;

        std::env::set_var("PRIVATE_KEY", DEFAULT_PRIVATE_KEY);

        let compile_path = perform_cmd_dev(
            "call_debug_types_all_args_unordered",
            &["compile", "tests/contracts/all_types.ral"],
            config,
            &url,
        );

        let deploy_path = perform_cmd_dev(
            "call_debug_types_all_args_unordered_deploy",
            &["deploy", "TestTypes", &compile_path],
            config,
            &url,
        );

        let contract_id = get_json_str_field_from_file(&deploy_path, "contractId");

        perform_cmd_test_contract(
            "call_debug_types_all_args_unordered",
            &[
                "call",
                "TestTypes",
                &contract_id,
                &compile_path,
                "test_args",
                "--args",
                "arg_addr=1DrDyTr9RpRsQnDnXo2YRiPzPW4ooHX5LLoqXrqfMrpQH",
                "arg_array=[[0,1], [2,3]]",
                "arg_u256=78678576576",
                "arg_vec=Coucou",
                "arg_struct.nested_struct.val=10000",
                "arg_bool=false",
                "arg_i256=-898",
                "arg_struct.counter=98",
            ],
            config,
            CONTRACT_ASSERT_CALL_FILTER.to_vec().into(),
            Some(&url),
        )
        .await;
    }
}

///////////
///
/// Contract Infos
///
///////////

mod infos {
    use crate::{
        CONTRACT_FILTERS, DEFAULT_PRIVATE_KEY, TEST_CONFIG,
        common::{get_json, get_json_str_field_from_file, setup_node},
        perform_cmd_dev, perform_cmd_test_contract,
    };

    #[tokio::test]
    async fn test_state() {
        let config = TEST_CONFIG.into();
        let (_container, url) = setup_node().await;

        std::env::set_var("PRIVATE_KEY", DEFAULT_PRIVATE_KEY);

        let compile_path = perform_cmd_dev(
            "state",
            &["compile", "tests/contracts/test_dir/sub_contract.ral"],
            config,
            &url,
        );

        let deploy_path = perform_cmd_dev(
            "state_deploy",
            &["deploy", "Sub", &compile_path],
            config,
            &url,
        );

        let contract_id = get_json_str_field_from_file(&deploy_path, "contractId");

        perform_cmd_test_contract(
            "state",
            &["state", &contract_id],
            config,
            CONTRACT_FILTERS.to_vec().into(),
            Some(&url),
        )
        .await;
    }

    #[tokio::test]
    async fn test_code() {
        let config = TEST_CONFIG.into();
        let (_container, url) = setup_node().await;

        std::env::set_var("PRIVATE_KEY", DEFAULT_PRIVATE_KEY);

        let compile_path = perform_cmd_dev(
            "code",
            &["compile", "tests/contracts/test_dir/sub_contract.ral"],
            config,
            &url,
        );

        let _deploy_path = perform_cmd_dev(
            "code_deploy",
            &["deploy", "Sub", &compile_path],
            config,
            &url,
        );

        let data = get_json(&compile_path);

        let code = data["contracts"]
            .as_array()
            .unwrap()
            .get(0)
            .unwrap()
            .as_object()
            .unwrap()["codeHash"]
            .as_str()
            .unwrap();

        perform_cmd_test_contract(
            "code",
            &["code", code],
            config,
            CONTRACT_FILTERS.to_vec().into(),
            Some(&url),
        )
        .await;
    }
}
