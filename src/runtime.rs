use crate::balances::Balances;
use crate::system::System;

#[derive(Debug)]
pub struct Runtime {
    pub system: System,
    pub balances: Balances,
}

impl Runtime {
    /// Initialize a new runtime
    pub fn new() -> Self {
        Runtime {
            system: System::new(),
            balances: Balances::new(),
        }
    }

    /// Simulate a transaction between two users
    pub fn simulate_transaction(&mut self, from: String, to: String, amount: u128) {
        // Increment block number
        self.system.increment_block();

        // Increment nonce for the sender
        self.system.increment_nonce(from.clone());

        // Perform the balance transfer
        match self.balances.transfer(from.clone(), to.clone(), amount) {
            Ok(_) => {
                println!(
                    "Transaction successful! {} sent {} to {}.",
                    from, amount, to
                );
            }
            Err(e) => {
                println!("Transaction failed: {}", e);
            }
        }

        // Print the updated state
        println!("Updated block number: {}", self.system.get_block_number());
        println!(
            "{}'s updated balance: {}",
            from,
            self.balances.get_balance(&from)
        );
        println!(
            "{}'s updated balance: {}",
            to,
            self.balances.get_balance(&to)
        );
        println!("{}'s nonce: {}", from, self.system.get_nonce(&from));
    }
}
