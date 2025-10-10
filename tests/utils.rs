use std::process::Command;

use insta::with_settings;
use insta_cmd::{assert_cmd_snapshot, get_cargo_bin};
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::{env, fs};

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

    let custom_filter = custom_filter.unwrap_or(FILTERS.to_vec());

    with_settings!({
        prepend_module_to_snapshot => false,
        snapshot_path => format!("snapshots/{}", module_path!().split("::").next().unwrap()),
        filters => custom_filter,
    }, {
        assert_cmd_snapshot!(name, Command::new(get_cargo_bin(BIN_NAME)).args(new_args));
    });
}

////////////////////////////////////

pub fn get_json_str_field_from_file(path: &str, field_name: &str) -> String {
    let data: serde_json::Value = {
        let data = fs::read_to_string(&path).expect("Unable to read path file");
        serde_json::from_str(&data).expect("Unable to parse path JSON")
    };

    data[field_name]
        .as_str()
        .expect(&format!("{field_name} not found in {path} JSON"))
        .to_string()
}
