// Main application entry point
mod transaction;
mod tracker;
mod ui;

use tracker::FinanceTracker;
use ui::{display_menu, handle_menu_choice, get_user_input};

// Main function - program entry point
fn main() {
    let mut tracker = FinanceTracker::new();  // Mutable variable

    // Try to load existing data on startup
    if let Err(_) = tracker.load_from_file("finance_data.txt") {
        println!("No existing data file found. Starting fresh!");
    }

    // Main application loop
    loop {
        display_menu();

        let choice = get_user_input("");

        // Handle menu choice - exit if returns false
        if !handle_menu_choice(&choice, &mut tracker) {
            break;
        }
    }
}