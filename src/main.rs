use clap::{Parser, ValueEnum};
use anyhow::Result;

mod wallets;

#[derive(Parser, Debug)]
#[command(name = "calldatafmt")]
#[command(about = "Format calldata for hardware-wallet verification", long_about = None)]
#[command(version)]
struct Cli {
    wallet: Wallet,
    calldata: String,
}

#[derive(Copy, Clone, Debug, ValueEnum)]
enum Wallet {
    Flex,
    Safe7,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.wallet {
        Wallet::Flex => {
            println!("\nDisplaying the formatted calldata on the Ledger Flex\n");
            let _ = wallets::display_ledger_flex_calldata(&cli.calldata);
        }
        Wallet::Safe7 => {
            println!("\nDisplaying the formatted calldata on the Trezor Safe 7\n");
            let _ = wallets::display_trezor_safe7_calldata(&cli.calldata);
        }
    }

    Ok(())
}

