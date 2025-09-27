use anyhow::Result;
use clap::Parser;

use blockchain::{cli::*, Blockchain};

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Interactive) | None => {
            let mut interactive = InteractiveMode::new()?;
            interactive.run()?;
        }
        Some(Commands::AddBlock { transactions }) => {
            let mut blockchain = Blockchain::new()?;
            let parsed_transactions: Result<Vec<_>, _> = transactions
                .iter()
                .map(|tx_str| parse_transaction_string(tx_str))
                .collect();

            match parsed_transactions {
                Ok(txs) => {
                    blockchain.add_block(txs)?;
                    println!("✅ Block added successfully!");
                }
                Err(e) => {
                    eprintln!("❌ Error parsing transactions: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Some(Commands::Show) => {
            let blockchain = Blockchain::new()?;
            for (i, block) in blockchain.chain().iter().enumerate() {
                println!("Block #{}: {:?}", i, block);
            }
        }
        Some(Commands::Validate) => {
            let blockchain = Blockchain::new()?;
            match blockchain.is_chain_valid() {
                Ok(()) => println!("✅ Blockchain is valid!"),
                Err(e) => {
                    println!("❌ Blockchain validation failed: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Some(Commands::Balance { address }) => {
            let blockchain = Blockchain::new()?;
            let balance = blockchain.get_balance(&address);
            println!("Balance for {}: {}", address, balance);
        }
    }

    Ok(())
}