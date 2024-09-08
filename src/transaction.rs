/// Represents a transaction in the blockchain.
pub struct Transaction {
    pub sender: String,   // The address of the sender.
    pub receiver: String, // The address of the receiver.
    pub amount: u64,      // The amount of currency being transferred.
}

impl Transaction {
    /// Creates a new transaction with the given sender, receiver, and amount.
    pub fn new(sender: String, receiver: String, amount: u64) -> Transaction {
        Transaction { sender, receiver, amount }
    }
}