use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::{collections::HashMap, time::Instant};
use super::{Block, Account, Transfer};
use anyhow::Result;
use std::boxed::Box;

pub struct Transaction {
    pub transaction: Box<dyn FnOnce() -> Result<()> + Send>,
    pub callback: Box<dyn FnOnce(Result<()>) -> () + Send>,
}


pub struct Blockchain {
    blockchain: Vec<Block>,
    accounts: Arc<Mutex<HashMap<String, Account>>>,
    current_block: Arc<Mutex<Block>>,
    transactions: Vec<Transaction>,
    last_process_time: Instant,
    process_interval: Duration
}

impl Blockchain {
    pub fn new() -> Blockchain {
        let blockchain = Vec::new();
        let accounts = Arc::new(Mutex::new(HashMap::new()));
        let current_block = Arc::new(Mutex::new(Block::new()));
        let last_process_time = Instant::now();
        let process_interval = Duration::from_secs(10);

        Blockchain {
            blockchain,
            accounts,
            transactions: Vec::new(),
            current_block,
            last_process_time,
            process_interval
        }
    }

    pub fn get_balance(&self, id: &String) -> Result<u128> {
        let accounts = self.accounts.lock().unwrap();
        let account = accounts.get(id);
        match account {
            Some(account) => Ok(account.get_balance().clone()),
            None => Err(anyhow::anyhow!("Account not found")),
        }
    }

    pub fn create_account(
        &mut self,
        id: String,
        balance: u128,
        callback: Box<dyn FnOnce(Result<()>) -> () + Send + 'static>,
    ) {
        let accounts_ref = Arc::clone(&self.accounts);

        let closure = move || -> Result<()> {
            let mut accounts = accounts_ref.lock().unwrap();
            if accounts.contains_key(&id) {
                return Err(anyhow::anyhow!("Account already exists"));
            }
            let account = Account::new(id.clone(), balance);
            println!("  New account created: {}", account.get_id());
            accounts.insert(account.get_id().clone(), account);
            Ok(())
        };

        self.transactions.push(Transaction {
            transaction: Box::new(closure),
            callback,
        });
    }
        
    pub fn add_transfer(
        &mut self,
        sender: String,
        receiver: String,
        amount: u128,
        callback: Box<dyn FnOnce(Result<()>) -> () + Send>,
    ) {
        let transfer = Transfer::new(sender.clone(), receiver.clone(), amount);
        let accounts_ref = Arc::clone(&self.accounts);
        let current_block_ref = Arc::clone(&self.current_block);
        
        let closure = move || -> Result<()> {
            let mut accounts = accounts_ref.lock().unwrap();
            if !accounts.contains_key(&sender) {
                return Err(anyhow::anyhow!("Account {} not found", transfer.get_sender_id()));
            } else if !accounts.contains_key(&receiver) {
                return Err(anyhow::anyhow!("Account {} not found", transfer.get_receiver_id()));
            } else if transfer.get_amount() <= &0 {
                return Err(anyhow::anyhow!("Transfer amount must be greater than 0"));
            }
    
            let able_to_withdraw = accounts.get_mut(&sender).unwrap().withdraw(*transfer.get_amount());
            if able_to_withdraw.is_err() {
                println!("  Transfer failed - Sender does not have sufficient funds: {} -> {} : {}b", transfer.get_sender_id(), transfer.get_receiver_id(), transfer.get_amount());
                return Err(anyhow::anyhow!("Insufficient funds"));
            }
            accounts.get_mut(&receiver).unwrap().deposit(*transfer.get_amount());
            current_block_ref.lock().unwrap().add_transfer(transfer.clone());
            println!("  Transfer complete: {} -> {} : {}b", transfer.get_sender_id(), transfer.get_receiver_id(), transfer.get_amount());
            Ok(())
        };
    
        self.transactions.push(Transaction {
            transaction: Box::new(closure),
            callback,
        });

    }

    pub fn mint(&mut self) {
        if self.last_process_time.elapsed() > self.process_interval {
            println!("Minting block...");
            self.last_process_time = Instant::now();
            // Execute the transactions
            while let Some(transaction) = self.transactions.pop() {
                let result = (transaction.transaction)();        
                (transaction.callback)(result);
            }
            self.transactions.clear();
            // Add the current block to the blockchain and create a new block
            let mut current_block = self.current_block.lock().unwrap();
            self.blockchain.push(current_block.clone());
            current_block.clear();
        }        
    }

}
