class Automata:
    def __init__(self, x=0, y=0, alive=False):
        self._x = x
        self._y = y
        self._alive = alive

    def __str__(self):
        if self._alive == True:
            return " O "
        return " * "

    def set_alive(self):
        self._alive = True

    def set_dead(self):
        self._alive = False

    def mortality_status(self):
        return self._alive

    def set_coordinates(self, x, y):
        self._x = x
        self._y = y

    def coordinates(self):
        return (self._x, self._y)
