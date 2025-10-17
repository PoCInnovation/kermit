use anyhow::Result;
use serde_json::Value;

pub fn print_output(output: Option<Value>) -> Result<()> {
    if let Some(output) = output {
        serde_json::to_writer_pretty(std::io::stdout(), &output)?;
        println!();
    } else {
        println!("Success!");
    }

    Ok(())
}
