// Transaction data structures and types
#[derive(Debug, Clone)]
pub struct Transaction {
    pub id: u32,
    pub description: String,
    pub amount: f64,
    pub category: String,
    pub transaction_type: TransactionType,
}

#[derive(Debug, Clone)]
pub enum TransactionType {
    Income,
    Expense,
}

impl Transaction {
    // Constructor for creating new transactions
    pub fn new(id: u32, description: String, amount: f64, category: String, transaction_type: TransactionType) -> Self {
        Transaction {
            id,
            description,
            amount,
            category,
            transaction_type,
        }
    }

    // Format transaction for display
    pub fn format_for_display(&self) -> String {
        let type_symbol = match self.transaction_type {
            TransactionType::Income => "+",
            TransactionType::Expense => "-",
        };
        format!("ID: {} | {}{:.2} | {} | {}",
                self.id, type_symbol, self.amount,
                self.category, self.description)
    }

    // Format transaction for file storage
    pub fn format_for_file(&self) -> String {
        let type_str = match self.transaction_type {
            TransactionType::Income => "Income",
            TransactionType::Expense => "Expense",
        };
        format!("{},{},{:.2},{},{}",
                self.id, type_str, self.amount,
                self.category, self.description)
    }
}