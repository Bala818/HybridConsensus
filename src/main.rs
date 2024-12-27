mod balances;
mod runtime;
mod system;

use balances::Balances;
use runtime::Runtime;
use system::System;

fn main() {
    // Initialize system and balances
    let mut system = System::new();
    let mut balances = Balances::new();

    // Add users and initialize balances
    balances.add_user("Alice".to_string(), 1000);
    balances.add_user("Bob".to_string(), 500);

    // Print initial block number and balances
    println!("Initial block number: {}", system.get_block_number());
    println!(
        "Alice's initial balance: {}",
        balances.get_balance(&"Alice".to_string())
    );

    println!(
        "Bob's initial balance: {}",
        balances.get_balance(&"Bob".to_string())
    );

    // Simulate a transaction: Alice transfers 300 to Bob
    system.increment_block(); // Increment block number
    system.increment_nonce("Alice".to_string()); // Increment Alice's transaction nonce
    match balances.transfer("Alice".to_string(), "Bob".to_string(), 300) {
        Ok(_) => println!("Transaction successful!"),
        Err(e) => println!("Transaction failed: {}", e),
    }

    // Print updated block number, balances, and Alice's nonce
    println!("Updated block number: {}", system.get_block_number());
    println!(
        "Alice's updated balance: {}",
        balances.get_balance(&"Alice".to_string())
    );
    println!(
        "Bob's updated balance: {}",
        balances.get_balance(&"Bob".to_string())
    );
    println!("Alice's nonce: {}", system.get_nonce(&"Alice".to_string()));

    // Simulate another transaction: Bob transfers 200 to Alice
    system.increment_block(); // Increment block number
    system.increment_nonce("Bob".to_string()); // Increment Bob's transaction nonce
    match balances.transfer("Bob".to_string(), "Alice".to_string(), 200) {
        Ok(_) => println!("Transaction successful!"),
        Err(e) => println!("Transaction failed: {}", e),
    }

    // Print final block number, balances, and Bob's nonce
    println!("Final block number: {}", system.get_block_number());
    println!(
        "Alice's final balance: {}",
        balances.get_balance(&"Alice".to_string())
    );
    println!();
    println!(
        "Bob's final balance: {}",
        balances.get_balance(&"Bob".to_string())
    );
    println!("Bob's nonce: {}", system.get_nonce(&"Bob".to_string()));

    // --- NEW: Initialize runtime ---
    let mut runtime = Runtime::new();

    // Add users and initialize balances in runtime
    runtime.balances.add_user("Alice".to_string(), 1000);
    runtime.balances.add_user("Bob".to_string(), 500);

    // Print runtime initial state
    println!("\n--- Runtime Initial State ---");
    println!(
        "Runtime block number: {}",
        runtime.system.get_block_number()
    );
    println!(
        "Alice's runtime initial balance: {}",
        runtime.balances.get_balance(&"Alice".to_string())
    );
    println!(
        "Bob's runtime initial balance: {}",
        runtime.balances.get_balance(&"Bob".to_string())
    );

    // Simulate transactions in runtime
    println!("\n--- Runtime Transactions ---");
    runtime.simulate_transaction("Alice".to_string(), "Bob".to_string(), 300);
    runtime.simulate_transaction("Bob".to_string(), "Alice".to_string(), 200);

    // Print runtime final state
    println!("\n--- Runtime Final State ---");
    println!(
        "Runtime final block number: {}",
        runtime.system.get_block_number()
    );
    println!(
        "Alice's runtime final balance: {}",
        runtime.balances.get_balance(&"Alice".to_string())
    );
    println!(
        "Bob's runtime final balance: {}",
        runtime.balances.get_balance(&"Bob".to_string())
    );
}
