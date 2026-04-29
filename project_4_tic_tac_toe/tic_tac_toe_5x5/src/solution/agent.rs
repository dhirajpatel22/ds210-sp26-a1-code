use std::collections::HashMap;
use std::time::{Duration, Instant};
use tic_tac_toe_stencil::agents::Agent;
use tic_tac_toe_stencil::board::{Board, Cell};
use tic_tac_toe_stencil::player::Player;

const C_EMPTY: u8 = 0;
const C_X: u8 = 1;
const C_O: u8 = 2;
const C_WALL: u8 = 3;


const SIDE_X: i32 = 1;
const SIDE_O: i32 = -1;

const TIME_MARGIN_MS: u64 = 20;

const TERMINAL_SCALE: i32 = 1000000;

const MAX_PLY: usize = 32;

#[derive(Clone, Copy)]
enum TTBound {
 Exact,
 Lower,
 Upper,
}

#[derive(Clone, Copy)]
struct TTEntry {
 depth: u8,
 value: i32,
 best_move: u16,
 bound: TTBound,
}

pub struct SolutionAgent {}

impl Agent for SolutionAgent {
 fn solve(board: &mut Board, player: Player, time_limit: u64) -> (i32, usize, usize) {
 if board.game_over() {
 return (board.score(), 0, 0);
 }

 let mut state = State::from_board(board);
 let deadline = Instant::now() + Duration::from_millis(time_limit.saturating_sub(TIME_MARGIN_MS));

 let root_side = match player {
 Player::X => SIDE_X,
 Player::O => SIDE_O,
 };

 let (score_side_perspective, best_idx) = state.search_root(root_side, deadline);

 let (r, c) = state.idx_to_rc(best_idx);

 let score_x = score_side_perspective * root_side;
 (score_x, r, c)
 }
}

struct State {
 n: usize, 
 cells: Vec<u8>, 
 zobrist: Vec<[u64; 4]>,
 turn_key: u64,
 hash: u64,
 tt: HashMap<u64, TTEntry>,

 line3s: Vec<[u16; 3]>,
 line3_x: Vec<u8>, 
 line3_o: Vec<u8>, 
 cell_to_l3: Vec<Vec<u16>>,


 line4s: Vec<[u16; 4]>,
 line4_x: Vec<u8>,
 line4_o: Vec<u8>,
 cell_to_l4: Vec<Vec<u16>>,

 
 killers: [[i32; 2]; MAX_PLY],
 history: Vec<i32>,

 nodes: u64,
 deadline: Instant,
 time_up: bool,
}

impl State {
 fn from_board(board: &Board) -> State {
 let cells_ref = board.get_cells();
 let n = cells_ref.len();
 let mut cells = vec![C_EMPTY; n * n];
 for i in 0..n {
 for j in 0..n {
 cells[i * n + j] = match &cells_ref[i][j] {
 Cell::Empty => C_EMPTY,
 Cell::X => C_X,
 Cell::O => C_O,
 Cell::Wall => C_WALL,
 };
 }
 }

 let zobrist = build_zobrist(n);
 let mut turn_seed = 0x9E37_79B9_7F4A_7C15u64 ^ n as u64;
 let turn_key = splitmix64(&mut turn_seed);
 let mut hash = 0u64;
 for idx in 0..cells.len() {
 hash ^= zobrist[idx][cells[idx] as usize];
 }

 let (line3s, cell_to_l3) = build_lines_3(&cells, n);
 let (line4s, cell_to_l4) = build_lines_4(&cells, n);

 let mut line3_x = vec![0u8; line3s.len()];
 let mut line3_o = vec![0u8; line3s.len()];
 for (k, line) in line3s.iter().enumerate() {
 for &idx in line.iter() {
 match cells[idx as usize] {
 C_X => line3_x[k] += 1,
 C_O => line3_o[k] += 1,
 _ => {}
 }
 }
 }

 let mut line4_x = vec![0u8; line4s.len()];
 let mut line4_o = vec![0u8; line4s.len()];
 for (k, line) in line4s.iter().enumerate() {
 for &idx in line.iter() {
 match cells[idx as usize] {
 C_X => line4_x[k] += 1,
 C_O => line4_o[k] += 1,
 _ => {}
 }
 }
 }

 State {
 n,
 cells,
 zobrist,
 turn_key,
 hash,
 tt: HashMap::with_capacity(4096),
 line3s,
 line3_x,
 line3_o,
 cell_to_l3,
 line4s,
 line4_x,
 line4_o,
 cell_to_l4,
 killers: [[-1, -1]; MAX_PLY],
 history: vec![0; n * n],
 nodes: 0,
 deadline: Instant::now(),
 time_up: false,
 }
 }

 #[inline(always)]
 fn idx_to_rc(&self, idx: usize) -> (usize, usize) {
 (idx / self.n, idx % self.n)
 }

 #[inline]
 fn apply(&mut self, idx: usize, side: i32) {
 debug_assert_eq!(self.cells[idx], C_EMPTY);
 let old = self.cells[idx] as usize;
 let new = if side == SIDE_X { C_X as usize } else { C_O as usize };
 self.hash ^= self.zobrist[idx][old] ^ self.zobrist[idx][new];
 self.cells[idx] = if side == SIDE_X { C_X } else { C_O };
 if side == SIDE_X {
 for &line_idx in &self.cell_to_l3[idx] {
 self.line3_x[line_idx as usize] += 1;
 }
 for &line_idx in &self.cell_to_l4[idx] {
 self.line4_x[line_idx as usize] += 1;
 }
 } else {
 for &line_idx in &self.cell_to_l3[idx] {
 self.line3_o[line_idx as usize] += 1;
 }
 for &line_idx in &self.cell_to_l4[idx] {
 self.line4_o[line_idx as usize] += 1;
 }
 }
 }

 #[inline]
 fn undo(&mut self, idx: usize, side: i32) {
 let old = self.cells[idx] as usize;
 self.hash ^= self.zobrist[idx][old] ^ self.zobrist[idx][C_EMPTY as usize];
 self.cells[idx] = C_EMPTY;
 if side == SIDE_X {
 for &line_idx in &self.cell_to_l3[idx] {
 self.line3_x[line_idx as usize] -= 1;
 }
 for &line_idx in &self.cell_to_l4[idx] {
 self.line4_x[line_idx as usize] -= 1;
 }
 } else {
 for &line_idx in &self.cell_to_l3[idx] {
 self.line3_o[line_idx as usize] -= 1;
 }
 for &line_idx in &self.cell_to_l4[idx] {
 self.line4_o[line_idx as usize] -= 1;
 }
 }
 }

 fn actual_score(&self) -> i32 {
 let mut score: i32 = 0;
 for k in 0..self.line3s.len() {
 if self.line3_x[k] == 3 {
 score += 1;
 } else if self.line3_o[k] == 3 {
 score -= 1;
 }
 }
 score
 }

 fn heuristic(&self) -> i32 {
 let mut score: i32 = 0;

 for k in 0..self.line3s.len() {
 let x = self.line3_x[k];
 let o = self.line3_o[k];
 if x > 0 && o > 0 {
 continue;
 }
 if x > 0 {
 score += match x {
 3 => 100, 
 2 => 20, 
 1 => 2,
 _ => 0,
 };
 } else if o > 0 {
 score -= match o {
 3 => 100,
 2 => 20,
 1 => 2,
 _ => 0,
 };
 }
 }

 for k in 0..self.line4s.len() {
 let x = self.line4_x[k];
 let o = self.line4_o[k];
 if x > 0 && o > 0 {
 continue;
 }
 if x > 0 {
 score += match x {
 4 => 40, 
 3 => 25, 
 _ => 0,
 };
 } else if o > 0 {
 score -= match o {
 4 => 40,
 3 => 25,
 _ => 0,
 };
 }
 }

 score
 }

 #[inline]
 fn is_full(&self) -> bool {
 self.cells.iter().all(|&c| c != C_EMPTY)
 }

 fn local_moves(&self, buf: &mut Vec<u16>) {
 buf.clear();
 let mut seen = vec![false; self.cells.len()];
 let mut found_stone = false;

 for idx in 0..self.cells.len() {
 if self.cells[idx] == C_EMPTY {
 continue;
 }
 found_stone = true;
 let row = idx / self.n;
 let col = idx % self.n;
 for delta_row in -1i32..=1 {
 for delta_col in -1i32..=1 {
 let new_row = row as i32 + delta_row;
 let new_col = col as i32 + delta_col;
 if new_row < 0 || new_col < 0 || new_row >= self.n as i32 || new_col >= self.n as i32 {
 continue;
 }
 let neighbor_idx = new_row as usize * self.n + new_col as usize;
 if self.cells[neighbor_idx] == C_EMPTY && !seen[neighbor_idx] {
 seen[neighbor_idx] = true;
 buf.push(neighbor_idx as u16);
 }
 }
 }
 }

 if !found_stone {
 let center = (self.n / 2) * self.n + (self.n / 2);
 if self.cells[center] == C_EMPTY {
 buf.push(center as u16);
 return;
 }
 }

 if buf.is_empty() {
 for idx in 0..self.cells.len() {
 if self.cells[idx] == C_EMPTY {
 buf.push(idx as u16);
 }
 }
 }
 }

 #[inline]
 fn check_time(&mut self) -> bool {
 if self.time_up {
 return true;
 }

 if self.nodes & 0x7ff == 0 && Instant::now() >= self.deadline {
 self.time_up = true;
 return true;
 }
 false
 }

 fn search_root(&mut self, root_side: i32, deadline: Instant) -> (i32, usize) {
 self.deadline = deadline;
 self.time_up = false;
 self.nodes = 0;
 self.tt.clear();
 self.hash ^= if root_side == SIDE_X { self.turn_key } else { 0 };

 let mut root_moves: Vec<u16> = Vec::with_capacity(25);
 self.local_moves(&mut root_moves);
 if root_moves.is_empty() {
 self.hash ^= if root_side == SIDE_X { self.turn_key } else { 0 };
 return (0, 0);
 }

 let mut ordered: Vec<(i32, u16)> = Vec::with_capacity(root_moves.len());
 for &m in &root_moves {
 self.apply(m as usize, root_side);
 let score = self.heuristic() * root_side;
 self.undo(m as usize, root_side);
 ordered.push((score, m));
 }
 ordered.sort_by(|a, b| b.0.cmp(&a.0));

 let mut best_move: u16 = ordered[0].1;
 let mut best_score: i32 = 0;

 let empty_count = self.cells.iter().filter(|&&c| c == C_EMPTY).count();
 let depth_cap = empty_count.min(MAX_PLY - 1) as u8;

 for depth in 1..=depth_cap {
 if let Some(entry) = self.tt.get(&self.hash).copied() {
 if let Some(pos) = ordered.iter().position(|&(_, m)| m == entry.best_move) {
 ordered.swap(0, pos);
 }
 }

 if let Some(pos) = ordered.iter().position(|&(_, m)| m == best_move) {
 ordered.swap(0, pos);
 }

 let mut alpha: i32 = -i32::MAX / 2;
 let beta: i32 = i32::MAX / 2;
 let mut iter_best_move: u16 = ordered[0].1;
 let mut iter_best_score: i32 = -i32::MAX / 2;
 let mut iter_scores: Vec<(i32, u16)> = Vec::with_capacity(ordered.len());
 let mut completed = true;

 let n_moves = ordered.len();
 for i in 0..n_moves {
 if self.check_time() {
 completed = false;
 break;
 }
 let m = ordered[i].1;
 self.apply(m as usize, root_side);
 self.hash ^= self.turn_key;
 let score = -self.negamax(depth - 1, -beta, -alpha, -root_side, 1);
 self.hash ^= self.turn_key;
 self.undo(m as usize, root_side);

 if self.time_up {
 completed = false;
 break;
 }

 iter_scores.push((score, m));
 if score > iter_best_score {
 iter_best_score = score;
 iter_best_move = m;
 }
 if score > alpha {
 alpha = score;
 }
 }

 if completed {
 best_move = iter_best_move;
 best_score = iter_best_score;
 iter_scores.sort_by(|a, b| b.0.cmp(&a.0));
 ordered = iter_scores;

 if best_score >= TERMINAL_SCALE / 2 {
 break;
 }
 } else {
 break;
 }
 }

 self.hash ^= if root_side == SIDE_X { self.turn_key } else { 0 };

 (best_score, best_move as usize)
 }

 fn negamax(&mut self, depth: u8, mut alpha: i32, beta: i32, side: i32, ply: usize) -> i32 {
 self.nodes = self.nodes.wrapping_add(1);
 if self.check_time() {
 return 0; 
 }

 if self.is_full() {
 let score = self.actual_score();
 return score * TERMINAL_SCALE * side;
 }

 if depth == 0 {
 return self.heuristic() * side;
 }

 let mut moves: Vec<u16> = Vec::with_capacity(20);
 let alpha_orig = alpha;

 if let Some(entry) = self.tt.get(&self.hash).copied() {
 if entry.depth >= depth {
 match entry.bound {
 TTBound::Exact => return entry.value,
 TTBound::Lower if entry.value >= beta => return entry.value,
 TTBound::Upper if entry.value <= alpha => return entry.value,
 _ => {}
 }
 }
 }

 self.local_moves(&mut moves);

 if moves.len() > 1 {
 let k = if ply < MAX_PLY { self.killers[ply] } else { [-1, -1] };
 let hist = &self.history;
 moves.sort_by(|&a, &b| {
 let ak = (k[0] == a as i32) as i32 + (k[1] == a as i32) as i32;
 let bk = (k[0] == b as i32) as i32 + (k[1] == b as i32) as i32;
 if ak != bk {
 return bk.cmp(&ak);
 }
 hist[b as usize].cmp(&hist[a as usize])
 });
 }

 if let Some(entry) = self.tt.get(&self.hash).copied() {
 if let Some(pos) = moves.iter().position(|&m| m == entry.best_move) {
 moves.swap(0, pos);
 }
 }

 let mut best: i32 = -i32::MAX / 2;
 let mut best_move: u16 = moves[0];
 for m in moves {
 self.apply(m as usize, side);
 self.hash ^= self.turn_key;
 let score = -self.negamax(depth - 1, -beta, -alpha, -side, ply + 1);
 self.hash ^= self.turn_key;
 self.undo(m as usize, side);

 if self.time_up {
 return 0;
 }

 if score > best {
 best = score;
 best_move = m;
 }
 if best > alpha {
 alpha = best;
 }
 if alpha >= beta {
 // Beta cutoff: record killer + history.
 if ply < MAX_PLY {
 let k0 = self.killers[ply][0];
 if k0 != m as i32 {
 self.killers[ply][1] = k0;
 self.killers[ply][0] = m as i32;
 }
 }
 let d = depth as i32;
 self.history[m as usize] = self.history[m as usize].saturating_add(d * d);
 break;
 }
 }

 let bound = if best <= alpha_orig {
 TTBound::Upper
 } else if best >= beta {
 TTBound::Lower
 } else {
 TTBound::Exact
 };
 self.tt.insert(self.hash, TTEntry { depth, value: best, best_move, bound });

 best
 }
}

fn splitmix64(seed: &mut u64) -> u64 {
 *seed = seed.wrapping_add(0x9E37_79B9_7F4A_7C15);
 let mut z = *seed;
 z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
 z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
 z ^ (z >> 31)
}

fn build_zobrist(n: usize) -> Vec<[u64; 4]> {
 let mut seed = 0xA5A5_1F1F_5A5A_2E2Eu64 ^ (n as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15);
 let mut table = Vec::with_capacity(n * n);
 for _ in 0..(n * n) {
 table.push([
 splitmix64(&mut seed),
 splitmix64(&mut seed),
 splitmix64(&mut seed),
 splitmix64(&mut seed),
 ]);
 }
 table
}

fn has_wall(cells: &[u8], idxs: &[u16]) -> bool {
 idxs.iter().any(|&i| cells[i as usize] == C_WALL)
}

fn build_lines_3(cells: &[u8], n: usize) -> (Vec<[u16; 3]>, Vec<Vec<u16>>) {
 let mut lines: Vec<[u16; 3]> = Vec::new();
 if n < 3 {
 return (lines, vec![Vec::new(); n * n]);
 }
 for i in 0..n {
 for j in 0..=(n - 3) {

 let row = [(i * n + j) as u16, (i * n + j + 1) as u16, (i * n + j + 2) as u16];
 if !has_wall(cells, &row) {
 lines.push(row);
 }

 let col = [(j * n + i) as u16, ((j + 1) * n + i) as u16, ((j + 2) * n + i) as u16];
 if !has_wall(cells, &col) {
 lines.push(col);
 }
 }
 }
 for i in 0..=(n - 3) {
 for j in 0..=(n - 3) {

 let d1 = [
 (i * n + j) as u16,
 ((i + 1) * n + j + 1) as u16,
 ((i + 2) * n + j + 2) as u16,
 ];
 if !has_wall(cells, &d1) {
 lines.push(d1);
 }
 }
 for j in 2..n {

 let d2 = [
 (i * n + j) as u16,
 ((i + 1) * n + j - 1) as u16,
 ((i + 2) * n + j - 2) as u16,
 ];
 if !has_wall(cells, &d2) {
 lines.push(d2);
 }
 }
 }

 let mut per_cell: Vec<Vec<u16>> = vec![Vec::new(); n * n];
 for (k, l) in lines.iter().enumerate() {
 for &idx in l.iter() {
 per_cell[idx as usize].push(k as u16);
 }
 }
 (lines, per_cell)
}

fn build_lines_4(cells: &[u8], n: usize) -> (Vec<[u16; 4]>, Vec<Vec<u16>>) {
 let mut lines: Vec<[u16; 4]> = Vec::new();
 if n < 4 {
 return (lines, vec![Vec::new(); n * n]);
 }
 for i in 0..n {
 for j in 0..=(n - 4) {
 // Row
 let row = [
 (i * n + j) as u16,
 (i * n + j + 1) as u16,
 (i * n + j + 2) as u16,
 (i * n + j + 3) as u16,
 ];
 if !has_wall(cells, &row) {
 lines.push(row);
 }
 let col = [
 (j * n + i) as u16,
 ((j + 1) * n + i) as u16,
 ((j + 2) * n + i) as u16,
 ((j + 3) * n + i) as u16,
 ];
 if !has_wall(cells, &col) {
 lines.push(col);
 }
 }
 }
 for i in 0..=(n - 4) {
 for j in 0..=(n - 4) {
 let d1 = [
 (i * n + j) as u16,
 ((i + 1) * n + j + 1) as u16,
 ((i + 2) * n + j + 2) as u16,
 ((i + 3) * n + j + 3) as u16,
 ];
 if !has_wall(cells, &d1) {
 lines.push(d1);
 }
 }
 for j in 3..n {
 let d2 = [
 (i * n + j) as u16,
 ((i + 1) * n + j - 1) as u16,
 ((i + 2) * n + j - 2) as u16,
 ((i + 3) * n + j - 3) as u16,
 ];
 if !has_wall(cells, &d2) {
 lines.push(d2);
 }
 }
 }

 let mut per_cell: Vec<Vec<u16>> = vec![Vec::new(); n * n];
 for (k, l) in lines.iter().enumerate() {
 for &idx in l.iter() {
 per_cell[idx as usize].push(k as u16);
 }
 }
 (lines, per_cell)
}