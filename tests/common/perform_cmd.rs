use std::{
    env,
    fs::{self, File},
    path::PathBuf,
    process::Command,
};

use anyhow::Result;
use insta::with_settings;
use insta_cmd::{assert_cmd_snapshot, get_cargo_bin};
use serde_json::Value;
use std::io::Write;

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

///////////////////////////

const DEFAULT_FILTERS: [(&str, &str); 1] = [(
    r#""cliqueId":\s*"[0-9a-f]+""#,
    r#""cliqueId": "<CLIQUE_ID>""#,
)];

/* Run the command and write the outputed result (if success) inside a temporary file */
pub fn perform_cmd_dev(name: &str, args: &[&str], config_name: Option<&str>) -> String {
    let config_path = config_name.unwrap_or("./alephium.config.yaml");

    let mut new_args = vec!["-n", "dev", "-c", config_path];
    new_args.extend(args);

    let tmp_dir = env::temp_dir();
    let mut file_path = PathBuf::from(&tmp_dir);
    file_path.push(name);

    // Write the command to the temporary file
    let mut file = File::create(&file_path).expect("Failed to create file");
    let output = Command::new(get_cargo_bin(BIN_NAME))
        .args(new_args)
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

/* These tests run in a Devnet environment from alephium-stack */
pub fn perform_cmd_test_dev(
    name: &str,
    args: &[&str],
    config_name: Option<&str>,
    custom_filter: Option<Vec<(&str, &str)>>,
) {
    let config_path = config_name.unwrap_or("./alephium.config.yaml");

    let mut new_args = vec!["-n", "dev", "-c", config_path];
    new_args.extend(args);

    let custom_filter = custom_filter.unwrap_or(DEFAULT_FILTERS.to_vec());

    with_settings!({
        prepend_module_to_snapshot => false,
        snapshot_path => format!("snapshots/{}", module_path!().split("::").next().unwrap()),
        filters => custom_filter,
    }, {
        assert_cmd_snapshot!(name, Command::new(get_cargo_bin(BIN_NAME)).args(new_args));
    });
}

////////////////////////////////////

pub fn get_json(path: &str) -> Value {
    let data = fs::read_to_string(&path).expect("Unable to read path file");
    serde_json::from_str(&data).expect("Unable to parse path JSON")
}

pub fn get_json_str_field_from_file(path: &str, field_name: &str) -> String {
    let data = get_json(path);

    data[field_name]
        .as_str()
        .expect(&format!("{field_name} not found in {path} JSON"))
        .to_string()
}
