use std::io::{BufRead, BufReader, Write};
use std::process::{Command, Stdio};

use crate::parser::{parse_float_or_zero, RE_CPU, RE_DIFF, RE_SPEED};
use crate::state::{ACCEPTED_SHARES, CPU_THREADS, DIFFICULTY, TOTAL_HPS, TOTAL_SHARES};
use crate::ui;

use std::path::PathBuf;
use std::env;



/// Check if shoopy-rig binary, config and (on Windows) driver exist
pub fn check_miner_files() -> bool {
    // Resolve base directory of the executable
    let exe_dir = env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|p| p.to_path_buf()))
        .unwrap_or_else(|| PathBuf::from("."));

    // Paths relative to exe_dir
    let bin_path = if cfg!(target_os = "windows") {
        exe_dir.join("miner/shoopy-rig.exe")
    } else {
        exe_dir.join("miner/shoopy-rig")
    };
    let config_path = exe_dir.join("miner/shoopy-rig.json");

    let mut missing_files = Vec::new();

    if !bin_path.exists() {
        missing_files.push(bin_path);
    }
    if !config_path.exists() {
        missing_files.push(config_path);
    }

    // Windows-only extra file check
    #[cfg(target_os = "windows")]
    {
        let driver_path = exe_dir.join("miner/WinRing0x64.sys");
        if !driver_path.exists() {
            missing_files.push(driver_path);
        }
    }

    if !missing_files.is_empty() {
        println!("❌ Shoopy miner is not ready.");
        println!("👉 Please disable your Antivirus (AV) temporarily and re-extract files.");
        println!("   Missing files:");
        for file in missing_files {
            println!("   - {}", file.display());
        }
        return false;
    }

    true
}




pub fn run_miner(address: &str) {
    if !check_miner_files() {
        return;
    }

    // Resolve base directory of the current exe
    let exe_dir = env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|p| p.to_path_buf()))
        .unwrap_or_else(|| PathBuf::from("."));

    // Miner binary path
    let miner_path = if cfg!(target_os = "windows") {
        exe_dir.join("miner/shoopy-rig.exe")
    } else {
        exe_dir.join("miner/shoopy-rig")
    };

    // Config path
    let config_path = exe_dir.join("miner/shoopy-rig.json");

    let mut child = Command::new(&miner_path)
        .args(&[
            "-c",
            config_path.to_str().unwrap(),
            "--no-color",
            "-o",
            "energy.shoopy.ir:3333",
            "-u",
            address,
            "-p", "minter",
        ])
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to start shoopy-rig");

    let stdout = child.stdout.take().expect("No stdout from shoopy-rig");
    let reader = BufReader::new(stdout);

    for line in reader.lines() {
        if let Ok(line) = line {
            std::io::stdout().flush().unwrap();
            if line.contains("new job") {
                if let Some(caps) = RE_DIFF.captures(&line) {
                    let diff: usize = caps[1].parse().unwrap();
                    DIFFICULTY.with(|c| *c.borrow_mut() = diff);
                    print!("⛏️");
                    std::io::stdout().flush().unwrap();
                    ui::update_header(address);
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
                if let Some(caps) = RE_DIFF.captures(&line) {
                    let diff: u64 = caps[1].parse().unwrap();
                    TOTAL_SHARES.with(|c| *c.borrow_mut() += diff);
                }
                ui::update_header(address);
            } else if line.contains("cpu") {
                if let Some(caps) = RE_CPU.captures(&line) {
                    let threads: usize = caps[1].parse().unwrap();
                    CPU_THREADS.with(|c| *c.borrow_mut() = threads);
                    print!("📡");
                    ui::update_header(address);
                }
            } else if line.contains("miner") && line.contains("speed") {
                if let Some(caps) = RE_SPEED.captures(&line) {
                    let total: f64 = parse_float_or_zero(&caps[1]);
                    TOTAL_HPS.with(|c| *c.borrow_mut() = total);
                    ui::update_header(address);
                }
            }
        }
    }
}
