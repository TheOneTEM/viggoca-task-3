# viggoca-task-3

This library has only *one* public function: game_turn. It takes the following args: 

- fen: Current game state (FEN string) as argument
- action: String in the form "e2 e4" where the first algebraic coordinate is the square of the piece to move, and the second coordinate is the square to which to move to.

The function will return the new FEN string if a valid move was made, otherwise it will return the original FEN string. 

The library can't castle or en passant, as I did not have time to complete those.
