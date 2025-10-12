mod common;

#[test]
fn test_help() {
    perform_cmd_test!("help", &["--help"]);
}

#[test]
fn test_bad_command() {
    perform_cmd_test!("bad_command", &["bad-command"]);
}

#[test]
fn test_bad_url() {
    perform_cmd_test!(
        "bad_url",
        &["--url", "http://invalid-url", "infos", "version"]
    );
}
