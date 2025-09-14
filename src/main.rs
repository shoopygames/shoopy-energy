mod address;
mod miner;
mod parser;
mod state;
mod ui;

use std::env;

fn run(address: &str) {
    ui::print_header(address);
    miner::run_miner(address);
}

fn main() {
    ui::enable_ansi_support();

    // Try to get the first argument, otherwise prompt user
    let mut address = env::args().nth(1).unwrap_or_else(|| ui::login_header());

    // Trim & validate address, fallback to login prompt if invalid
    if address::is_valid_address(address.trim()).is_err() {
        address = ui::login_header();
    }

    run(address.trim());
}
