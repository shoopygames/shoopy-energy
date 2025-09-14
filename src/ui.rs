use crate::state::{ACCEPTED_SHARES, CPU_THREADS, DIFFICULTY, TOTAL_HPS, TOTAL_SHARES};
use std::io::Write;

#[cfg(windows)]
pub fn enable_ansi_support() {
    use std::ptr::null_mut;
    use winapi::um::consoleapi::GetConsoleMode;
    use winapi::um::consoleapi::SetConsoleMode;
    use winapi::um::processenv::GetStdHandle;
    use winapi::um::winbase::STD_OUTPUT_HANDLE;
    use winapi::um::wincon::ENABLE_PROCESSED_OUTPUT;
    use winapi::um::wincon::ENABLE_VIRTUAL_TERMINAL_PROCESSING;

    unsafe {
        let handle = GetStdHandle(STD_OUTPUT_HANDLE);
        if handle != null_mut() {
            let mut mode = 0;
            if GetConsoleMode(handle, &mut mode) != 0 {
                SetConsoleMode(
                    handle,
                    mode | ENABLE_PROCESSED_OUTPUT | ENABLE_VIRTUAL_TERMINAL_PROCESSING,
                );
            }
        }
    }
}
#[cfg(target_os = "linux")]
pub fn enable_ansi_support() {}

#[cfg(not(any(target_os = "linux", windows)))]
pub fn enable_ansi_support() {}

const HEAD_1: &str = "╔═════════════════════════════════════════════════════════════════╗";
const HEAD_2: &str = concat!(
    "║      🦉 Shoopy Energy Minter ",
    env!("CARGO_PKG_VERSION"),
    " ⛏️ — Producing Energy ⚡      ║"
);
const HEAD_3: &str = "╚═════════════════════════════════════════════════════════════════╝";

pub fn login_header() -> String {
    print!("\x1B[2J\x1B[H");
    // print!("\x1b[?25l");
    println!("{}", &HEAD_1);
    println!("{}", &HEAD_2);
    println!("{}", &HEAD_3);
    std::io::stdout().flush().unwrap();
    let address = crate::address::ask_for_address_loop();
    address
}

pub fn print_header(address: &str) {
    // Clear screen and hide cursor
    print!("\x1B[2J\x1B[H");
    // print!("\x1b[?25l");
    print!("\x1b[1;1H"); // line 5 col 1
    print!("\x1b[2K"); // clear line
    print!("\x1b[2;1H"); // line 5 col 1
    print!("\x1b[2K"); // clear line
    print!("\x1b[3;1H"); // line 5 col 1
    print!("\x1b[2K"); // clear line
    print!("\x1b[4;1H"); // line 5 col 1
    print!("\x1b[2K"); // clear line
    print!("\x1b[5;1H"); // line 5 col 1
    print!("\x1b[2K"); // clear line
    print!("\x1b[H");
    println!("{}", &HEAD_1);
    println!("{}", &HEAD_2);
    println!("{}", &HEAD_3);
    println!("🏛️ {}", address);
    println!(
        "🧩 {} | 🔌 {} | 🔥 {} | 🚀 {}H/s | 🧮 {}H | ⚡ {}",
        "RandomX", 0, 0, 0, 0, 0
    );
    std::io::stdout().flush().unwrap();
}

pub fn update_header(address: &str) {
    let diff = DIFFICULTY.with(|c| *c.borrow());
    let cpu_threads = CPU_THREADS.with(|c| *c.borrow());
    let total_hps = TOTAL_HPS.with(|c| *c.borrow());
    let accepted_shares = ACCEPTED_SHARES.with(|c| *c.borrow());
    let total_shares = TOTAL_SHARES.with(|c| *c.borrow());
    print!("\x1b[s"); // save cursor
    print!("\x1b[H"); // move to top
    print!("\x1b[1;1H"); // line 5 col 1
    print!("\x1b[2K"); // clear line
    println!("{}", &HEAD_1);
    print!("\x1b[2;1H"); // line 5 col 1
    print!("\x1b[2K"); // clear line
    println!("{}", &HEAD_2);
    print!("\x1b[3;1H"); // line 5 col 1
    print!("\x1b[2K"); // clear line
    println!("{}", &HEAD_3);
    print!("\x1b[4;1H"); // line 5 col 1
    print!("\x1b[2K"); // clear line
    println!("🏛️ {}", address);
    print!("\x1b[5;1H"); // line 5 col 1
    print!("\x1b[2K"); // clear line
    
    println!(
        "🧩 {} | 🔌 {} | 🔥 {} | 🚀 {}H/s | 🧮 {}H | ⚡ {}",
        "RandomX",
        cpu_threads,
        format_metric_prefix(diff as f64),
        format_metric_prefix(total_hps.try_into().unwrap()),
        format_metric_prefix(total_shares as f64),
        accepted_shares,
    );

    print!("\x1b[u"); // restore cursor
    // print!("\x1b[?25l");
    std::io::stdout().flush().unwrap();
}

pub fn format_metric_prefix(value: f64) -> String {
    // Metric prefixes from deca (10^1) to quetta (10^30)
    let prefixes = [
        (30, "Q"), // quetta
        (27, "R"), // ronna
        (24, "Y"), // yotta
        (21, "Z"), // zetta
        (18, "E"), // exa
        (15, "P"), // peta
        (12, "T"), // tera
        (9, "G"),  // giga
        (6, "M"),  // mega
        (3, "K"),  // kilo
    ];

    // Convert to f64 for decimal scaling
    for &(exp, prefix) in prefixes.iter() {
        let scale = 10f64.powi(exp);
        if value >= scale {
            let scaled = value / scale;
            return format!("{:.2}{}", scaled, prefix);
        }
    }

    format!("{:.2}", value)
}
