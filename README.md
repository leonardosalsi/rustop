# Rustop

![Screenshot_20230908_102831](https://github.com/leonardosalsi/rustop/assets/55445584/9890f5e2-d7bd-4a11-a072-17c2cedfffd5)
## Overview
Rustop is a lightweight and efficient system monitoring tool, designed as an alternative to Linux's `top` command. Built with Rust and Slint, this project aims to provide real-time updates on system performance and resource usage.

## Features
- Real-time CPU, memory, and disk usage statistics
- Interactive user interface
- Multi-threaded measurements for accurate data
- Easily extensible and customizable

## Installation

### Prerequisites
- Rust programming language
- Slint library

### Steps
1. Clone the repository:<br>
```git clone https://github.com/leonardosalsi/rustop.git```
2. Navigate to the project directory:<br>
```cd rustop```
3. Build the project:<br>
```cargo build --release```
4. Run the executable:<br>
```./target/release/rustop```

## Usage
Simply run the `rustop` executable to start monitoring your system. Use keyboard shortcuts to interact with the interface.

## Contributing
We welcome contributions! Feel free to open an issue or submit a pull request.

## License
This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
