
use uuid::Uuid;

#[derive(Debug)]
pub struct Account {
    pub id: Uuid,
    pub balance: u64,
}

impl Account {
    pub fn new() -> Account {
        Account {
            id: Uuid::new_v4(),
            balance: 100,
        }
    }

    pub fn get_id(&self) -> &Uuid {
        &self.id
    }
}
