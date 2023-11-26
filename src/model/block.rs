use uuid::Uuid;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Transaction { 
    sender: Uuid,
    receiver: Uuid,
    amount: u32
}

#[derive(Debug, Clone)]
pub struct Block {
    transactions: Vec<Transaction>
}


impl Transaction {
    pub fn new(sender: Uuid, receiver: Uuid, amount: u32) -> Transaction {
        Transaction {
            sender,
            receiver,
            amount
        }
    }

    pub fn get_sender(&self) -> &Uuid {
        &self.sender
    }

    pub fn get_receiver(&self) -> &Uuid {
        &self.receiver
    }

    pub fn get_amount(&self) -> &u32 {
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
