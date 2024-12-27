use std::collections::BTreeMap;
pub struct Balances {
    balances: BTreeMap<String, u128>, // Key: User ID, Value: Balance
}

impl Balances {
    pub fn new() -> Self {
        Balances {
            balances: BTreeMap::new(),
        }
    }
    pub fn add_user(&mut self, user_id: String, initial_balance: u128) {
        self.balances.insert(user_id, initial_balance);
    }
    pub fn get_balance(&self, user_id: &String) -> u128 {
        *self.balances.get(user_id).unwrap_or(&0)
    }
    pub fn transfer(&mut self, from: String, to: String, amount: u128) -> Result<(), String> {
        if let Some(from_balance) = self.balances.get(&from) {
            if *from_balance < amount {
                return Err("Insufficient balance.".to_string());
            }
        } else {
            return Err("Sender not found.".to_string());
        }
        if !self.balances.contains_key(&to) {
            return Err("Receiver not found.".to_string());
        }
        if let Some(from_balance) = self.balances.get_mut(&from) {
            *from_balance -= amount;
        }
        if let Some(to_balance) = self.balances.get_mut(&to) {
            *to_balance += amount;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_user() {
        let mut balances = Balances::new();
        balances.add_user("Alice".to_string(), 1000);
        assert_eq!(balances.get_balance(&"Alice".to_string()), 1000);
    }

    #[test]
    fn test_transfer() {
        let mut balances = Balances::new();
        balances.add_user("Alice".to_string(), 1000);
        balances.add_user("Bob".to_string(), 500);

        let result = balances.transfer("Alice".to_string(), "Bob".to_string(), 300);
        assert!(result.is_ok());
        assert_eq!(balances.get_balance(&"Alice".to_string()), 700);
        assert_eq!(balances.get_balance(&"Bob".to_string()), 800);
    }

    #[test]
    fn test_insufficient_balance() {
        let mut balances = Balances::new();
        balances.add_user("Alice".to_string(), 100);
        let result = balances.transfer("Alice".to_string(), "Bob".to_string(), 200);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Insufficient balance.");
    }
}
