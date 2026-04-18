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
            board.apply_move(possible_move, player); //enter minimax here? 

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

fn heuristic(board: &Board) -> i32 { 
    return board.score();
    }

fn minimax(curDepth: u8, nodeIndex: i32, maxTurn: bool, board: &Board, 
        scores: i32, targetDepth: u8) -> i32 { 
    //scores come from the heuristic function
    // I hardcoded the target depth to 3, should it be dynamic? 

    
    //I need to turn scores to a vec to index into it
    let mut scores_vec: Vec<i32> = Vec::new();
    let targetDepth: u8 = (scores_vec.len() as f64).log2() as u8; //shouldnt it be hardcoded?
    
    if curDepth == targetDepth {
        return scores_vec[nodeIndex as usize]  
    }
    //scores should be a vec to index into it, as usize
    // push the heurtistic if base case? 

    if maxTurn {
        scores_vec.push(heuristic(board)); //??? 
        return 
            (minimax(curDepth + 1,nodeIndex * 2, 
                    false, board,heuristic(board), targetDepth ))
                    .max((minimax(curDepth + 1, nodeIndex * 2 + 1,  
                    false, board,heuristic(board), targetDepth ))
                    
                   )
    }
    
    else {
        scores_vec.push(heuristic(board)); //???
        return (minimax(curDepth + 1, nodeIndex * 2,  
                     true, board,heuristic(board), targetDepth ))
                     .min((minimax(curDepth + 1, nodeIndex * 2 + 1, 
                    true, board,heuristic(board), targetDepth ))
                )
    }
}
    // heuristic(board);?
    //missing max turn
    //needs heuristic to run