from gui import Tetris
from tkinter import TclError


def main():
    gui = Tetris()

    try:
        while gui.state():
            gui.update()
            gui.update_idletasks()
    except TclError:
        pass


if __name__ == "__main__":
    main()
