from app import *
from websocket import create_connection
from time import sleep

def main():
    # create connection
    conn = create_connection("ws://127.0.0.1:23512")

    # spawn app
    app = App()
    app.run(conn)

if __name__ == "__main__":
    main()
