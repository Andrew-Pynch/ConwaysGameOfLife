from random import randint

from cell import Automata


class Board:
    def __init__(self, rows, cols):
        self._rows = rows
        self._cols = cols

        # cell_row = [Automata(alive=False) for i in range(self._rows)]
        # self._grid = [list(cell_row) for j in range(self._cols)]

        self._grid = [
            [
                Automata(x=row_cells, y=column_cells, alive=False)
                for column_cells in range(self._cols)
            ]
            for row_cells in range(self._rows)
        ]

        self.generate_board()

    def __getitem__(self, key):
        x = key[0]
        y = key[1]
        return self._grid[x][y]

    def generate_board(self):
        for row in self._grid:
            for automata in row:
                # there is a 33% chance the cells spawn alive.
                chance_number = randint(0, 2)
                if chance_number == 1:
                    automata.set_alive()

    def show(self):
        print("\n" * 2)
        for row in self._grid:
            for automata in row:
                print(automata, end="")
            print()  # to create a new line pr. row.

    def update(self):
        for row in self._grid:
            for automata in row:
                # Analyze atomatas neighbors
                alive_neighbors, dead_neighbors = self.check_automata_neighbors(
                    automata
                )
                # Alive ruleset
                if automata._alive == True:
                    if len(alive_neighbors) < 2:
                        automata.set_dead()
                    elif len(alive_neighbors) == 2 or len(alive_neighbors) == 3:
                        automata.set_alive()
                    elif len(alive_neighbors) > 3:
                        automata.set_dead()
                # Dead ruleset
                elif automata._alive == False:
                    if len(alive_neighbors) == 3:
                        automata.set_alive()

    def check_automata_neighbors(self, automata):
        alive_neighbors = []
        dead_neighbors = []

        cell_row, cell_col = automata.coordinates()

        for row in range(3):
            for col in range(3):
                try:
                    neighbor = self._grid[cell_row - 1 + row][cell_col - 1 + col]
                    if neighbor.coordinates() != automata.coordinates():
                        if neighbor._alive == True:
                            alive_neighbors.append(neighbor)
                        else:
                            dead_neighbors.append(neighbor)
                except:
                    pass

        return (alive_neighbors, dead_neighbors)
