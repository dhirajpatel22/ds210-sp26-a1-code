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
    fn solve(board: &mut Board, player: Player, _time_limit: u64) -> (i32, usize, usize) {
        // If you want to make a recursive call to this solution, use
        // `SolutionAgent::solve(...)`
        if board.game_over() {
            return (board.score(), 0, 0);
        }

        let moves = board.moves();
        let move_count = moves.len();

        // make max_depth dynamic depending on how number of moves
        let max_depth: u8 = if move_count >= 17 {
            3
        } else if move_count >= 12 {
            4
        } else {
            5
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

        // trim branches in cases where move_count is high
        let candidate_count: usize = if move_count >= 20 { //if move_count >= 20, only keep top 8
            8
        } else if move_count >= 16 {
            10
        } else if move_count >= 12 {
            12
        } else {
            move_count
        };

        //initialize best_move & best_score
        let mut best_move: (usize, usize) = (0, 0);
        let mut best_score: i32 = match player {
            X => i32::MIN,
            O => i32::MAX,
        };
        
        for (_, possible_move) in ordered_moves.into_iter().take(candidate_count) { //loop through best candiate moves
            board.apply_move(possible_move, player); //temporarily make the move

            let future_score = minimax_with_depth(board, player.flip(), 1, max_depth); //recursively evaluate move

            board.undo_move(possible_move, player); //undo the move

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
    let cells = board.get_cells();
    let n = cells.len();

    let mut estimate = board.score() * 100;

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

    // Rows and columns.
    for i in 0..n {
        for j in 0..=(n - 3) {
            estimate += eval_window(&cells[i][j], &cells[i][j + 1], &cells[i][j + 2]);
            estimate += eval_window(&cells[j][i], &cells[j + 1][i], &cells[j + 2][i]);
        }
    }

    // Diagonals.
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