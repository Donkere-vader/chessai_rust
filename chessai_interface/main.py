#! .env/bin/python3
import tkinter as tk
from tkinter.constants import TOP, LEFT
import chess_ai
from .storage import loads, dumps
from copy import deepcopy
import json
from datetime import datetime as dt
import os
from typing import List, Tuple


UNICODE_CHESS_PIECES = {
    "rook": "♜",
    "knight": "♞",
    "bishop": "♝",
    "king": "♚",
    "queen": "♛",
    "pawn": "♟",
}


PIECE_COLORS = {
    "white": "#5e81ac",
    "black": "#bf616a",
}


class GameWindow(tk.Tk):
    def __init__(self, fen_code, log_file_name, color):
        super().__init__()
        self.title("chess ai")
        self.boards_history = []
        self.board = [[None for _ in range(8)] for _ in range(8)]
        self.board, self.castling = loads(fen_code)
        if color in ["white", "black"]:
            self.color = color
        else:
            raise Exception(f"Invalid color: {color}")
        self.depth = 4
        self.viewing_side = self.color
        self.selected_tile = None
        self.log = []

        if log_file_name is None:
            now = dt.now()
            self.log_file_name = f"logs/game_{now.year}_{now.month}_{now.day}_{now.hour}_{now.minute}_{now.second}.json"
        else:
            self.log_file_name = f"logs/{log_file_name}" if log_file_name.endswith(".json") else f"logs/{log_file_name}.json"
    
            if os.path.isdir("logs") and os.path.exists(f"logs/{self.log_file_name}"):
                with open(f"logs/{self.log_file_name}", 'r') as f:
                    self.log = json.load(f)

        self.initial_draw()
        self.draw()

    def click(self, coord):
        def reset_selected_tile():
            self.selected_tile = None
            self.draw()

        if self.selected_tile is None and self.board[coord[1]][coord[0]] is not None:
            self.selected_tile = coord
            self.draw()
        elif self.selected_tile == coord:
            reset_selected_tile()
        elif self.selected_tile is not None:
            self.move(self.selected_tile, coord)
            reset_selected_tile()
            self.draw()

    def get_highlights(self) -> List[Tuple[int, int]]:
        if len(self.boards_history) == 0: return []
        highligths = []

        old_board, _old_castling = loads(self.boards_history[-1])

        for y in range(8):
            for x in range(8):
                if old_board[y][x] != self.board[y][x]:
                    highligths.append((x, y))

        return highligths

    def move(self, frm, to):
        piece = self.board[frm[1]][frm[0]]
        if piece[1] == 'pawn' and to[1] in [0, 7]:
            piece = (piece[0], 'queen')
        self.boards_history.append(dumps(self.board, self.color, self.castling))
        self.board[to[1]][to[0]] = piece
        self.board[frm[1]][frm[0]] = None
        self.log.append({
            "fen_code": dumps(self.board, self.color, self.castling),
        })
        self.write_log()
        self.draw()
    
    def write_log(self):
        if "logs" not in os.listdir():
            os.mkdir("logs")
        elif not os.path.isdir("logs"):
            raise Exception("Failed to find logs directory")

        with open(self.log_file_name, 'w') as f:
            json.dump(self.log, f, indent=2)

    def undo(self):
        if len(self.boards_history) > 0:
            self.board, self.castling = loads(self.boards_history.pop(len(self.boards_history) - 1))
            del self.log[-1]
        self.draw()

    def toggle_viewing_side(self):
        self.viewing_side = "black" if self.viewing_side == "white" else "white"
        self.draw()

    def initial_draw(self):
        self.tile_buttons = []

        self.frames = {}
        self.frames['chess_board'] = tk.Frame(self)

        for y in range(8):
            button_row = []
            for x in range(8):
                new_button = tk.Button(
                    self.frames['chess_board'],
                    border=0,
                    highlightthickness=0,
                    font=("arial", 50),
                    width=2,
                    command= lambda y=y, x=x: self.click((x, y)),
                )
                button_row.append(new_button)
                new_button.grid(row=7 - y, column=x)
            self.tile_buttons.append(button_row)
        
        self.frames['chess_board'].grid(row=0, column=0)

        self.frames['side_bar'] = tk.Frame(self)
        ai_button = tk.Button(
            self.frames['side_bar'],
            text="calculate ai move",
            command=self.ai_move
        )
        ai_button.pack(side=TOP)
        undo_button = tk.Button(
            self.frames['side_bar'],
            text="Undo",
            command=self.undo,
        )
        undo_button.pack(side=TOP)

        self.frames['depth'] = tk.Frame(self.frames['side_bar'])
        down_button = tk.Button(
            self.frames['depth'],
            text="\/",
            command= lambda: self.update_depth(-1),
        )
        down_button.pack(side=LEFT)
        self.depth_label = tk.Label(
            self.frames['depth'],
            width=3,
            font=('arial', 25),
            text=self.depth
        )
        self.depth_label.pack(side=LEFT)
        up_button = tk.Button(
            self.frames['depth'],
            text="/\\",
            command= lambda: self.update_depth(1),
        )
        up_button.pack(side=LEFT)
        self.frames['depth'].pack(side=TOP)

        switch_viewing_side_button = tk.Button(
            self.frames['side_bar'],
            text="Switch viewing side",
            command=self.toggle_viewing_side
        )
        switch_viewing_side_button.pack(side=TOP)

        self.frames['side_bar'].grid(row=0, column=1)

    def update_depth(self, depth):
        self.depth = max(self.depth + depth, 2)
        self.draw()

    def ai_move(self):
        start = dt.now()
        self.boards_history.append(dumps(self.board, self.color, self.castling))
        fen_string, move = chess_ai.get_best_move(dumps(self.board, self.color, self.castling), self.depth, True)
        self.log.append({
            "fen_string": fen_string,
        })
        self.write_log()
        self.board, self.castling = loads(fen_string)
        elapsed = dt.now() - start
        print(f"Calculation time: {elapsed}")
        self.draw()

    def draw(self):
        highlights = self.get_highlights()

        for y, button_row in enumerate(reversed(self.tile_buttons) if self.viewing_side == "black" else self.tile_buttons):
            for x, button in enumerate(reversed(button_row) if self.viewing_side == "black" else button_row):
                tile = self.board[y][x]
                bg = "#f0d9b5" if (x + y) % 2 == 1 else "#b58863" 
                if (x, y) in highlights:
                    bg = "#c4e580"
                if (x, y) == self.selected_tile:
                    bg = "yellow"
                fg = PIECE_COLORS[tile[0]] if tile is not None else "white"
                button.config(
                    bg=bg,
                    activebackground=bg,
                    text=UNICODE_CHESS_PIECES[tile[1]]  if tile is not None else "",
                    fg=fg,
                    activeforeground=fg,
                    command= lambda y=y, x=x: self.click((x, y)),
                )
        
        self.depth_label.config(text=self.depth)
