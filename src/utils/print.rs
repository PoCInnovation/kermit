use anyhow::Result;
use serde_json::Value;

use crate::utils::HttpResponse;

pub fn print_output(output: Option<HttpResponse<Value>>) -> Result<()> {
    if let Some(output) = output {
        serde_json::to_writer_pretty(std::io::stdout(), &output.data)?;
        println!();
    } else {
        println!("Success!");
    }

    Ok(())
}
