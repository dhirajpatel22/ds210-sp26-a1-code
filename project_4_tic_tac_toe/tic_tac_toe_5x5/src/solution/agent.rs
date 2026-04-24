use std::time::{Duration, Instant};
use tic_tac_toe_stencil::agents::Agent;
use tic_tac_toe_stencil::board::{Board, Cell};
use tic_tac_toe_stencil::player::Player::{self, O, X};

// Your solution solution.
pub struct SolutionAgent {}

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

        // Create 200 ms margin - maybe change??????
        let start = Instant::now();
        let end_time = start + Duration::from_millis(time_limit.saturating_sub(200));

        // Order moves by one-ply heuristic so alpha-beta prunes more branches
        let mut ordered_moves: Vec<(i32, (usize, usize))> = Vec::with_capacity(move_count);
        for m in moves {
            board.apply_move(m, player);
            let h = heuristic(board);
            board.undo_move(m, player);
            ordered_moves.push((h, m));
        }
        ordered_moves.sort_by(|a, b| match player {
            X => b.0.cmp(&a.0),
            O => a.0.cmp(&b.0),
        });

        // Return to best heuristic move if we can't complete even depth 1
        let mut best_move = ordered_moves[0].1;
        let mut best_score = match player {
            X => i32::MIN,
            O => i32::MAX,
        };

        let is_small = move_count <= 9;

        for depth in 1..=30 {
            if !is_small && Instant::now() >= end_time {
                break;
            }

            let mut alpha = i32::MIN;
            let mut beta = i32::MAX;
            let mut iter_best_move = ordered_moves[0].1;
            let mut iter_best_score = match player {
                X => i32::MIN,
                O => i32::MAX,
            };
            let mut timed_out = false;

            for (_, m) in ordered_moves.iter() {
                if !is_small && Instant::now() >= end_time {
                    timed_out = true;
                    break;
                }

                board.apply_move(*m, player);
                let score = alphabeta(board, player.flip(), 1, depth, alpha, beta, end_time, is_small);
                board.undo_move(*m, player);

                // alphabeta ran out of time, wasn't able to finish and returned a false score. ignore it
                let timed_out_inside = !is_small && (score == i32::MIN + 1 || score == i32::MAX - 1);
                if timed_out_inside {
                    timed_out = true;
                    break;
                }

                let better = match player {
                    X => score > iter_best_score,
                    O => score < iter_best_score,
                };
                if better {
                    iter_best_score = score;
                    iter_best_move = *m;
                }
                match player {
                    X => alpha = alpha.max(iter_best_score),
                    O => beta = beta.min(iter_best_score),
                }
            }

            if !timed_out {
                best_move = iter_best_move;
                best_score = iter_best_score;
                // Move best move to front for better pruning in next iteration
                let best_idx = ordered_moves.iter().position(|(_, m)| *m == best_move).unwrap_or(0);
                ordered_moves.swap(0, best_idx);
            }

            // Brutforced them. Dont go deeper
            if best_score.abs() >= 500000 {
                break;
            }
        }

        (best_score, best_move.0, best_move.1)
    }
}

fn heuristic(board: &Board) -> i32 {
    let cells = board.get_cells();
    let n = cells.len();

    // Count empty cells to determine game phase
    let empty_count: usize = cells.iter()
        .flatten()
        .filter(|c| matches!(c, Cell::Empty))
        .count();
    let is_endgame = empty_count <= 10;

    let mut estimate = 0;

    fn eval_window(a: &Cell, b: &Cell, c: &Cell, is_endgame: bool) -> i32 {
        use Cell::{Empty, O, Wall, X};
        let threat = if is_endgame { 2 } else { 1 };
        match (a, b, c) {
            (Wall, _, _) | (_, Wall, _) | (_, _, Wall) => 0,
            (X, X, X) => 10000,
            (O, O, O) => -10000,
            (X, X, Empty) | (X, Empty, X) | (Empty, X, X) => 12 * threat,
            (O, O, Empty) | (O, Empty, O) | (Empty, O, O) => -12 * threat,
            (X, Empty, Empty) | (Empty, X, Empty) | (Empty, Empty, X) => 2,
            (O, Empty, Empty) | (Empty, O, Empty) | (Empty, Empty, O) => -2,
            _ => 0,
        }
    }
    
    fn eval_window_4(a: &Cell, b: &Cell, c: &Cell, d: &Cell, is_endgame: bool) -> i32 {
        use Cell::{Empty, O, Wall, X};
        let threat = if is_endgame { 2 } else { 1 };
        match (a, b, c, d) {
            (Wall, _, _, _) | (_, Wall, _, _) | (_, _, Wall, _) | (_, _, _, Wall) => 0,
            (X, X, X, X) => 50000,
            (O, O, O, O) => -50000,
            (X, X, X, Empty) | (X, X, Empty, X) | (X, Empty, X, X) | (Empty, X, X, X) => 40 * threat,
            (O, O, O, Empty) | (O, O, Empty, O) | (O, Empty, O, O) | (Empty, O, O, O) => -40 * threat,
            (X, X, Empty, Empty) | (X, Empty, X, Empty) | (X, Empty, Empty, X) |
            (Empty, X, X, Empty) | (Empty, X, Empty, X) | (Empty, Empty, X, X) => 5,
            (O, O, Empty, Empty) | (O, Empty, O, Empty) | (O, Empty, Empty, O) |
            (Empty, O, O, Empty) | (Empty, O, Empty, O) | (Empty, Empty, O, O) => -5,
            _ => 0,
        }
    }

    // 3 cells window
    for i in 0..n {
        for j in 0..=(n - 3) {
            estimate += eval_window(&cells[i][j], &cells[i][j + 1], &cells[i][j + 2], is_endgame);
            estimate += eval_window(&cells[j][i], &cells[j + 1][i], &cells[j + 2][i], is_endgame);
        }
    }

    // 3 diagonal (both)
    for i in 0..=(n - 3) {
        for j in 0..=(n - 3) {
            estimate += eval_window(&cells[i][j], &cells[i + 1][j + 1], &cells[i + 2][j + 2], is_endgame);
        }
        for j in 2..n {
            estimate += eval_window(&cells[i][j], &cells[i + 1][j - 1], &cells[i + 2][j - 2], is_endgame);
        }
    }

    // 4 cell window (not for 3x3)
    if n >= 4 {
        for i in 0..n {
            for j in 0..=(n - 4) {
                estimate += eval_window_4(&cells[i][j], &cells[i][j+1], &cells[i][j+2], &cells[i][j+3], is_endgame);
                estimate += eval_window_4(&cells[j][i], &cells[j+1][i], &cells[j+2][i], &cells[j+3][i], is_endgame);
            }
        }
        // diagonals
        for i in 0..=(n - 4) {
            for j in 0..=(n - 4) {
                estimate += eval_window_4(&cells[i][j], &cells[i+1][j+1], &cells[i+2][j+2], &cells[i+3][j+3], is_endgame);
            }
            for j in 3..n {
                estimate += eval_window_4(&cells[i][j], &cells[i+1][j-1], &cells[i+2][j-2], &cells[i+3][j-3], is_endgame);
            }
        }
    }

    if !is_endgame && n == 5 {
        let center_weights: [[i32; 5]; 5] = [
            [0, 0, 0, 0, 0],
            [0, 1, 2, 1, 0],
            [0, 2, 3, 2, 0],
            [0, 1, 2, 1, 0],
            [0, 0, 0, 0, 0],
        ];
        for i in 0..5 {
            for j in 0..5 {
                match cells[i][j] {
                    Cell::X => estimate += center_weights[i][j],
                    Cell::O => estimate -= center_weights[i][j],
                    _ => {}
                }
            }
        }
    }

    estimate
}

fn alphabeta(board: &mut Board, player: Player, cur_depth: u8, max_depth: u8, mut alpha: i32, mut beta: i32, end_time: Instant, is_small: bool) -> i32 {

    if !is_small && Instant::now() >= end_time {
        // ran out of time. return the crap score so to throw it away later
        return match player { X => i32::MIN + 1, O => i32::MAX - 1 };
    }

    if board.game_over() {
        return board.score() * 1000000;
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
                let score = alphabeta(board, player.flip(), cur_depth + 1, max_depth, alpha, beta, end_time, is_small);
                board.undo_move(m, player);
                best = best.max(score);
                alpha = alpha.max(best);
                if beta <= alpha {
                    break; 
                }
            }
            best
        }
        O => {
            let mut best = i32::MAX;
            for m in moves {
                board.apply_move(m, player);
                let score = alphabeta(board, player.flip(), cur_depth + 1, max_depth, alpha, beta, end_time, is_small);
                board.undo_move(m, player);
                best = best.min(score);
                beta = beta.min(best);
                if beta <= alpha {
                    break;
                }
            }
            best
        }
    }
}