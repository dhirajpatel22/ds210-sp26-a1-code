use tic_tac_toe_stencil::agents::Agent;
use tic_tac_toe_stencil::board::Board;
use tic_tac_toe_stencil::player::Player::{self, X, O};

// Your solution solution.
pub struct SolutionAgent {}

// Put your solution here.
impl Agent for SolutionAgent {
    // Should returns (<score>, <x>, <y>)
    // where <score> is your estimate for the score of the game
    // and <x>, <y> are the position of the move your solution will make.
    fn solve(board: &mut Board, player: Player, _time_limit: u64) -> (i32, usize, usize) {
        // If you want to make a recursive call to this solution, use
        // `SolutionAgent::solve(...)`
        if board.game_over() {
            return (board.score(), 0, 0); //is this correct way to return zero move?
        }
        
        let moves = board.moves();
        let mut best_move: (usize, usize) = (0, 0);
        let mut best_score: i32 = i32::MIN; //only for X

        for possible_move in moves {
            let mut test_board = board.clone();
            test_board.apply_move(possible_move, player);
            
            let (future_score, _, _) = SolutionAgent::solve(&mut test_board, player, _time_limit);

            let better = future_score > best_score; //only for X
            
            if better {
                best_score = future_score;
                best_move = possible_move;
            }
        }

        return (best_score, best_move.0, best_move.1);
    }
}
