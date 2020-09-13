# Implementation Strategy

### Board

- Rows
- Cols
- Cells that are dead or alive
- Indexable by x, y coordinates

### Automata

- Alive?
- X
- Y
- Ability to update its status and check its neighbors

  # Cell is alive it will

  - Die if there are less than two living neighbors
  - Continue living if there are exactly two or three living neighbors
  - Die if there are more than three living neighbors

  # Cell is dead it will

  - Resurrect if there are exactly three living neighbors
