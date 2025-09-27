use clap::{Parser, Subcommand};
use colored::*;
use std::io::{self, Write};

use crate::blockchain::Blockchain;
use crate::errors::Result;
use crate::transaction::Transaction;

#[derive(Parser)]
#[command(name = "blockchain")]
#[command(about = "A Rust-based blockchain prototype with simplified Proof-of-History")]
#[command(version = "0.1.0")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(about = "Start interactive mode")]
    Interactive,
    #[command(about = "Add a new block with transactions")]
    AddBlock {
        #[arg(help = "Transaction data in format 'from:to:amount:data'")]
        transactions: Vec<String>,
    },
    #[command(about = "Display the entire blockchain")]
    Show,
    #[command(about = "Validate the blockchain")]
    Validate,
    #[command(about = "Get balance for an address")]
    Balance {
        #[arg(help = "Address to check balance for")]
        address: String,
    },
}

pub struct InteractiveMode {
    blockchain: Blockchain,
}

impl InteractiveMode {
    pub fn new() -> Result<Self> {
        Ok(InteractiveMode {
            blockchain: Blockchain::new()?,
        })
    }

    pub fn run(&mut self) -> Result<()> {
        println!("{}", "🔗 Blockchain initialized!".bright_green().bold());
        println!("{}", "Welcome to the Rust Blockchain Prototype".bright_blue().bold());
        println!();

        loop {
            self.print_menu();
            let choice = self.get_user_input("Enter your choice: ")?;

            match choice.trim() {
                "1" => self.add_block_interactive()?,
                "2" => self.show_blockchain()?,
                "3" => self.validate_blockchain()?,
                "4" => self.show_balance_interactive()?,
                "5" => self.show_stats()?,
                "6" => {
                    println!("{}", "Goodbye! 👋".bright_yellow());
                    break;
                }
                _ => {
                    println!("{}", "❌ Invalid choice. Please try again.".red());
                }
            }
            println!();
        }

        Ok(())
    }

    fn print_menu(&self) {
        println!("{}", "╭─ Choose an action ─╮".bright_cyan());
        println!("│ 1. Add a new block  │");
        println!("│ 2. Show blockchain  │");
        println!("│ 3. Validate chain   │");
        println!("│ 4. Check balance    │");
        println!("│ 5. Show stats       │");
        println!("│ 6. Exit             │");
        println!("{}", "╰────────────────────╯".bright_cyan());
    }

    fn add_block_interactive(&mut self) -> Result<()> {
        println!("{}", "📝 Adding a new block".bright_yellow().bold());

        let mut transactions = Vec::new();

        loop {
            println!("Enter transaction details (or 'done' to finish):");

            let from = self.get_user_input("From address: ")?;
            if from.trim().to_lowercase() == "done" {
                break;
            }

            let to = self.get_user_input("To address: ")?;
            let amount_str = self.get_user_input("Amount: ")?;
            let data = self.get_user_input("Data (optional): ")?;

            let amount: f64 = amount_str.trim().parse().map_err(|_| {
                crate::errors::BlockchainError::InvalidTransaction {
                    message: "Invalid amount format".to_string(),
                }
            })?;

            let data = if data.trim().is_empty() { None } else { Some(data.trim().to_string()) };

            let transaction = Transaction::new(
                from.trim().to_string(),
                to.trim().to_string(),
                amount,
                data,
            )?;

            transactions.push(transaction);
            println!("{}", "✅ Transaction added".green());
        }

        if transactions.is_empty() {
            println!("{}", "❌ No transactions to add".red());
            return Ok(());
        }

        self.blockchain.add_block(transactions)?;
        println!("{}", "🎉 Block added successfully!".bright_green().bold());

        Ok(())
    }

    fn show_blockchain(&self) -> Result<()> {
        println!("{}", "🔗 Blockchain Contents".bright_blue().bold());
        println!("{}", "─".repeat(80).bright_black());

        for (i, block) in self.blockchain.chain().iter().enumerate() {
            println!("{}", format!("Block #{}", i).bright_white().bold());
            println!("  Index: {}", block.index);
            println!("  Timestamp: {}", block.timestamp.format("%Y-%m-%d %H:%M:%S UTC"));
            println!("  Previous Hash: {}", &block.previous_hash[..16].bright_black());
            println!("  Hash: {}", &block.hash[..16].bright_green());
            println!("  PoH Hash: {}", &block.poh_hash[..16].bright_yellow());
            println!("  Transactions:");

            for (j, tx) in block.transactions.iter().enumerate() {
                println!("    {}. {} → {} ({})",
                    j + 1,
                    tx.from.bright_cyan(),
                    tx.to.bright_magenta(),
                    tx.amount.to_string().bright_yellow()
                );
                if let Some(data) = &tx.data {
                    println!("       Data: {}", data.italic());
                }
            }
            println!("{}", "─".repeat(40).bright_black());
        }

        Ok(())
    }

    fn validate_blockchain(&self) -> Result<()> {
        print!("{}", "🔍 Validating blockchain... ".bright_yellow());
        io::stdout().flush().unwrap();

        match self.blockchain.is_chain_valid() {
            Ok(()) => println!("{}", "✅ Blockchain is valid!".bright_green().bold()),
            Err(e) => println!("{}", format!("❌ Validation failed: {}", e).red().bold()),
        }

        Ok(())
    }

    fn show_balance_interactive(&self) -> Result<()> {
        let address = self.get_user_input("Enter address to check: ")?;
        let balance = self.blockchain.get_balance(address.trim());

        println!("{}", format!("💰 Balance for {}: {}",
            address.trim().bright_cyan(),
            balance.to_string().bright_yellow().bold()
        ));

        Ok(())
    }

    fn show_stats(&self) -> Result<()> {
        println!("{}", "📊 Blockchain Statistics".bright_blue().bold());
        println!("{}", "─".repeat(30).bright_black());

        println!("Total blocks: {}", self.blockchain.len().to_string().bright_green());
        println!("PoH tick count: {}", self.blockchain.poh_tick_count().to_string().bright_yellow());

        let total_transactions: usize = self.blockchain
            .chain()
            .iter()
            .map(|block| block.transactions.len())
            .sum();

        println!("Total transactions: {}", total_transactions.to_string().bright_cyan());

        if let Ok(latest_block) = self.blockchain.get_latest_block() {
            println!("Latest block hash: {}", latest_block.hash[..16].bright_green());
        }

        Ok(())
    }

    fn get_user_input(&self, prompt: &str) -> Result<String> {
        print!("{}", prompt.bright_white());
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        Ok(input)
    }
}

pub fn parse_transaction_string(tx_str: &str) -> Result<Transaction> {
    let parts: Vec<&str> = tx_str.split(':').collect();

    if parts.len() < 3 {
        return Err(crate::errors::BlockchainError::InvalidTransaction {
            message: "Transaction format should be 'from:to:amount:data'".to_string(),
        });
    }

    let from = parts[0].trim().to_string();
    let to = parts[1].trim().to_string();
    let amount: f64 = parts[2].trim().parse().map_err(|_| {
        crate::errors::BlockchainError::InvalidTransaction {
            message: "Invalid amount format".to_string(),
        }
    })?;

    let data = if parts.len() > 3 && !parts[3].trim().is_empty() {
        Some(parts[3].trim().to_string())
    } else {
        None
    };

    Transaction::new(from, to, amount, data)
}