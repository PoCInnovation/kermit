use anyhow::Result;
use bigdecimal::BigDecimal;
use clap::{Parser, ValueEnum};

const ADDRESS_ZERO: &str = "111111111111111111111111111111111";
const HASH_ZERO: &str = "0000000000000000000000000000000000000000000000000000000000000000";

const ATTO_IN_ALPH: u64 = 10u64.pow(18);
const ATTO_IN_GATTO: u64 = 10u64.pow(9);

/// CLI arguments for `kermit utils`.
#[derive(Parser)]
pub enum UtilsSubcommands {
    /// Get the Alephium zero address.
    #[command(visible_alias = "az")]
    AddressZero,

    /// Get the Alephium hash zero.
    #[command(visible_alias = "hz")]
    HashZero,

    /// Convert amount between different Alephium units (atto, gatto, alph).
    #[command(visible_alias = "c")]
    Convert {
        amount: BigDecimal,
        unit: AlephiumUnit,
    },
}

#[derive(Clone, ValueEnum)]
#[clap(rename_all = "lowercase")]
pub enum AlephiumUnit {
    Alph,
    Gatto,
    Atto,
}

fn convert_amount(amount: BigDecimal, unit: AlephiumUnit) -> (BigDecimal, BigDecimal, BigDecimal) {
    match unit {
        AlephiumUnit::Alph => {
            let atto = &amount * ATTO_IN_ALPH;
            let gatto = &amount * ATTO_IN_GATTO;
            (atto, gatto, amount)
        },
        AlephiumUnit::Gatto => {
            let atto = &amount * ATTO_IN_GATTO;
            let alph = &amount / ATTO_IN_GATTO;
            (atto, amount, alph)
        },
        AlephiumUnit::Atto => {
            let gatto = &amount / ATTO_IN_GATTO;
            let alph = &amount / ATTO_IN_ALPH;
            (amount, gatto, alph)
        },
    }
}

impl UtilsSubcommands {
    pub async fn run(self) -> Result<()> {
        let output = match self {
            Self::AddressZero => ADDRESS_ZERO,
            Self::HashZero => HASH_ZERO,
            Self::Convert { amount, unit } => {
                let (atto, gatto, alph) = convert_amount(amount, unit);

                &format!(
                    "atto:\t{}\ngatto:\t{}\nalph:\t{}",
                    atto.to_plain_string(),
                    gatto.to_plain_string(),
                    alph.to_plain_string()
                )
            },
        };

        println!("{output}");

        Ok(())
    }
}
