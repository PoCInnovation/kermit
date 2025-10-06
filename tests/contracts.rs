mod utils;

use crate::utils::perform_cmd_test_dev;

#[test]
fn test_compile_simple() {
    perform_cmd_test_dev(
        "compile",
        &["contracts", "compile", "tests/contracts/sub_contract.ral"],
    );
}

#[test]
fn test_compile_folder() {
    perform_cmd_test_dev("compile_folder", &["contracts", "compile", "tests/contracts"]);
}

#[test]
fn test_compile_invalid_file() {
    perform_cmd_test_dev(
        "compile_invalid_file",
        &["contracts", "compile", "tests/contracts/impossible.ral"],
    );
}

#[test]
fn test_compile_invalid_import() {
    perform_cmd_test_dev(
        "compile_invalid_import",
        &["contracts", "compile", "tests/fail_contracts/wrong_import.ral"],
    );
}

#[test]
fn test_compile_no_import() {
    perform_cmd_test_dev(
        "compile_no_import",
        &["contracts", "compile", "tests/fail_contracts/sub_contract_no_import.ral"],
    );
}
