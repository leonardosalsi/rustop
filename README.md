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

## Next Steps (Roadmap)

Here are some features and improvements planned for future releases:

### Short-Term Goals
- Implement network usage monitoring
- Add support for custom themes
- Improve error handling and logging

### Long-Term Goals
- Cross-platform support (Windows, macOS)
- User authentication for secure monitoring
- Develop a centralized management system for anomaly detection across system clusters
- Integrate machine learning algorithms for real-time anomaly detection

### Under Consideration
- Docker container support
- Cloud-based monitoring solutions

## Contributing
If you're interested in contributing to any of these features, or have ideas of your own, feel free to open an issue or submit a pull request.

## License
This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
