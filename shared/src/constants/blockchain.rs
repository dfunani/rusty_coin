#[derive(Debug)]
pub enum BlockType {
    TRANSACTION,
    CONTRACT,
    UNIT,
}

impl BlockType {
    pub fn to_string(&self) -> String {
        use BlockType::{CONTRACT, TRANSACTION, UNIT};
        match self {
            TRANSACTION => String::from("Transaction Block"),
            CONTRACT => String::from("Contract Block"),
            UNIT => String::from("Unit Block"),
        }
    }
}

#[derive(Debug)]
pub enum TransactionStatus {
    DRAFT,
    APPROVED,
    REJECTED,
    INSUFFICIENT,
    TRANSFERED,
    REVERSED,
}

impl TransactionStatus {
    pub fn to_string(&self) -> String {
        use TransactionStatus::{APPROVED, DRAFT, INSUFFICIENT, REJECTED, REVERSED, TRANSFERED};
        match self {
            DRAFT => String::from("Transaction Drafted"),
            APPROVED => String::from("Transaction Approved"),
            REJECTED => String::from("Transaction Rejected"),
            INSUFFICIENT => String::from("Insufficient Funds"),
            TRANSFERED => String::from("Funds Transfered"),
            REVERSED => String::from("Transaction Reversed"),
        }
    }
}

#[derive(Debug)]
pub enum ContractStatus {
    DRAFT,
    APPROVED,
    REJECTED,
    ACTIVE,
    CLOSED,
}

impl ContractStatus {
    pub fn to_string(&self) -> String {
        use ContractStatus::{ACTIVE, APPROVED, CLOSED, DRAFT, REJECTED};
        match self {
            DRAFT => String::from("Draft"),
            APPROVED => String::from("Approved"),
            REJECTED => String::from("Rejected"),
            ACTIVE => String::from("Active"),
            CLOSED => String::from("Closed"),
        }
    }
}
