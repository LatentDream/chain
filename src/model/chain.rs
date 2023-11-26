use std::thread;
use std::time::Duration;
use std::{collections::HashMap, time::Instant};
use std::sync::mpsc::{channel, Sender, Receiver, TryRecvError};
use super::{Block, Account, Transaction};
use std::sync::{Arc, Mutex};
use anyhow::Result;

#[derive(Debug)]
pub struct Blockchain {
    blockchain: Arc<Mutex<Vec<Block>>>,
    accounts: Arc<Mutex<HashMap<String, Account>>>,
    sender: Sender<Transaction>,
}

impl Blockchain {
    pub fn new() -> Blockchain {
        let blockchain = Arc::new(Mutex::new(Vec::new()));
        let accounts = Arc::new(Mutex::new(HashMap::new()));
        let (sender, receiver) = channel();

        let blockchain_ref = Arc::clone(&blockchain);
        let accounts_ref = Arc::clone(&accounts);

        // Worker thread to process transactions of the header block
        thread::spawn(move || {
            Self::minter(receiver, blockchain_ref, accounts_ref);
        });
        println!("Blockchain created");
        Blockchain {
            blockchain,
            accounts,
            sender,
        }
    }

    pub fn create_account(&mut self, id: String, balance: u128) -> Result<()> {
        if self.accounts.lock().unwrap().contains_key(&id) {
            return Err(anyhow::anyhow!("Account already exists"));
        }
        let account = Account::new(id, balance);
        println!("  New account created: {}", account.get_id());
        self.accounts.lock().unwrap().insert(account.get_id().clone(), account);
        Ok(())
    }
    
    pub fn get_balance(&self, id: &String) -> Result<u128> {
        let accounts = self.accounts.lock().unwrap();
        let accout = accounts.get(id);
        match accout {
            Some(account) => Ok(account.get_balance().clone()),
            None => Err(anyhow::anyhow!("Account not found")),
        }
    }

    pub fn add_transaction(&self, from: String, to: String, amount: u128) -> Result<()> {
        let transaction = Transaction::new(from, to, amount);
        let accounts = self.accounts.lock().unwrap();
        // Pre-check if the accounts exist and has sufficient funds
        if !accounts.contains_key(transaction.get_sender_id()) {
            return Err(anyhow::anyhow!("Account {} not found", transaction.get_sender_id()));
        } else if !accounts.contains_key(transaction.get_receiver_id()) {
            return Err(anyhow::anyhow!("Account {} not found", transaction.get_receiver_id()));
        } else if *(accounts.get(transaction.get_sender_id()).unwrap().get_balance()) < *transaction.get_amount() {
            return Err(anyhow::anyhow!("Insufficient funds"));
        } else if transaction.get_amount() <= &0 {
            return Err(anyhow::anyhow!("Transaction amount must be greater than 0"));
        }

        // Send the transaction to the processing thread
        if let Err(err) = self.sender.send(transaction) {
            return Err(anyhow::anyhow!("Error sending transaction: {:?}", err));
        }

        Ok(())
    }

    fn minter(receiver: Receiver<Transaction>, blockchain_ref: Arc<Mutex<Vec<Block>>>, accounts_ref: Arc<Mutex<HashMap<String, Account>>>) {

        let mut current_block = Block::new();
        let mut last_process_time = Instant::now();
        let process_interval = Duration::from_secs(10);
    
        let _minter_thread = thread::spawn(move || {
    
            loop {
                // Check if the thread has received a transaction
                match receiver.try_recv() {
                    Ok(transaction) => {
                        println!("  Received transaction: {}", transaction);
                        current_block.add_transaction(transaction);
                    },
                    Err(TryRecvError::Empty) => {
                        // No transaction received, continue
                    },
                    Err(TryRecvError::Disconnected) => {
                        println!("Minter thread disconnected");
                        break;
                    }
                }
    
                // Check if it's time to process the block
                if last_process_time.elapsed() >= process_interval {
                    println!("Minting block");
                    let _ = Blockchain::process_block(current_block.clone(), Arc::clone(&blockchain_ref), Arc::clone(&accounts_ref));
                    last_process_time = Instant::now();
                    current_block = Block::new();
                }
            }
        });    
    }


    fn process_block(new_block: Block, blockchain: Arc<Mutex<Vec<Block>>>, accounts: Arc<Mutex<HashMap<String, Account>>>) -> Result<()> {
        let mut accounts = accounts.lock().unwrap();
        let mut valid_new_block = Block::new();

        // Process each transaction in the block and update the accounts
        for transaction in new_block.get_transactions() {
            let able_to_withdraw = accounts.get_mut(transaction.get_sender_id()).unwrap().withdraw(*transaction.get_amount());
            if able_to_withdraw.is_err() {
                println!("Transaction dropped - Sender does not have sufficient funds: {} -> {} : {}b", transaction.get_sender_id(), transaction.get_receiver_id(), transaction.get_amount());
                continue;
            }            
            accounts.get_mut(transaction.get_receiver_id()).unwrap().deposit(*transaction.get_amount());
            valid_new_block.add_transaction(transaction.clone());
        }
        blockchain.lock().unwrap().push(valid_new_block.clone());
        Ok(())
    }

}


