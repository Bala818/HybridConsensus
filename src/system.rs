use std::collections::BTreeMap;
#[derive(Debug)]
pub struct System {
    block_number: u128,
    transaction_count: BTreeMap<String, u128>,
}
impl System {
    //new instance of the system pallet
    pub fn new() -> Self {
        System {
            block_number: 0,
            transaction_count: BTreeMap::new(),
        }
    }
    pub fn increment_block(&mut self) {
        self.block_number = self
            .block_number
            .checked_add(1)
            .expect("Block number overflow!");
    }
    //current block number
    pub fn get_block_number(&self) -> u128 {
        self.block_number
    }
    pub fn increment_nonce(&mut self, user: String) {
        let nonce = self.transaction_count.entry(user.clone()).or_insert(0);
        *nonce += 1;
    }
    pub fn get_nonce(&self, user: &String) -> u128 {
        *self.transaction_count.get(user).unwrap_or(&0)
    }
}
