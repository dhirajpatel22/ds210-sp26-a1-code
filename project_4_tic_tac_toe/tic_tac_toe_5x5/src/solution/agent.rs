use tic_tac_toe_stencil::agents::Agent;
use tic_tac_toe_stencil::board::{Board, Cell};
use tic_tac_toe_stencil::player::Player::{self, O, X};

// Your solution solution.
pub struct SolutionAgent {}

// Put your solution here.
impl Agent for SolutionAgent {
    // Should returns (<score>, <x>, <y>)
    // where <score> is your estimate for the score of the game
    // and <x>, <y> are the position of the move your solution will make.
    fn solve(board: &mut Board, player: Player, time_limit: u64) -> (i32, usize, usize) {
        // If you want to make a recursive call to this solution, use
        // `SolutionAgent::solve(...)`
        if board.game_over() {
            return (board.score(), 0, 0);
        }

        let moves = board.moves();
        let move_count = moves.len();

        // make max_depth dynamic depending on how number of moves changes
        // TO DHIRAJ - I think we can increase depoth more and make it more detailed
        //use time limit here - intrduce the notion of time left: time_limit - time spent so far, and adjust max_depth accordingly

        let max_depth: u8 = if move_count >= 17 {
            6
        } else if move_count >= 12 {
            7
        } else {
            8
        };

        // Order candidates with a one-ply heuristic so we search promising moves first.
        let mut ordered_moves: Vec<(i32, (usize, usize))> = Vec::with_capacity(move_count);
        for m in moves { //iterate through every move
            board.apply_move(m, player); //temporarily apply the move
            let h = heuristic(board); //call heuristic to get a quick look at how good the move is
            board.undo_move(m, player); //undo the move
            ordered_moves.push((h, m)); //save how good the move is (h) & the move (m) to list
        }

        //sort the moves by how good they are
        ordered_moves.sort_by(|a, b| match player { 
            X => b.0.cmp(&a.0), //sort descending for X
            O => a.0.cmp(&b.0), //sort ascending for O
        });

        //initialize best_move & best_score
        let mut best_move: (usize, usize) = (0, 0);
        let mut best_score: i32 = match player {
            X => i32::MIN,
            O => i32::MAX,
        };

        // alpha-beta window at the top level
        let mut alpha = i32::MIN;
        let mut beta = i32::MAX;

        for (_, possible_move) in ordered_moves.into_iter() { //loop through all moves (ordered by heuristic)
            board.apply_move(possible_move, player); //temporarily make the move

            let future_score = alphabeta(board, player.flip(), 1, max_depth, alpha, beta); //alpha-beta search

            board.undo_move(possible_move, player); //undo the move

            let better = match player { 
                X => future_score > best_score,
                O => future_score < best_score,
            };

            if better {
                best_score = future_score;
                best_move = possible_move;
            }

            // narrow the window at the top level
            match player {
                X => alpha = alpha.max(best_score),
                O => beta = beta.min(best_score),
            }
        }

        return (best_score, best_move.0, best_move.1);
    }
}

fn heuristic(board: &Board) -> i32 {
    let cells = board.get_cells();
    let n = cells.len();

    let mut estimate = board.score(); //100->1000 to prioritize winning/losing moves over heuristic evaluation of non-winning moves

    // TO DHIRAJ - maybe increase the window when there are more moves left or just better running time?
    fn eval_window(a: &Cell, b: &Cell, c: &Cell) -> i32 {
        use Cell::{Empty, O, Wall, X};

        match (a, b, c) {
            (Wall, _, _) | (_, Wall, _) | (_, _, Wall) => 0,
            (X, X, X) => 100,
            (O, O, O) => -100,
            (X, X, Empty) | (X, Empty, X) | (Empty, X, X) => 12,
            (O, O, Empty) | (O, Empty, O) | (Empty, O, O) => -12,
            (X, Empty, Empty) | (Empty, X, Empty) | (Empty, Empty, X) => 2,
            (O, Empty, Empty) | (Empty, O, Empty) | (Empty, Empty, O) => -2,
            _ => 0,
        }
    }

    // iterate over rows and columns & update estimate
    for i in 0..n {
        for j in 0..=(n - 3) {
            estimate += eval_window(&cells[i][j], &cells[i][j + 1], &cells[i][j + 2]);
            estimate += eval_window(&cells[j][i], &cells[j + 1][i], &cells[j + 2][i]);
        }
    }

    // iterate over diagonals & update estimate
    for i in 0..=(n - 3) {
        for j in 0..=(n - 3) {
            estimate += eval_window(&cells[i][j], &cells[i + 1][j + 1], &cells[i + 2][j + 2]);
        }
        for j in 2..n {
            estimate += eval_window(&cells[i][j], &cells[i + 1][j - 1], &cells[i + 2][j - 2]);
        }
    }

    estimate
}

fn alphabeta(board: &mut Board, player: Player, cur_depth: u8, max_depth: u8, mut alpha: i32, mut beta: i32) -> i32 {
    if board.game_over() {
        return board.score() * 1000;
    }

    if cur_depth >= max_depth {
        return heuristic(board);
    }

    let moves = board.moves();

    match player {
        X => {
            let mut best = i32::MIN;
            for m in moves {
                board.apply_move(m, player);
                let score = alphabeta(board, player.flip(), cur_depth + 1, max_depth, alpha, beta);
                board.undo_move(m, player);
                best = best.max(score);
                alpha = alpha.max(best);
                if beta <= alpha {
                    break; // beta cutoff
                }
            }
            best
        }
        O => {
            let mut best = i32::MAX;
            for m in moves {
                board.apply_move(m, player);
                let score = alphabeta(board, player.flip(), cur_depth + 1, max_depth, alpha, beta);
                board.undo_move(m, player);
                best = best.min(score);
                beta = beta.min(best);
                if beta <= alpha {
                    break; // alpha cutoff
                }
            }
            best
        }
    }
}