import sys
import asyncio
import serial_asyncio
from absAsync import serial_protocol

async def reader(loop: asyncio.AbstractEventLoop, port: str) -> None:
    _, protocol = await serial_asyncio.create_serial_connection(loop, serial_protocol, port, baudrate=115_200)

    while True:
        await asyncio.sleep(0.3)
        protocol.resume_reading()

if __name__ == '__main__':
    if len(sys.argv) < 2:
        print(f"Usage: {sys.argv[0]} <port>")
        sys.exit(1)
    port = sys.argv[1]
    loop = asyncio.get_event_loop()
    loop.run_until_complete(reader(loop, port))
    loop.close()
