mod ui;
mod state;
mod miner;
mod parser;
mod address;

fn main() {
    ui::enable_ansi_support();

    let address = ui::login_header();
    ui::print_header(&address);

    miner::run_miner(&address);
}
