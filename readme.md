# Serial communication
Example code how to transfer data over serial communication on Linux, both read and write.

## Install:
```shell
sudo apt install socat
pip install pyserial
pip install pyserial-asyncio
```

## How to compile C++:
```shell
// All
make

// Single file
g++ -o <output file name> <file to compile>.cpp
```

## How to run Rust:
```shell
cd rust
cargo run --bin <file to run>
```

## How to run:
All programs accept the serial port path as a command-line argument.

### Automated Testing
You can run a full cross-test of all C++, Rust, and Python implementations using the provided script:
```shell
python3 test_all.py
```
This script will set up virtual ports using `socat` and verify communication across all reader/writer combinations (ensure C++ programs are compiled first).

### Manual Running:
#### 1. Create bidirectional socket cable communication:
```shell
socat -d -d pty,raw,echo=0 pty,raw,echo=0
```
It will create 2 addresses (e.g., `/dev/pts/12` and `/dev/pts/13`).

#### 2. Run a reader:
```shell
# C++
./cpp/read_sync /dev/pts/12

# Python
python3 python/read.py /dev/pts/12

# Rust
cargo run --bin read_sync -- /dev/pts/12
```

#### 3. Run a writer:
```shell
# C++
./cpp/write_sync /dev/pts/13

# Python
python3 python/write.py /dev/pts/13

# Rust
cargo run --bin write_sync -- /dev/pts/13
```

## TODO:
<ul>
    <li>Forth</li>
    <li>Rexx</li>
    <li>Focal</li>
    <li>Rapira</li>
    <li>Modula</li>
    <li>Prolog</li>
    <li>Cobol</li>
    <li>Ruby</li>
    <li>ASM X86</li>
</ul>
