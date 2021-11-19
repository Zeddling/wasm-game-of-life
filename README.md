# Cornway's Game of Life
It's a cellular automaton devised by British mathematician John Horton Conway in 1970.
The game's evolution is determined by its initial state whilst requiring no input. The game
is played by creating an initial configuration and observing how it evolves. It is Turing complete.
More information can be found [here](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life)

Tutorial: [rustwasm](https://rustwasm.github.io/)

### Implementation
We create a fixed size, periodic universe where cells on the edges have neighbors that wrap
around to the other side of the universe thus accomodating for infinite patterns like gliders
while ensuring we don't run out of memory.
