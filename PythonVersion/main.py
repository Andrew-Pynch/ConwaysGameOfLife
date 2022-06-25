import time

from board import Board
from cell import Automata

ROWS = 10
COLS = 10
ITERATIONS = 100


def main():
    board = Board(ROWS, COLS)
    board.show()
    for iteration in range(ITERATIONS):
        board.update()
        time.sleep(0.5)
        board.show()


if __name__ == "__main__":
    main()
    # cell = Automata(x=0, y=1, alive=False)
    # print(cell.coordinates())
