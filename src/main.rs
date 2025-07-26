// Main application entry point
mod transaction;
mod tracker;
mod ui;

use tracker::FinanceTracker;
use ui::{display_menu, handle_menu_choice, get_user_input};

// Main function - program entry point
fn main() {
    let mut tracker = FinanceTracker::new();  // Mutable variable

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