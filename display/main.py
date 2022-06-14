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
        super().__init__()
        self.title("Tetris: IVFISH")

        self.canvas = None
        self.init_display()

        self.bind("<Key>", lambda event: keyPressed(event, self))  # keyboard inputs
        self.bind('<KeyRelease>', lambda event: keyReleased(event, self))
        self.keyHeld = None
        self.DAS = 75

        self.runTime = 0

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

    def updateBoard(self, newBoard: str):

        color_map = {
            "■": self.FillColor,
            "⬚": self.FillColor,
            "□": self.EmptyColor,

            "\n■": self.FillColor,
            "\n⬚": self.FillColor,
            "\n□": self.EmptyColor
        }

        newBoard = newBoard.strip().split(" ")

        for index, item in enumerate(newBoard):
            self.canvas.itemconfigure(f"{22 - index // 10},{index % 10}", fill=color_map[item])

    def updateQueue(self, newQueue: str):
        self.canvas.itemconfigure("QueueText", text=newQueue)

    def updateHold(self, newHold: str):
        self.canvas.itemconfigure("HoldText", text=newHold)

    def updateStats(self):
        # stats not done in rust yet
        pass

    async def display(self):
        try:
            while self.state():
                self.checkDas()

                self.update()
                self.update_idletasks()
                await asyncio.sleep(0.05)
        except TclError:
            pass

    def checkDas(self):
        if not self.keyHeld:
            return

        key, time = self.keyHeld
        time = (self.runTime - time) * 1000
        # if time < self.DAS:
        #     return

        if key == 'Left':
            inputList.put_nowait("DasLeft")
        elif key == 'Right':
            inputList.put_nowait("DasRight")


def keyPressed(event, tetris):
    global newInput, nextInput

    keySymToCommand = {
        'Left': "MoveLeft",
        'Right': "MoveRight",
        'Down': "SoftDrop",
        'space': "HardDrop",
        'x': "RotateCW",
        'z': "RotateCCW",
        'a': "Rotate180",
        'c': "HoldPiece"
    }

    if event.keysym == 'Left' or event.keysym == 'Right':
        tetris.keyHeld = (event.keysym, tetris.runTime)

    try:
        inputList.put_nowait(keySymToCommand[event.keysym])
    except KeyError:
        pass


def keyReleased(event, game):
    if game.keyHeld and event.keysym == game.keyHeld[0]:
        game.keyHeld = None


async def handler(websocket):
    async for msg in websocket:

        if len(msg) > 50:  # board
            Tetris.updateBoard(msg)
            nextInput = await inputList.get()
            response = {'contents': nextInput}
            await websocket.send(json.dumps(response))

        elif len(msg) > 5:
            Tetris.updateQueue(msg)

        else:
            Tetris.updateHold(msg)


async def serverLoop():
    async with websockets.serve(handler, "localhost", 5678, ping_interval=None):
        await asyncio.Future()  # run forever


if __name__ == "__main__":
    Tetris = Tetris()
    x = asyncio.ensure_future(Tetris.display())
    y = asyncio.ensure_future(serverLoop())

    loop = asyncio.get_event_loop()
    loop.run_forever()
