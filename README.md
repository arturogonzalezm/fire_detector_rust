# Fire Detector in Rust Programming Language

## Description

This project is a fire detector built using the Rust programming language. It is designed to detect fire and send an
alert to a specified email address.

## Features

- Detects fire using a camera
- Capture logs into database for monitoring and analysis purposes

## Installation

To install and run this project, follow these steps:

1. Clone the repository to your local machine.
2. If you are using Linux Ubuntu run:

```bash
nano ~/.bashrc
```

```bash
export LIBCLANG_PATH=/usr/lib/llvm-18/lib
export LLVM_CONFIG_PATH=/usr/bin/llvm-config
echo 'export LIBCLANG_PATH=/usr/lib/llvm-18/lib' >> ~/.bashrc
echo 'export LLVM_CONFIG_PATH=/usr/bin/llvm-config' >> ~/.bashrc

sudo apt-get update
sudo apt-get install llvm-dev
which llvm-config
find /usr -name llvm-config
export LLVM_CONFIG_PATH=/usr/bin/llvm-config
sudo apt-get install libclang-dev
```

```bash
source ~/.bashrc
```

```bash
find /usr -name libclang.so
```

```bash
cargo clean 
cargo build
```

```bash
C/snap/bin/cargo build --color=always --message-format=json-diagnostic-rendered-ansi --package fire_detector_rust --bin fire_detector_rust
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.27s
Process finished with exit code 0
```

