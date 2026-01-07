import asyncio

class serial_protocol(asyncio.Protocol):
    def connection_made(self, transport: asyncio.BaseTransport) -> None:
        self.transport = transport
        self.buffer = b''

    async def send(self, msg: str) -> None:
        self.transport.write(msg.encode('utf-8'))
        if hasattr(self.transport, 'serial'):
            self.transport.serial.flush()

    def data_received(self, data: bytes) -> None:
        self.buffer += data
        try:
            res = self.buffer.decode("utf-8")
            if res:
                print(f'data received: {res}', flush=True)
            self.buffer = b''
        except UnicodeDecodeError:
            pass
        self.pause_reading()

    def connection_lost(self) -> None:
        self.transport.loop.stop()

    def pause_reading(self) -> None:
        self.transport.pause_reading()

    def resume_reading(self) -> None:
        self.transport.resume_reading()
