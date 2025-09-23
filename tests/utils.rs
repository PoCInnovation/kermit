use std::process::Command;

use insta::with_settings;
use insta_cmd::{assert_cmd_snapshot, get_cargo_bin};

const BIN_NAME: &str = "kermit";
const FILTERS: [(&str, &str); 1] = [(
    r#""cliqueId":\s*"[0-9a-f]+""#,
    r#""cliqueId": "<CLIQUE_ID>""#,
)];

pub fn perform_cmd_test(name: &str, args: &[&str]) {
    with_settings!({
        prepend_module_to_snapshot => false,
        snapshot_path => format!("snapshots/{}", module_path!().split("::").next().unwrap()),
        filters => FILTERS,
    }, {
        assert_cmd_snapshot!(name, Command::new(get_cargo_bin(BIN_NAME)).args(args));
    });
}
