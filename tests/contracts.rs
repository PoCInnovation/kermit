mod utils;

use crate::utils::perform_cmd_test_dev;

#[test]
fn test_compile_simple() {
    perform_cmd_test_dev(
        "compile",
        &["contracts", "compile", "tests/contracts/sub_contract.ral"],
    );
}
