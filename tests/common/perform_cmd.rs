use std::{
    env,
    fs::{self, File},
    io::Write,
    path::PathBuf,
    process::Command,
};

use anyhow::Result;
use insta::with_settings;
use insta_cmd::{assert_cmd_snapshot, get_cargo_bin};
use serde_json::Value;

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

pub fn perform_cmd_output(name: &str, args: &[&str], url: &str) -> String {
    let tmp_dir = env::temp_dir();
    let mut file_path = PathBuf::from(&tmp_dir);
    file_path.push(name);

    // Write the command to the temporary file
    let mut file = File::create(&file_path).expect("Failed to create file");
    let output = build_cmd(Some(url), args)
        .output()
        .expect("Failed to execute command");

    if !output.status.success() {
        panic!(
            "Command execution failed with status: {}\nstderr: {}",
            output.status,
            String::from_utf8_lossy(&output.stderr)
        );
    }

    writeln!(file, "{}", String::from_utf8_lossy(&output.stdout)).expect("Failed to write to file");

    file_path.to_string_lossy().to_string()
}

////////////////////////////////////

pub fn get_json(path: &str) -> Value {
    let data = fs::read_to_string(path).expect("Unable to read path file");
    serde_json::from_str(&data).expect("Unable to parse path JSON")
}

pub fn get_json_str_field_from_file(path: &str, field_name: &str) -> String {
    let data = get_json(path);

    data[field_name]
        .as_str()
        .unwrap_or_else(|| panic!("{field_name} not found in {path} JSON"))
        .to_string()
}
