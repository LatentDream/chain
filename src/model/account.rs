use anyhow::Result;


#[derive(Debug)]
pub struct Account {
    id: String,
    balance: u128,
}

impl Account {
    pub fn new(id: String, balance: u128) -> Account {
        Account {
            id,
            balance
        }
    }

    pub fn get_id(&self) -> &String {
        &self.id
    }

    pub fn get_balance(&self) -> &u128 {
        &self.balance
    }

    pub fn withdraw(&mut self, amount: u128) -> Result<()> {
        if self.balance < amount {
            return Err(anyhow::anyhow!("Insufficient funds"));
        }
        self.balance -= amount;
        Ok(())
    }

    pub fn deposit(&mut self, amount: u128) {
        self.balance += amount;
    }
}
