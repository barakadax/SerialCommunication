import sys
import asyncio
import serial_asyncio
from absAsync import serial_protocol

if __name__ == '__main__':
    if len(sys.argv) < 2:
        print(f"Usage: {sys.argv[0]} <port>")
        sys.exit(1)
    port = sys.argv[1]
    loop = asyncio.get_event_loop()
    coro = serial_asyncio.create_serial_connection(loop, serial_protocol, port, baudrate=115_200)
    _, protocol = loop.run_until_complete(coro)

    while True:
        msg = input('Input: ')
        loop.run_until_complete(protocol.send(msg))
