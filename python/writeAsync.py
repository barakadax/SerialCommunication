import asyncio
import serial_asyncio
from absAsync import serial_protocol

if __name__ == '__main__':
    loop = asyncio.get_event_loop()
    coro = serial_asyncio.create_serial_connection(loop, serial_protocol, '/dev/pts/2', baudrate=115_200)
    _, protocol = loop.run_until_complete(coro)

    while True:
        msg = input('Input: ')
        loop.run_until_complete(protocol.send(msg))
