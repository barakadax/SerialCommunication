import asyncio
import serial_asyncio
from absAsync import serial_protocol

async def reader(loop: asyncio.AbstractEventLoop) -> None:
    _, protocol = await serial_asyncio.create_serial_connection(loop, serial_protocol, '/dev/pts/4', baudrate=115_200)

    while True:
        await asyncio.sleep(0.3)  # read once every 0.3 seconds
        protocol.resume_reading()

if __name__ == '__main__':
    loop = asyncio.get_event_loop()
    loop.run_until_complete(reader(loop))
    loop.close()
