import json

def as_arr(dct):
    arr = dct["info"]
    return json.loads(arr)
    

class Board:
    def __init__(self, json_board):
        new_board = json.loads(json_board, object_hook=as_arr)
        print(new_board)
        self.board = [
            [bool((col >> row) & 1) 
             for row in range(20)] 
            for col in new_board
        ]
