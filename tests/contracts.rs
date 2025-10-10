mod utils;

use serial_test::serial;

use crate::utils::{get_json_str_field_from_file, perform_cmd_dev, perform_cmd_test_dev};

const CONTRACT_FILTERS: [(&str, &str); 2] = [
    (
        r#""contractId":\s*"[A-Za-z0-9]+""#,
        r#""contractId": <contractID>"#,
    ),
    (r#""txId":\s*"[0-9a-f]+""#, r#""txId": <txId>"#),
];

// only return the result
const CONTRACT_CALL_FILTERS: [(&str, &str); 1] = [(r#"(?s)^.*("returns":\s*\[[^\]]*\]).*$"#, "$1")];
const CONTRACT_ASSERT_CALL_FILTER: [(&str, &str); 1] =
    [(r#"(?s)(?m).*?^\s*("type":\s*"CallContract.*)$.*"#, "$1\n")];

const TEST_CONFIG: &str = "./tests/contracts/test.config.yaml";

///////////
///
/// Compile
///
///////////

#[test]
fn test_compile_simple() {
    perform_cmd_test_dev(
        "compile",
        &[
            "contracts",
            "compile",
            "tests/contracts/test_dir/sub_contract.ral",
        ],
        None,
        None,
    );
}

#[test]
fn test_compile_folder() {
    perform_cmd_test_dev(
        "compile_folder",
        &["contracts", "compile", "tests/contracts/test_dir"],
        None,
        None,
    );
}

#[test]
fn test_compile_invalid_file() {
    perform_cmd_test_dev(
        "compile_invalid_file",
        &["contracts", "compile", "tests/contracts/impossible.ral"],
        None,
        None,
    );
}

#[test]
fn test_compile_invalid_import() {
    perform_cmd_test_dev(
        "compile_invalid_import",
        &[
            "contracts",
            "compile",
            "tests/fail_contracts/wrong_import.ral",
        ],
        None,
        None,
    );
}

#[test]
fn test_compile_no_import() {
    perform_cmd_test_dev(
        "compile_no_import",
        &[
            "contracts",
            "compile",
            "tests/fail_contracts/sub_contract_no_import.ral",
        ],
        None,
        None,
    );
}

// todo: tests on the contract types

///////////
///
/// Deploy
///
///////////

#[test]
#[serial]
fn test_deploy_contract() {
    let config = TEST_CONFIG.into();
    let path = perform_cmd_dev(
        "deploy_sub_contract",
        &[
            "contracts",
            "compile",
            "tests/contracts/test_dir/sub_contract.ral",
        ],
        config,
    );

    perform_cmd_test_dev(
        "deploy_sub_contract",
        &["contracts", "deploy", "Sub", &path],
        config,
        CONTRACT_FILTERS.to_vec().into(),
    );
}

#[test]
#[serial]
fn test_deploy_contract_debug() {
    let config = TEST_CONFIG.into();

    let path = perform_cmd_dev(
        "deploy_sub_contract_debug",
        &[
            "contracts",
            "compile",
            "tests/contracts/test_dir/token_faucet.ral",
        ],
        config,
    );

    perform_cmd_test_dev(
        "deploy_sub_contract_debug",
        &["contracts", "deploy", "TokenFaucet", &path],
        config,
        CONTRACT_FILTERS.to_vec().into(),
    );
}

#[test]
#[serial]
fn test_deploy_contract_debug_types() {
    let config = TEST_CONFIG.into();

    let path = perform_cmd_dev(
        "deploy_sub_contract_debug_types",
        &["contracts", "compile", "tests/contracts/all_types.ral"],
        config,
    );

    perform_cmd_test_dev(
        "deploy_sub_contract_debug_types",
        &["contracts", "deploy", "TestTypes", &path],
        config,
        CONTRACT_FILTERS.to_vec().into(),
    );
}

///////////
///
/// Call Contract
///
///////////

#[test]
#[serial]
fn test_call_contract_debug_no_args() {
    let config = TEST_CONFIG.into();

    let compile_path = perform_cmd_dev(
        "call_debug_types_no_args",
        &["contracts", "compile", "tests/contracts/all_types.ral"],
        config,
    );

    let deploy_path = perform_cmd_dev(
        "call_debug_types_no_args_deploy",
        &["contracts", "deploy", "TestTypes", &compile_path],
        config,
    );

    let contract_id = get_json_str_field_from_file(&deploy_path, "contractId");

    perform_cmd_test_dev(
        "call_debug_types_no_args",
        &[
            "contracts",
            "call",
            "TestTypes",
            &contract_id,
            &compile_path,
            "test",
        ],
        config,
        CONTRACT_ASSERT_CALL_FILTER.to_vec().into(),
    );
}

#[test]
#[serial]
fn test_call_contract_debug_arg() {
    let config = TEST_CONFIG.into();

    let compile_path = perform_cmd_dev(
        "call_debug_types_arg",
        &[
            "contracts",
            "compile",
            "tests/contracts/test_dir/sub_contract.ral",
        ],
        config,
    );

    let deploy_path = perform_cmd_dev(
        "call_debug_types_arg_deploy",
        &["contracts", "deploy", "Sub", &compile_path],
        config,
    );

    let contract_id = get_json_str_field_from_file(&deploy_path, "contractId");

    perform_cmd_test_dev(
        "call_debug_types_arg",
        &[
            "contracts",
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
    );
}

#[test]
#[serial]
fn test_call_contract_debug_all_args() {
    let config = TEST_CONFIG.into();

    let compile_path = perform_cmd_dev(
        "call_debug_types_all_args",
        &["contracts", "compile", "tests/contracts/all_types.ral"],
        config,
    );

    let deploy_path = perform_cmd_dev(
        "call_debug_types_all_args_deploy",
        &["contracts", "deploy", "TestTypes", &compile_path],
        config,
    );

    let contract_id = get_json_str_field_from_file(&deploy_path, "contractId");

    perform_cmd_test_dev(
        "call_debug_types_all_args",
        &[
            "contracts",
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
    );
}

#[test]
#[serial]
fn test_call_contract_debug_all_args_unordered() {
    let config = TEST_CONFIG.into();

    let compile_path = perform_cmd_dev(
        "call_debug_types_all_args_unordered",
        &["contracts", "compile", "tests/contracts/all_types.ral"],
        config,
    );

    let deploy_path = perform_cmd_dev(
        "call_debug_types_all_args_unordered_deploy",
        &["contracts", "deploy", "TestTypes", &compile_path],
        config,
    );

    let contract_id = get_json_str_field_from_file(&deploy_path, "contractId");

    perform_cmd_test_dev(
        "call_debug_types_all_args_unordered",
        &[
            "contracts",
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
    );
}
