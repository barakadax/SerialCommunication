# Serial communication

Example code how to transfer data over serial communication, both read and write.<br>
Was written and tested in Ubuntu 22.04 with Python 3.10.12

## Install:
```shell
sudo apt install socat
pip install pyserial
pip install pyserial-asyncio
```

## How to compile C++:
```shell
g++ -o a write.cpp
```

## How to run:
### 1:
Create bidirectional socket cable communication:
```shell
socat -d -d pty,raw,echo=0 pty,raw,echo=0
```
It will create 2 address, when writing to any of those you can read from the other.

### 2:
Run any of the readers.

### 3:
Run any of the writers.

## Documentation:
<a href="https://pyserial.readthedocs.io/en/latest/#">PySerial</a><br>
<a href="https://pyserial-asyncio.readthedocs.io/en/latest/index.html">Asyncio PySerial</a>