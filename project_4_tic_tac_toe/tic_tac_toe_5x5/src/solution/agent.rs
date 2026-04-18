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

        let moves = board.moves();
        let mut best_move: (usize, usize) = (0, 0);
        let mut best_score: i32 = match player {
            X => i32::MIN,
            O => i32::MAX,
        };

        for possible_move in moves {
            board.apply_move(possible_move, player);

            let (future_score, _, _) =
                SolutionAgent::solve(board, player.flip(), _time_limit);

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

fn heuristic(board: &Board) -> Vec<i32> {
        let scores = board.score();
        let mut vec_scores: Vec<i32> = Vec::new();
        let vec_scores = vec_scores.push(scores);
        return vec_scores; //changed this from dhiraj
    }

fn minimax(cur_depth: u8, nodeIndex: i32, maxTurn: bool, 
        scores: i32, targetDepth: u8) -> i32 {
    
    if curDepth == targetDepth {
        return scores[nodeIndex] //scores should be a vec to index into it 
    }

    if maxTurn {
        return ((minimax(curDepth + 1,nodeIndex * 2, 
                    false, scores, targetDepth )), 

                   minimax(curDepth + 1, nodeIndex * 2 + 1,  
                    false, scores, targetDepth )
                ).max()
    }
    
    else {
        return ((minimax(curDepth + 1, nodeIndex * 2,  
                     true, scores, targetDepth )), 

                   minimax(curDepth + 1, nodeIndex * 2 + 1, 
                          true, scores, targetDepth)
                ).min()
    }
}
    // heuristic(board);?
    //missing max turn
    //needs heuristic to run