use std::fmt;

#[derive(Debug, Clone)]
pub struct Transaction { 
    sender: String,
    receiver: String,
    amount: u128
}

#[derive(Debug, Clone)]
pub struct Block {
    transactions: Vec<Transaction>
}


impl Transaction {
    pub fn new(sender: String, receiver: String, amount: u128) -> Transaction {
        Transaction {
            sender,
            receiver,
            amount
        }
    }

    pub fn get_sender_id(&self) -> &String {
        &self.sender
    }

    pub fn get_receiver_id(&self) -> &String {
        &self.receiver
    }

    pub fn get_amount(&self) -> &u128 {
        &self.amount
    }
}

impl fmt::Display for Transaction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Transaction: {} -> {} : {}b", self.sender, self.receiver, self.amount)
    }
}

impl Block {
    pub fn new() -> Block {
        Block {
            transactions: Vec::new()
        }
    }

    pub fn add_transaction(&mut self, transaction: Transaction) {
        self.transactions.push(transaction);
    }

    pub fn get_transactions(&self) -> &Vec<Transaction> {
        &self.transactions
    }
}
