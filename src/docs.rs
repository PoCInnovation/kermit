use anyhow::{Context, Result};
use clap::Parser;
use std::fs;
use std::io::{self, Write};

use crate::args::Kermit;

#[derive(Parser)]
pub struct GenerateDocsSubcommands {}

impl GenerateDocsSubcommands {
    pub async fn run(&self) -> Result<()> {
        let mut markdown = clap_markdown::help_markdown::<Kermit>();

        let contracts_header = "## `kermit contracts`";
        if let Some(pos) = markdown.find(contracts_header) {
            let insert_pos = pos + contracts_header.len();
            let mut new_markdown = markdown[..insert_pos].to_string();

            let extra_docs = fs::read_to_string("src/docs/contracts_extra.md")
                .with_context(|| "Failed to read src/docs/contracts_extra.md")?;
            new_markdown.push_str(&extra_docs);

            new_markdown.push_str(&markdown[insert_pos..]);
            markdown = new_markdown;
        }

        let mut stdout = io::stdout();
        stdout.write_all(markdown.as_bytes())?;

        Ok(())
    }
}

