use regex::Regex;
use std::cell::RefCell;
use std::io::{BufRead, BufReader, Write};
use std::process::{Command, Stdio};

// Shared mutable state using RefCell
thread_local! {
    static DIFFICULTY: RefCell<usize> = RefCell::new(0);
    static CPU_THREADS: RefCell<usize> = RefCell::new(0);
    static TOTAL_HPS: RefCell<f64> = RefCell::new(0.0);
    static ACCEPTED_SHARES: RefCell<usize> = RefCell::new(0);
    static TOTAL_SHARES: RefCell<u64> = RefCell::new(0);
}

#[cfg(windows)]
fn enable_ansi_support() {
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
fn enable_ansi_support() {}

#[cfg(not(any(target_os = "linux", windows)))]
fn enable_ansi_support() {}

const HEAD_1: &str = "╔═════════════════════════════════════════════════════════════════╗";
const HEAD_2: &str = concat!(
    "║      🦉 Shoopy Energy Minter ",
    env!("CARGO_PKG_VERSION"),
    " ⛏️ — Harvesting Energy ⚡      ║"
);
const HEAD_3: &str = "╚═════════════════════════════════════════════════════════════════╝";

fn login_header() -> String {
    print!("\x1B[2J\x1B[H");
    print!("\x1b[?25l");
    println!("{}", &HEAD_1);
    println!("{}", &HEAD_2);
    println!("{}", &HEAD_3);
    std::io::stdout().flush().unwrap();
    let address = ask_for_address_loop();
    address
}
fn print_header(address: &str) {
    // Clear screen and hide cursor
    print!("\x1B[2J\x1B[H");
    print!("\x1b[?25l");
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

fn update_header(address: &str) {
    print!("\x1b[s"); // save cursor
    print!("\x1b[H"); // move to top
    let diff = DIFFICULTY.with(|c| *c.borrow());
    let cpu_threads = CPU_THREADS.with(|c| *c.borrow());
    let total_hps = TOTAL_HPS.with(|c| *c.borrow());
    let accepted_shares = ACCEPTED_SHARES.with(|c| *c.borrow());
    let total_shares = TOTAL_SHARES.with(|c| *c.borrow());
    println!("{}", &HEAD_1);
    println!("{}", &HEAD_2);
    println!("{}", &HEAD_3);
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
    print!("\x1b[?25l");
    std::io::stdout().flush().unwrap();
}
fn parse_float_or_zero(s: &str) -> f64 {
    if s == "n/a" {
        0.0
    } else {
        s.parse::<f64>().unwrap_or(0.0)
    }
}

/// Validate ganjdari address
fn is_valid_address(addr: &str) -> Result<&str, &'static str> {
    if !addr.starts_with("SH77")
        || addr.len() != 64
        || !addr.chars().all(|c| c.is_ascii_alphanumeric())
    {
        return Err("Treasury Address is invalid. Please enter the correct one or create one here.\r\nhttps://shoopy.ir/");
    }
    Ok(addr)
}

/// Ask user for a valid address using a loop
fn ask_for_address_loop() -> String {
    loop {
        println!("Please enter your Shoopy Treasury Address: ");
        print!("🏛️ ");
        std::io::stdout().flush().unwrap();

        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        let address = input.trim().to_string();

        match is_valid_address(&address) {
            Ok(_) => return address,
            Err(e) => println!("❌ {}", e),
        }
    }
}

fn format_metric_prefix(value: f64) -> String {
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

fn main() {
    enable_ansi_support();
    let address = login_header();
    print_header(&address);

    let mut miner_path = "";
    if cfg!(target_os = "windows") {
        miner_path = "./miner/shoopy-rig.exe";
    }
    if cfg!(target_os = "linux") {
        miner_path = "./miner/shoopy-rig";
    }

    let mut child = Command::new(miner_path)
        .args(&["-c","./miner/shoopy-rig.json","--no-color","-o", "energy.shoopy.ir:3333", "-u", &address])
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to start shoopy-rig");

    let stdout = child.stdout.take().expect("No stdout from shoopy-rig");
    let reader = BufReader::new(stdout);

    let re_diff = Regex::new(r"diff (\d+)").unwrap();
    let re_cpu = Regex::new(r"\((\d+)\s+threads\)").unwrap();
    let re_speed = Regex::new(
                r"speed\s+10s/60s/15m\s+(\d+\.?\d*|n/a)\s+(\d+\.?\d*|n/a)\s+(\d+\.?\d*|n/a)\s+H/s\s+max\s+(\d+\.?\d*|n/a)"
    ).unwrap();

    for line in reader.lines() {
        if let Ok(line) = line {
            std::io::stdout().flush().unwrap();
            if line.contains("new job") {
                if let Some(caps) = re_diff.captures(&line) {
                    let diff: usize = caps[1].parse().unwrap();
                    DIFFICULTY.with(|c| *c.borrow_mut() = diff);
                    print!("⛏️");
                    std::io::stdout().flush().unwrap();
                    update_header(&address);
                }
            } else if line.contains("randomx") && line.contains("allocated") {
                print!("🟢");
                std::io::stdout().flush().unwrap();
            } else if line.contains("net")
                && (line.contains("no active pools, stop mining")
                    || line.contains("connection refused")
                    || line.contains("temporary failure")
                    || line.contains("error"))
            {
                print!("❌");
                std::io::stdout().flush().unwrap();
            } else if line.contains("net") && line.contains("use pool") {
                print!("🛰️");
                std::io::stdout().flush().unwrap();
            } else if line.contains("accepted") {
                ACCEPTED_SHARES.with(|c| *c.borrow_mut() += 1);
                print!("⚡");
                std::io::stdout().flush().unwrap();
                if let Some(caps) = re_diff.captures(&line) {
                    let diff: u64 = caps[1].parse().unwrap();
                    TOTAL_SHARES.with(|c| *c.borrow_mut() += diff);
                }
                update_header(&address);
            } else if line.contains("cpu") {
                if let Some(caps) = re_cpu.captures(&line) {
                    let threads: usize = caps[1].parse().unwrap();
                    CPU_THREADS.with(|c| *c.borrow_mut() = threads);
                    print!("📡");
                    update_header(&address);
                }
            } else if line.contains("miner") && line.contains("speed") {
                if let Some(caps) = re_speed.captures(&line) {
                    let total: f64 = parse_float_or_zero(&caps[1]);
                    TOTAL_HPS.with(|c| *c.borrow_mut() = total);
                    update_header(&address);
                }
            }
        }
    }
}
