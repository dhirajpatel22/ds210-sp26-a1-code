use tic_tac_toe_stencil::agents::Agent;
use tic_tac_toe_stencil::board::Board;
use tic_tac_toe_stencil::player::Player::{self, O, X};

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
            return (board.score(), 0, 0);
        }

        const MAX_DEPTH: u8 = 4;

        let moves = board.moves();
        let mut best_move: (usize, usize) = (0, 0);
        let mut best_score: i32 = match player {
            X => i32::MIN,
            O => i32::MAX,
        };

        for possible_move in moves {
            board.apply_move(possible_move, player);

            let future_score = minimax_with_depth(board, player.flip(), 1, MAX_DEPTH);

            board.undo_move(possible_move, player);

            let better = match player {
                X => future_score > best_score,
                O => future_score < best_score,
            };

            if better {
                best_score = future_score;
                best_move = possible_move;
            }
        }

        return (best_score, best_move.0, best_move.1);
    }
}

fn heuristic(board: &Board) -> i32 { 
    return board.score();
}

fn minimax_with_depth(board: &mut Board, player: Player, cur_depth: u8, max_depth: u8) -> i32 {
    if board.game_over() {
        return board.score();
    }

    if cur_depth >= max_depth {
        return heuristic(board);
    }

    let moves = board.moves();
    let mut best_score = match player {
        X => i32::MIN,
        O => i32::MAX,
    };

    for possible_move in moves {
        board.apply_move(possible_move, player);
        let future_score = minimax_with_depth(board, player.flip(), cur_depth + 1, max_depth);
        board.undo_move(possible_move, player);

        best_score = match player {
            X => best_score.max(future_score),
            O => best_score.min(future_score),
        };
    }

    best_score
}