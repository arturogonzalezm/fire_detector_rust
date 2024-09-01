# Fire Detector in Rust Programming Language

## Description

This project implements a fire detection system using the Rust programming language. 
It utilises computer vision to detect fire in real-time and sends alerts to a specified email address when a fire is detected.

## Features

- Real-time fire detection using camera input
- Email alerts when fire is detected
- Logging of detection events to a database for monitoring and analysis
- Built with Rust for high performance and safety

## Prerequisites

- Rust programming language (latest stable version)
- Cargo package manager
- LLVM and Clang libraries
- (Add any other specific libraries or tools required)

## Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/arturogonzalezm/fire_detector_rust.git
   cd fire_detector_rust
   ```

2. Set up the required environment variables (for Linux Ubuntu):
   ```bash
   echo 'export LIBCLANG_PATH=/usr/lib/llvm-18/lib' >> ~/.bashrc
   echo 'export LLVM_CONFIG_PATH=/usr/bin/llvm-config' >> ~/.bashrc
   source ~/.bashrc
   ```

3. Install necessary dependencies:
   ```bash
   sudo apt-get update
   sudo apt-get install llvm-dev libclang-dev
   ```

4. Verify the installation:
   ```bash
   which llvm-config
   find /usr -name libclang.so
   ```

5. Build the project:
   ```bash
   cargo clean
   cargo build
   ```
   
6. Test if it's compiled successfully:
   ```bash
   Compiling libc v0.2.158
   Compiling once_cell v1.19.0
   Compiling num-traits v0.2.19
   Compiling clang-sys v1.8.1
   Compiling jobserver v0.1.32
   Compiling cc v1.1.15
   Compiling clang v2.0.0
   Compiling opencv-binding-generator v0.90.2
   Compiling opencv v0.92.2
   Compiling fire_detector_rust v0.1.0 (/home/path/path/fire_detector_rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 43.46s
    ```
   
7. Run the project:
    ```bash
    cargo run
    ```

## Usage

(Add instructions on how to run the program, including any command-line arguments or configuration files needed)

## Configuration

(Explain how to configure the email alerts, adjust detection sensitivity, etc.)

## Database Setup

(Provide instructions on setting up and connecting to the database for logging)

