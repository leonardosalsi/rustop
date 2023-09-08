# RUSTop

![screenshot](https://github.com/leonardosalsi/rustop/assets/55445584/3e9bec4e-2202-48c4-88ca-4be1cd21e317)
## Overview
Rustop is a lightweight and efficient system monitoring tool, designed as an alternative to Linux's `top` command. Built with Rust and Slint, this project aims to provide real-time updates on system performance and resource usage.

## Features
- Real-time CPU, memory,disk usage and network statistics
- Interactive user interface
- Multi-threaded measurements for accurate data
- Easily extensible and customizable

## Installation

### Prerequisites
Dependencies
```
chrono = "0.4.28"
compound_duration = "1.2.1"
serde = "1.0.188"
serde_json = "1.0.105"
slint = "1.2.0"
sysinfo = "0.29.10"
tokio = {version="1.32.0", features = ["full"]}
```

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
