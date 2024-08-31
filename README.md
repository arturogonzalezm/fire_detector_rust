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
2. If you are using MacOS" run:

```bash
nano ~/.zshrc
```

```bash
export PKG_CONFIG_PATH="/usr/local/opt/llvm/lib/pkgconfig"
export OPENCV_LINK_LIBS="clang"
export LDFLAGS="-L/opt/homebrew/opt/llvm/lib -L/opt/homebrew/opt/llvm/lib/c++ -lunwind"
export CPPFLAGS="-I/opt/homebrew/opt/llvm/include"
export PATH="/opt/homebrew/opt/llvm/bin:$PATH"
export DYLD_LIBRARY_PATH="/Library/Developer/CommandLineTools/usr/lib:$DYLD_LIBRARY_PATH"
export LD_LIBRARY_PATH="/opt/homebrew/Cellar/llvm/18.1.8/lib:$LD_LIBRARY_PATH"
export LIBCLANG_PATH="/Library/Developer/CommandLineTools/usr/lib"
```

```bash
brew reinstall llvm
brew reinstall opencv
```

```bash
source ~/.zshrc
```

```bash
find / -name libclang.dylib 2>/dev/null
find /opt/homebrew/Cellar/llvm/ -name libclang.dylib
otool -L /opt/homebrew/Cellar/llvm/18.1.8/lib/libclang.dylib
```

```bash
cargo clean 
cargo build
```

```bash
Compiling libc v0.2.158
Compiling glob v0.3.1
Compiling memchr v2.7.4
Compiling regex-syntax v0.8.4
Compiling semver v1.0.23
Compiling percent-encoding v2.3.1
Compiling shlex v1.3.0
Compiling dunce v1.0.5
Compiling once_cell v1.19.0
Compiling autocfg v1.3.0
Compiling pkg-config v0.3.30
Compiling vcpkg v0.2.15
Compiling num-traits v0.2.19
Compiling clang-sys v1.8.1
Compiling aho-corasick v1.1.3
Compiling jobserver v0.1.32
Compiling cc v1.1.15
Compiling regex-automata v0.4.7
Compiling clang v2.0.0
Compiling regex v1.10.6
Compiling opencv-binding-generator v0.90.2
Compiling opencv v0.92.2
Compiling fire_detector_rust v0.1.0 (/Users/path/RustroverProjects/fire_detector_rust)
Finished `dev` profile [unoptimized + debuginfo] target(s) in 23.35s
```

