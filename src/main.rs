mod balances;

use balances::Balances;

fn main() {
    let mut balances = Balances::new();

    // Add users and initialize balances
    balances.add_user("Alice".to_string(), 1000);
    balances.add_user("Bob".to_string(), 500);

    // Print initial balances
    println!(
        "Alice's balance: {}",
        balances.get_balance(&"Alice".to_string())
    );
    println!(
        "Bob's balance: {}",
        balances.get_balance(&"Bob".to_string())
    );

    // Perform a transfer
    match balances.transfer("Alice".to_string(), "Bob".to_string(), 300) {
        Ok(_) => println!("Transfer successful!"),
        Err(e) => println!("Transfer failed: {}", e),
    }

    // Print updated balances
    println!(
        "Alice's balance: {}",
        balances.get_balance(&"Alice".to_string())
    );
    println!(
        "Bob's balance: {}",
        balances.get_balance(&"Bob".to_string())
    );
}
