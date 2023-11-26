use std::fmt;

#[derive(Debug, Clone)]
pub struct Transfer { 
    sender: String,
    receiver: String,
    amount: u128
}

#[derive(Debug, Clone)]
pub struct Block {
    transfers: Vec<Transfer>
}


impl Transfer {
    pub fn new(sender: String, receiver: String, amount: u128) -> Transfer {
        Transfer {
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

impl fmt::Display for Transfer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Transaction: {} -> {} : {}b", self.sender, self.receiver, self.amount)
    }
}

impl Block {
    pub fn new() -> Block {
        Block {
            transfers: Vec::new()
        }
    }

    pub fn add_transfer(&mut self, transaction: Transfer) {
        self.transfers.push(transaction);
    }

    pub fn get_transfers(&self) -> &Vec<Transfer> {
        &self.transfers
    }

    pub fn clear(&mut self) {
        self.transfers.clear();
    }
}
