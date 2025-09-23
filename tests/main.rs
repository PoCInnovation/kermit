mod utils;

use crate::utils::perform_cmd_test;

#[test]
fn test_help() {
    perform_cmd_test("help", &["--help"]);
}

#[test]
fn test_bad_command() {
    perform_cmd_test("bad_command", &["invalid-command"]);
}

#[test]
fn test_bad_url() {
    perform_cmd_test(
        "bad_url",
        &["--url", "http://invalid-url", "infos", "version"],
    );
}

#[test]
fn test_bad_url_base() {
    perform_cmd_test(
        "bad_url_base",
        &["--url", "invalid-url", "infos", "version"],
    );
}
