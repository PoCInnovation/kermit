use clap::Parser;

#[derive(Parser)]
pub enum WalletArgs {
    #[clap(subcommand)]
    New,
}
