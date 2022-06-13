from game import Tetris

import json
import threading
import asyncio
import websockets


async def handler(websocket):
    async for message in websocket:
        print(message)


async def main():
    display = threading.Thread(target=Tetris, kwargs={})
    display.start()

    async with websockets.serve(handler, "localhost", 23512):
        await asyncio.Future()  # run forever


if __name__ == "__main__":
    asyncio.run(main())
