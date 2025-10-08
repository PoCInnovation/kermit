mod utils;

use crate::utils::{perform_cmd_dev, perform_cmd_test_dev};

const CONTRACT_FILTERS: [(&str, &str); 2] = [
    (
        r#""contractId":\s*"[A-Za-z0-9]+""#,
        r#""contractId": <contractID>"#,
    ),
    (r#""txId":\s*"[0-9a-f]+""#, r#""txId": <txId>"#),
];

///////////
///
/// Compile
///
///////////

#[test]
fn test_compile_simple() {
    perform_cmd_test_dev(
        "compile",
        &["contracts", "compile", "tests/contracts/sub_contract.ral"],
        None,
        None,
    );
}

#[test]
fn test_compile_folder() {
    perform_cmd_test_dev(
        "compile_folder",
        &["contracts", "compile", "tests/contracts"],
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
fn test_deploy_contract() {
    let config = Some("tests/contracts/test.config.yaml");
    let path = perform_cmd_dev(
        "deploy_sub_contract",
        &["contracts", "compile", "tests/contracts/sub_contract.ral"],
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
fn test_deploy_contract_debug() {
    let config = Some("tests/contracts/test.config.yaml");

    let path = perform_cmd_dev(
        "deploy_sub_contract_debug",
        &["contracts", "compile", "tests/contracts/token_faucet.ral"],
        config,
    );

    perform_cmd_test_dev(
        "deploy_sub_contract_debug",
        &["contracts", "deploy", "TokenFaucet", &path],
        config,
        CONTRACT_FILTERS.to_vec().into(),
    );
}
