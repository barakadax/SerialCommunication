import asyncio

class serial_protocol(asyncio.Protocol):
    def connection_made(self, transport: asyncio.BaseTransport) -> None:
        self.transport = transport

    async def send(self, msg: str) -> None:
        self.transport.write(msg.encode('utf-16'))
        self.transport.serial.flush()

    def data_received(self, data: bytes) -> None:
        print(f'data received: {data.decode("utf-16")}')
        self.pause_reading()

    def connection_lost(self) -> None:
        self.transport.loop.stop()

    def pause_reading(self) -> None:
        self.transport.pause_reading()

    def resume_reading(self) -> None:
        self.transport.resume_reading()
