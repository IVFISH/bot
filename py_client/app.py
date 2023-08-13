import tkinter as tk
from board import *

class App(tk.Tk):
    # 20 buffer on all sides
    # 30 cell size

    def __init__(self):
        super().__init__()
        self.canvas = tk.Canvas(self, width=340, height=730)
        self.canvas.pack()
        self.init_board()

    def update_board(self, board: Board):
        colors = ['#313456', '#53ceab'] # empty fill
        for row in range(20):
            for col in range(10):
                cell_id = self.cells[row][col]
                color = colors[board.board[row][col]]
                self.canvas.itemconfig(cell_id, fill=color)

    def init_board(self):
        self.cells = [
            [self.canvas.create_rectangle(*self.get_coords(row, col))
             for col in range(10)]
            for row in range(20)
        ]

    def get_coords(self, row, col):
        # returns the two corners of the cell
        y1, y2 = (23 - row) * 30, (22 - row) * 30
        x1, x2 = col * 30, col * 30 + 30
        return [x1 + 20, y1 + 20, x2 + 20, y2 + 20]

    def run(self, conn):
        try:
            while self.state():
                board = conn.recv()
                self.update_board(Board(board))
                conn.send(board)

                self.update()
                self.update_idletasks()
        except tk.TclError:
            print("Exiting...")

