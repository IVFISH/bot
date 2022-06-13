from tkinter import *


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
        self.bind("<Key>", lambda event: keyPressed(event))  # keyboard inputs

    def init_display(self):
        self.geometry(f"{self.WindowX}x{self.WindowY}")
        self.canvas = Canvas(self, width=self.WindowX, height=self.WindowY, background=self.BackgroundColor)
        self.canvas.pack()
        # self.title(f"Number {self.NumberInGeneration} in generation {self.Generation}")

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
            for y in range(b, t, -self.GridWidth):  # iterate vertically
                for x in range(l, r, self.GridWidth):  # iterate horizontally
                    rect(x, y, x + self.GridWidth, y - self.GridWidth, fill=self.EmptyColor, tag=f"{x},{y}")

        initGrid()
        initRectangles()

    def updateBoard(self, newBoard: str):
        pass


def keyPressed(event):
    print(event.keysym)
