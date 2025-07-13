// Finance tracker core logic
use std::collections::HashMap;
use std::fs;
use std::io;
use crate::transaction::{Transaction, TransactionType};

pub struct FinanceTracker {
    transactions: Vec<Transaction>,           // Vec to store all transactions
    categories: HashMap<String, f64>,         // HashMap: category -> budget limit
    next_id: u32,                            // Simple ID counter
}

impl FinanceTracker {
    // Constructor - creates a new tracker with default budget categories
    pub fn new() -> Self {
        let mut categories = HashMap::new();
        // Initialize default budget limits for each category
        categories.insert("Food".to_string(), 500.0);
        categories.insert("Transportation".to_string(), 200.0);
        categories.insert("Entertainment".to_string(), 150.0);
        categories.insert("Utilities".to_string(), 300.0);
        categories.insert("Other".to_string(), 100.0);

        FinanceTracker {
            transactions: Vec::new(),
            categories,
            next_id: 1,
        }
    }

    // Getter for categories - returns borrowed reference
    pub fn categories(&self) -> &HashMap<String, f64> {
        &self.categories
    }

    // Takes ownership of String parameters - demonstrates Rust ownership
    pub fn add_transaction(&mut self, description: String, amount: f64, category: String, transaction_type: TransactionType) {
        let transaction = Transaction::new(self.next_id, description, amount, category, transaction_type);

        // Move transaction into Vec
        self.transactions.push(transaction);
        self.next_id += 1;
        println!("Transaction added successfully!");
    }

    // Borrows self-immutably - demonstrates references
    pub fn display_transactions(&self) {
        if self.transactions.is_empty() {
            println!("No transactions found.");
            return;
        }

        println!("\n--- All Transactions ---");
        // Iterate over borrowed references to avoid moving data
        for transaction in &self.transactions {
            println!("{}", transaction.format_for_display());
        }
    }

    // Functional programming with iterators and closures
    pub fn calculate_total_income(&self) -> f64 {
        self.transactions.iter()
            .filter(|t| matches!(t.transaction_type, TransactionType::Income))
            .map(|t| t.amount)
            .sum()
    }

    // Similar pattern for expenses
    pub fn calculate_total_expenses(&self) -> f64 {
        self.transactions.iter()
            .filter(|t| matches!(t.transaction_type, TransactionType::Expense))
            .map(|t| t.amount)
            .sum()
    }

    // Expression - simple arithmetic calculation
    pub fn calculate_balance(&self) -> f64 {
        self.calculate_total_income() - self.calculate_total_expenses()
    }

    // Conditional logic for warnings
    pub fn show_summary(&self) {
        let total_income = self.calculate_total_income();
        let total_expenses = self.calculate_total_expenses();
        let balance = self.calculate_balance();

        println!("\n--- Financial Summary ---");
        println!("Total Income: ${:.2}", total_income);
        println!("Total Expenses: ${:.2}", total_expenses);
        println!("Balance: ${:.2}", balance);

        // Conditional warning
        if balance < 0.0 {
            println!("⚠️  Warning: You're spending more than you earn!");
        }
    }

    // HashMap operations and complex expressions
    pub fn show_category_breakdown(&self) {
        let mut category_spending: HashMap<String, f64> = HashMap::new();

        // Calculate spending per category
        for transaction in &self.transactions {
            if matches!(transaction.transaction_type, TransactionType::Expense) {
                let current = category_spending.get(&transaction.category).unwrap_or(&0.0);
                category_spending.insert(transaction.category.clone(), current + transaction.amount);
            }
        }

        println!("\n--- Spending by Category ---");
        // Iterate over HashMap and calculate percentages
        for (category, budget) in &self.categories {
            let spent = category_spending.get(category).unwrap_or(&0.0);
            let percentage = if *budget > 0.0 { (spent / budget) * 100.0 } else { 0.0 };

            print!("{}: ${:.2} / ${:.2} ({:.1}%)", category, spent, budget, percentage);

            // Conditional budget warnings
            if spent > budget {
                println!(" ⚠️  OVER BUDGET!");
            } else if percentage > 80.0 {
                println!(" ⚠️  Near limit");
            } else {
                println!();
            }
        }
    }

    // File I/O with Result error handling
    pub fn save_to_file(&self, filename: &str) -> io::Result<()> {
        let mut content = String::new();
        content.push_str("# Personal Finance Tracker Data\n");
        content.push_str("# Format: ID,Type,Amount,Category,Description\n");

        // String manipulation and formatting
        for transaction in &self.transactions {
            content.push_str(&transaction.format_for_file());
            content.push('\n');
        }

        // File write operation - can return error
        fs::write(filename, content)?;
        println!("Data saved to {}", filename);
        Ok(())
    }

    // File reading with error handling and parsing
    pub fn load_from_file(&mut self, filename: &str) -> io::Result<()> {
        let content = fs::read_to_string(filename)?;
        let mut max_id = 0;

        // Clear existing data before loading
        self.transactions.clear();

        // String parsing with loops and conditionals
        for line in content.lines() {
            if line.starts_with('#') || line.trim().is_empty() {
                continue;
            }

            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() >= 5 {
                // Nested error handling with pattern matching
                if let Ok(id) = parts[0].parse::<u32>() {
                    if let Ok(amount) = parts[2].parse::<f64>() {
                        let transaction_type = if parts[1] == "Income" {
                            TransactionType::Income
                        } else {
                            TransactionType::Expense
                        };

                        let transaction = Transaction::new(
                            id,
                            parts[4..].join(","), // Handle commas in description
                            amount,
                            parts[3].to_string(),
                            transaction_type,
                        );

                        self.transactions.push(transaction);
                        max_id = max_id.max(id);
                    }
                }
            }
        }

        self.next_id = max_id + 1;
        println!("Data loaded from {}", filename);
        Ok(())
    }
}