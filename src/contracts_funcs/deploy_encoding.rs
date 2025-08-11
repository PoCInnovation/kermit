use std::collections::HashMap;

type Fields = HashMap<String, String>;

fn add_std_prefix(fields: &mut Fields, std_interface_id: &str) {
    let std_interface_prefix = "414c5048".to_string();
    if !std_interface_id.is_empty() {
        fields.insert(
            "__stdInterfaceId".to_string(),
            std_interface_prefix + std_interface_id,
        );
    }
}

