from json import JSONDecodeError
from tkinter import *
import asyncio
import websockets
import json

inputList = asyncio.Queue()


class Tetris(Tk):
    WindowX = 520
    WindowY = 936
    Top = 100
    Left = 100
    Right = WindowX - 100
    Bottom = WindowY - 100

    EmptyColor = '#313456'
    FillColor = '#f3e9e4'
    TextColor = '#DDDDDD'
    BackgroundColor = '#313456'
    GridColor = '#000000'

    BorderThickness = 5
    GridThickness = 1
    GridWidth = 32

    def __init__(self):
        print("init!")
        super().__init__()
        self.title("Tetris: IVFISH")

        self.canvas = None
        self.init_display()
        self.bind("<Key>", lambda event: keyPressed(event))  # keyboard inputs

    def init_display(self):
        self.geometry(f"{self.WindowX}x{self.WindowY}")
        self.canvas = Canvas(self, width=self.WindowX, height=self.WindowY, background=self.BackgroundColor)
        self.canvas.pack()

        t, b, l, r = self.Top, self.Bottom, self.Left, self.Right

        def initGrid() -> None:
            # Setting borders
            c = self.canvas.create_line
            thick = self.BorderThickness // 2
            c(l - thick, t - thick, l - thick, b + thick, width=self.BorderThickness, tag='border', fill=self.GridColor)
            c(l - thick, t - thick, r + thick, t - thick, width=self.BorderThickness, tag='border', fill=self.GridColor)
            c(r + thick, t - thick, r + thick, b + thick, width=self.BorderThickness, tag='border', fill=self.GridColor)
            c(r + thick, b + thick, l - thick, b + thick, width=self.BorderThickness, tag='border', fill=self.GridColor)

        def initRectangles() -> None:
            # Set rectangles for each grid
            rect = self.canvas.create_rectangle
            for row, y in enumerate(range(b, t, -self.GridWidth)):  # iterate vertically
                for col, x in enumerate(range(l, r, self.GridWidth)):  # iterate horizontally
                    rect(x, y, x + self.GridWidth, y - self.GridWidth, fill=self.EmptyColor, tag=f"{row},{col}")

        def init_text() -> None:
            c = self.canvas.create_text

            c(440, 75, font=('Courier', 14, 'bold'), text=f'Queue: ', tags="QueueText", fill=self.TextColor)
            c(60, 75, font=('Courier', 14, 'bold'), text="None", tags="HoldText", fill=self.TextColor)

        initGrid()
        initRectangles()
        init_text()

    def updateBoard(self, newBoard: list[list[bool]]):
        for row in range(23):
            for col in range(10):
                newColor = self.FillColor if newBoard[row][col] else self.EmptyColor
                self.canvas.itemconfigure(f"{row},{col}", fill=newColor)

    def updateQueue(self, newQueue: str):
        self.canvas.itemconfigure("QueueText", text=newQueue)

    def updateHold(self, newHold: str):
        self.canvas.itemconfigure("HoldText", text=newHold)

    def updateStats(self):
        # stats not done in rust yet
        pass

    def update_display(self, newBoard: list[list[bool]] = None, newQueue: str = None, newHold: str = None):
        if newBoard is not None:
            self.updateBoard(newBoard)

        if newQueue is not None:
            self.updateQueue(newQueue)

        if newHold is not None:
            self.updateHold(newHold)

    async def display(self):
        try:
            while self.state():
                self.update()
                self.update_idletasks()
                await asyncio.sleep(0.05)
        except TclError:
            pass


def keyPressed(event):
    global newInput, nextInput

    keySymToCommand = {
        'Left': "MoveLeft",
        'Right': "MoveRight",
        'Down': "MoveDown",
        'space': "HardDrop",
        'x': "RotateCW",
        'z': "RotateCCW",
        'a': "Rotate180",
        'c': "HoldPiece"
    }

    try:
        inputList.put_nowait(keySymToCommand[event.keysym])
    except KeyError:
        pass


async def handler(websocket):
    print("handle")
    async for message in websocket:
        print(message)

        try:

            msg = json.loads(message)
            print(msg)

            if isinstance(msg, list): # board
                print("update board")
                a.updateBoard(msg)
                nextInput = await inputList.get()
                response = {
                    'eshan': nextInput
                }
                await websocket.send(json.dumps(response))

            if isinstance(msg, int): # op codes
                pass

        except JSONDecodeError:
            a.updateQueue(message)



async def serverLoop():
    async with websockets.serve(handler, "localhost", 5678, ping_interval=None):
        await asyncio.Future()  # run forever


if __name__ == "__main__":
    a = Tetris()
    x = asyncio.ensure_future(a.display())
    y = asyncio.ensure_future(serverLoop())

    loop = asyncio.get_event_loop()
    loop.run_forever()
