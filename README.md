# 🦉 Shoopy-Energy

Shoopy-Energy is a Rust-based mining interface for the Shoopy Treasure Hunt Game. It connects to the [Shoopy-Rig](https://github.com/shoopygames/shoopy-rig) (a modified version of XMRig) to mine RandomX-based energy and tracks in-game progress in real-time.

## Features

- Cross-platform support (Windows & Linux)
- Real-time mining statistics:
  - Hash rate
  - CPU threads
  - Difficulty
  - Accepted shares
  - Total shares mined
- Supports Shoopy Cell addresses
- ANSI-colored console output
- Simple interface to start mining with a single command

## Installation

1. Clone the repository:

```bash
git clone https://github.com/shoopygames/shoopy-energy.git
cd shoopy-energy
```

2. Build the project:

```bash
cargo build --release
```

3. Download or place the Shoopy-Rig binary (`shoopy-rig` for Linux or `shoopy-rig.exe` for Windows) in the `miner/` directory.

## Usage

```bash
./shoopy-energy
```

You will be prompted to enter your Shoopy Cell address. The miner will then start and display real-time stats.

## Contributing

- Fork the repository
- Make your changes
- Open a pull request

## License

Shoopy-Energy is licensed under MIT License. See [LICENSE](LICENSE) for details.

## Disclaimer

Shoopy-Energy is a tool for the Shoopy Treasure Hunt Game. It is designed for educational and in-game purposes. Make sure to follow the Shoopy game rules when using this software.

