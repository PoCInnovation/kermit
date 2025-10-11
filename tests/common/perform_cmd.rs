use std::process::Command;

use anyhow::Result;
use insta::with_settings;
use insta_cmd::{assert_cmd_snapshot, get_cargo_bin};

const BIN_NAME: &str = "kermit";

fn build_cmd(url: Option<&str>, args: &[&str]) -> Command {
    let mut cmd = Command::new(get_cargo_bin(BIN_NAME));

    if let Some(url) = url {
        cmd.arg(format!("--url={url}"));
    }
    cmd.args(args);

    cmd
}

pub fn perform_cmd(url: &str, args: &[&str]) -> Result<()> {
    build_cmd(Some(url), args).spawn()?.wait()?;

    Ok(())
}

pub fn perform_cmd_test(
    name: &str,
    path: &str,
    url: Option<&str>,
    args: &[&str],
    filters: &[(&str, &str)],
) {
    with_settings!({
        prepend_module_to_snapshot => false,
        snapshot_path => format!("../snapshots/{path}"),
        filters => filters.to_vec(),
    }, {
        assert_cmd_snapshot!(name, build_cmd(url, args));
    });
}

#[macro_export]
macro_rules! perform_cmd_test {
    ($name:expr, $args:expr) => {
        use $crate::common::perform_cmd_test;

        perform_cmd_test($name, &module_path!().replace("::", "/"), None, $args, &[]);
    };
    ($name:expr, $url:expr, $args:expr) => {
        use $crate::common::perform_cmd_test;

        perform_cmd_test(
            $name,
            &module_path!().replace("::", "/"),
            Some($url),
            $args,
            &[],
        );
    };
    ($name:expr, $url:expr, $args:expr, $filters:expr) => {
        use $crate::common::perform_cmd_test;

        perform_cmd_test(
            $name,
            &module_path!().replace("::", "/"),
            Some($url),
            $args,
            $filters,
        );
    };
}
