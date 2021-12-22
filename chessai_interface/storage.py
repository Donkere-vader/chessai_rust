
FEN_PIECE_LETTERS = {
    "rook": "r",
    "king": "k",
    "queen": "q",
    "knight": "n",
    "pawn": "p",
    "bishop": "b"
}

def dumps(game_board, color, castling) -> str:
    board_string = ""

    for rank in reversed(game_board):
        empty_on_row = 0
        for piece in rank:
            if piece is None:
                empty_on_row += 1
            else:
                if empty_on_row > 0:
                    board_string += str(empty_on_row)
                    empty_on_row = 0
                letter = FEN_PIECE_LETTERS[piece[1]]
                if piece[0] == "white":
                    letter = letter.upper()
                board_string += letter
        if empty_on_row:
            board_string += str(empty_on_row)
        board_string += "/"
    
    castling_list = []
    for piece in castling:
        let = 'k' if piece[1] == 'king' else 'q'
        castling_list.append(let.upper() if piece[0] == 'white' else let)
    
    return f"{board_string[:-1]} {'b' if color == 'black' else 'w'} {''.join(castling_list) if len(castling_list) else '-'} - 0 1"


def loads(fen_code):
    board = [[None for _ in range(8)] for _ in range(8)]
    board_string = fen_code.split()[0]
    castling_string = fen_code.split()[2]

    y = 7
    x = 0
    for char in board_string:
        if char in '12345678':
            x += int(char)
        elif char == "/":
            y -= 1
            x = 0
        else:
            board[y][x] = ("white" if char.upper() == char else "black", list(FEN_PIECE_LETTERS.keys())[list(FEN_PIECE_LETTERS.values()).index(char.lower())])
            x += 1

    castling = []
    for char in castling_string:
        if char == "-":
            break
        castling.append(("white" if char.upper() == char else "black", list(FEN_PIECE_LETTERS.keys())[list(FEN_PIECE_LETTERS.values()).index(char.lower())]))

    return board, castling
