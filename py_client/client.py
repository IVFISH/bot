from app import *
from websocket import create_connection
from multiprocessing import Process
from time import sleep

def await_and_update(conn, app):
    # awaits the connection and updates the app with a new state
    while True:
        board = Board(conn.recv())
        app.update_board(board)
        time.sleep(0.1)


def display_app(app):
    app.display()

def main():
    # create connection
    conn = create_connection("ws://127.0.0.1:23512")

    # spawn app
    app = App()

    # spawn threads:
    # one thread for updating display frames
    p1 = Process(target=display_app, args=(app,))
    # one thread for awaiting new board states
    p2 = Process(target=await_and_update, args=(conn, app,))
    # run threads
    p1.start()
    p2.start()
    # complete threads
    p1.join()
    p2.join()


if __name__ == "__main__":
    main()
