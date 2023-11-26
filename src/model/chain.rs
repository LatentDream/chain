use std::thread;
use std::time::Duration;
use std::{collections::HashMap, time::Instant};
use uuid::Uuid;
use std::sync::mpsc::{channel, Sender, Receiver, TryRecvError};
use super::{Block, Account, Transaction, block};
use std::sync::{Arc, Mutex};

#[derive(Debug)]
pub struct Blockchain {
    blocks: Arc<Mutex<Vec<Block>>>,
    accounts: HashMap<Uuid, Account>,
    sender: Sender<Transaction>,
}

impl Blockchain {
    pub fn new() -> Blockchain {

        let blocks = Arc::new(Mutex::new(Vec::new()));
        let accounts = HashMap::new();
        let (sender, receiver) = channel();

        let blocks_clone = Arc::clone(&blocks);

        // Worker thread to process transactions
        thread::spawn(move || {
            Self::minter(receiver, blocks_clone);
        });

        Blockchain {
            blocks,
            accounts,
            sender,
        }
    }

    pub fn create_account(&mut self) -> Uuid {
        let account = Account::new();
        let id = account.get_id().clone();
        self.accounts.insert(account.id, account);
        id
    }
    
    pub fn get_account(&self, id: &Uuid) -> Option<&Account> {
        let accout = self.accounts.get(id);
        match accout {
            Some(account) => Some(account.clone()),
            None => None
        }
    }

    pub fn add_transaction(&self, transaction: Transaction) {
        // Send the transaction to the processing thread
        if let Err(err) = self.sender.send(transaction) {
            eprintln!("Error sending transaction: {:?}", err);
        }
    }


    fn minter(receiver: Receiver<Transaction>, blocks: Arc<Mutex<Vec<Block>>>) {

        let mut current_block = Block::new();
        let mut last_process_time = Instant::now();
        let process_interval = Duration::from_secs(10);
    
        // Create channel to communicate with the thread
        let _minter_thread = thread::spawn(move || {
    
            loop {
                // Check if the thread has received a transaction
                match receiver.try_recv() {
                    Ok(transaction) => {
                        println!("Received transaction: {}", transaction);
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
                    last_process_time = Instant::now();
                    blocks.lock().unwrap().push(current_block.clone());
                    current_block = Block::new();
                }
            }
        });    
    }

}


