// User interface and input handling
use std::io::{self, Write};
use crate::tracker::FinanceTracker;
use crate::transaction::TransactionType;

// Helper function for user input with prompt
pub fn get_user_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();  // Ensure prompt displays before input

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}

// Input validation loop for positive numbers
pub fn get_amount_input(prompt: &str) -> f64 {
    loop {
        let input = get_user_input(prompt);
        match input.parse::<f64>() {
            Ok(amount) if amount > 0.0 => return amount,
            _ => println!("Please enter a valid positive number."),
        }
    }
}

// Display main menu options
pub fn display_menu() {
    println!("\n--- Personal Finance Tracker ---");
    println!("1. Add Income");
    println!("2. Add Expense");
    println!("3. View All Transactions");
    println!("4. Show Summary");
    println!("5. Show Category Breakdown");
    println!("6. Save Data");
    println!("7. Load Data");
    println!("8. Exit");
    print!("Choose an option: ");
    io::stdout().flush().unwrap();
}

// Handle menu choices - returns true to continue, false to exit
pub fn handle_menu_choice(choice: &str, tracker: &mut FinanceTracker) -> bool {
    match choice {
        "1" => {
            // Add income transaction
            let description = get_user_input("Enter income description: ");
            let amount = get_amount_input("Enter amount: $");
            tracker.add_transaction(description, amount, "Income".to_string(), TransactionType::Income);
        }
        "2" => {
            // Add expense transaction with category selection
            let description = get_user_input("Enter expense description: ");
            let amount = get_amount_input("Enter amount: $");

            println!("\nAvailable categories:");
            // Convert HashMap keys to Vec for indexed access
            let categories: Vec<&String> = tracker.categories().keys().collect();
            for (i, category) in categories.iter().enumerate() {
                println!("{}. {}", i + 1, category);
            }

            let input = get_user_input("Enter category number or name: ");
            // Parse input as a number or use as the category name
            let category = if let Ok(num) = input.parse::<usize>() {
                if num > 0 && num <= categories.len() {
                    categories[num - 1].clone()
                } else {
                    println!("Invalid number, using input as category name.");
                    input
                }
            } else {
                input
            };

            tracker.add_transaction(description, amount, category, TransactionType::Expense);
        }
        "3" => tracker.display_transactions(),
        "4" => tracker.show_summary(),
        "5" => tracker.show_category_breakdown(),
        "6" => {
            // Error handling for file save
            if let Err(e) = tracker.save_to_file("finance_data.txt") {
                println!("Error saving file: {}", e);
            }
        }
        "7" => {
            // Error handling for file load
            if let Err(e) = tracker.load_from_file("finance_data.txt") {
                println!("Error loading file: {}", e);
            }
        }
        "8" => {
            println!("Goodbye!");
            return false;  // Signal to exit
        }
        _ => println!("Invalid option. Please try again."),
    }
    true  // Continue running
}