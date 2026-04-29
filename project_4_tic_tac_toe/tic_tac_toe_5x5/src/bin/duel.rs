// Head-to-head bench: NEW SolutionAgent vs OldSolutionAgent.
// Runs N games on Layout5x5::Random(walls), alternating which side NEW plays.
// Reports win / draw / loss for NEW.
//
// Usage:
//   cargo run --bin duel -- --games 30 --walls 5
//   cargo run --release --bin duel -- --games 60 --walls 5
//
// Note: tournament/tests run WITHOUT --release. The duel can be run either
// way; release mode is faster but doesn't reflect grading conditions.

use clap::Parser;
use tic_tac_toe_stencil::game_loop;
use tic_tac_toe_stencil::Outcome;

use tic_tac_toe_5x5::layout::Layout5x5;
use tic_tac_toe_5x5::solution::agent::SolutionAgent;
use tic_tac_toe_5x5::solution::old_agent::OldSolutionAgent;

const TIME_LIMIT: u64 = 2000;

#[derive(Parser, Debug)]
struct Args {
    /// Number of games to play
    #[arg(short, long, default_value_t = 30)]
    games: usize,

    /// Number of walls on the random 5x5 layout
    #[arg(short, long, default_value_t = 5)]
    walls: usize,
}

fn main() {
    let args = Args::parse();
    let mut new_wins = 0;
    let mut old_wins = 0;
    let mut draws = 0;
    let mut new_x_wins = 0;
    let mut new_o_wins = 0;

    println!(
        "Duel: NEW SolutionAgent vs OLD agent — {} games, walls={}, time_limit={}ms",
        args.games, args.walls, TIME_LIMIT
    );
    println!("{:-<60}", "");

    for i in 0..args.games {
        // Alternate sides each game so we control for first-move advantage.
        let new_plays_x = i % 2 == 0;
        let layout = Layout5x5::Random(args.walls);

        let outcome = if new_plays_x {
            game_loop::<_, SolutionAgent, OldSolutionAgent>(layout, TIME_LIMIT, true)
        } else {
            game_loop::<_, OldSolutionAgent, SolutionAgent>(layout, TIME_LIMIT, true)
        };

        let new_won = match outcome {
            Outcome::X => new_plays_x,
            Outcome::O => !new_plays_x,
            Outcome::Draw => false,
        };
        let old_won = match outcome {
            Outcome::X => !new_plays_x,
            Outcome::O => new_plays_x,
            Outcome::Draw => false,
        };

        if outcome == Outcome::Draw {
            draws += 1;
        } else if new_won {
            new_wins += 1;
            if new_plays_x {
                new_x_wins += 1;
            } else {
                new_o_wins += 1;
            }
        } else if old_won {
            old_wins += 1;
        }

        println!(
            "Game {:>3}: NEW={}, result={:?}  [running: NEW {}-{}-{} OLD]",
            i + 1,
            if new_plays_x { "X" } else { "O" },
            outcome,
            new_wins,
            draws,
            old_wins
        );
    }

    println!("{:-<60}", "");
    println!(
        "FINAL — NEW: {} wins ({} as X, {} as O)  |  OLD: {} wins  |  Draws: {}",
        new_wins, new_x_wins, new_o_wins, old_wins, draws
    );
    let games_with_winner = new_wins + old_wins;
    if games_with_winner > 0 {
        let pct = 100.0 * new_wins as f32 / games_with_winner as f32;
        println!("NEW win rate (excluding draws): {:.1}%", pct);
    }
}
