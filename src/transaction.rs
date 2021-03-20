use std::time::SystemTime;

use sha2::{Digest, Sha256};

#[derive(Debug, Clone)]
pub struct Transaction {
    hash: String,
    source_addr: String,
    destination_addr: String,
    input_amount: f64,
    output_amount: f64,
    timestamp: u128,
}

impl Transaction {
    pub fn new<T1, T2>(
        source_addr: T1,
        destination_addr: T2,
        input_amount: f64,
        output_amount: f64,
    ) -> Self
    where
        T1: Into<String>,
        T2: Into<String>,
    {
        let unix_epoch_time = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap();

        let mut transaction = Self {
            hash: String::new(),
            source_addr: source_addr.into(),
            destination_addr: destination_addr.into(),
            input_amount,
            output_amount,
            timestamp: unix_epoch_time.as_micros(),
        };

        transaction.hash = transaction.calculate_hash();

        transaction
    }

    pub fn calculate_hash(&self) -> String {
        let mut hasher = Sha256::new();

        let data = format!(
            "{}{}{}{}{}",
            self.source_addr,
            self.destination_addr,
            self.input_amount,
            self.output_amount,
            self.timestamp,
        );

        hasher.update(data);
        let hash = hasher.finalize();

        format!("{:X}", hash)
    }

    pub fn hash(&self) -> &String {
        &self.hash
    }
}
