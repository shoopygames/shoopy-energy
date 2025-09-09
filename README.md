# 🦉 Shoopy-Minter

Shoopy-Minter is a Rust-based mining interface for the Shoopy Treasure Hunt Game. It connects to the [Shoopy-Rig](https://github.com/shoopygames/shoopy-rig) (a modified version of XMRig) to mine RandomX-based energy and tracks in-game progress in real-time.

## Features

- Cross-platform support (Windows & Linux)
- Real-time mining statistics:
  - Hash rate
  - CPU threads
  - Difficulty
  - Accepted shares
  - Total shares mined
- Supports Shoopy Ganjdari addresses
- ANSI-colored console output
- Simple interface to start mining with a single command

## Installation

1. Clone the repository:

```bash
git clone https://github.com/shoopygames/shoopy-minter.git
cd shoopy-minter
```

2. Build the project:

```bash
cargo build --release
```

3. Download or place the Shoopy-Rig binary (`shoopy-rig` for Linux or `shoopy-rig.exe` for Windows) in the `bin/` directory.

## Usage

```bash
./shoopy-minter
```

You will be prompted to enter your Shoopy Ganjdari address. The miner will then start and display real-time stats.

## Screenshots

```
╔═════════════════════════════════════════════════════════════════╗
║      🦉 Shoopy Treasure Hunt Miner ⛏️ — Harvesting Energy ⚡     ║
╚═════════════════════════════════════════════════════════════════╝
🏛️ SH77xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
🧩 RandomX | 🔌 2 | 🔥 100.00K | 🚀 1.25KH/s | 🧮 777.70KH | ⚡ 77
🛰️⛏️🟢📡⛏️⚡⚡⛏️⚡⛏️⛏️⚡⚡⚡⛏️⚡⛏️⚡⚡⛏️⛏️⛏️⛏️⚡⛏️⛏️⛏️⛏️
⛏️⛏️⚡⚡⚡⛏️⛏️⚡⚡⛏️⚡⛏️⛏️⚡⛏️⚡⛏️⚡⚡⚡⛏️⛏️⚡⛏️⛏️⛏️⛏️⛏️
⛏️⚡⚡⛏️⚡⚡⛏️⚡⛏️⚡⛏️⚡⚡⛏️⚡⚡⛏️⚡⛏️⚡⛏️⚡⚡⚡⚡⚡⛏️⛏️
⚡⛏️⚡⛏️⚡⛏️⚡⚡⛏️⛏️⛏️⚡⚡⚡⛏️⛏️⛏️⛏️⛏️⚡⚡⚡⚡⚡⚡⛏️⛏️⚡
⚡⚡⛏️⛏️⛏️⛏️⚡⚡⚡⛏️⚡⛏️⚡⛏️⚡⛏️⚡⛏️⛏️⛏️⚡⛏️⛏️⛏️⚡⚡⛏️⛏️
⛏️⛏️⚡⚡⛏️⛏️⛏️⚡⛏️⛏️⛏️⚡⚡⚡⛏️⛏️⛏️⚡⚡⛏️⛏️⚡⛏️⛏️⚡⛏️⚡⛏️
```

## Contributing

- Fork the repository
- Make your changes
- Open a pull request

## License

Shoopy-Minter is licensed under MIT License. See [LICENSE](LICENSE) for details.

## Disclaimer

Shoopy-Minter is a tool for the Shoopy Treasure Hunt Game. It is designed for educational and in-game purposes. Make sure to follow the Shoopy game rules when using this software.

