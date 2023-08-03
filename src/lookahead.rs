use crate::bot::Bot;
use crate::constants::piece_constants::PIECE_L;
use crate::game::Game;
use crate::piece;
use crate::piece::Piece;
use crate::placement::{Placement, PlacementList};
use itertools::Itertools;

// TODO: decide WHERE to score, decide WHERE to prune, priority queue?
// KEEP MUTLITHREADING IN MIND

/// More than one depth lookahead.
/// Given a starting game, generate all possible game states a given depth away
/// and link them to the initial placement in their sequence aka their "base Placement".
/// TODO: linking to base_placement
pub fn many_lookahead(start_game: Game, depth: u8) -> Vec<Game> {
    // base call of movegen on start_game, THIS WILL GENERATE BASE PLACEMENTS
    let mut b = Bot { game: start_game };
    let placements = move_gen_hold(b); // WILL HAVE TO EXTRACT BASE COMMANDS
    let base_games = games_from_placements(placements, &start_game);

    // repeatedly call lookahead, using the output as the input for the next iteration
    let mut new_games = base_games;
    for _ in 1..depth {
        new_games = lookahead(new_games);
    }

    // once at depth, stop looking ahead and return the output of the final lookahead
    new_games
}

/// One depth lookahead, helper method for [`lookahead.many_lookahead`].
/// Given a list of games and their base Placements, do movegen on every game.
/// Return a (larger) list of games, each with their base Placement.
/// TODO: finalize inputs (game or bot?), implement base Placement Rc, update game, multithreading
fn lookahead(games: Vec<Game>) -> Vec<Game> {
    let mut out = Vec::new();
    for game in games {
        let mut bot: Bot = Bot { game };
        out.extend(games_from_placements(move_gen_hold(bot), &game));
    }
    out
}

// helper methods---------------------------------------------

fn move_gen_hold(mut bot: Bot) -> Vec<Placement> {
    let mut placements = bot.move_gen().placements;

    // make sure hold is not redundant
    if bot.game.hold == Some(bot.game.active.r#type) {
        return placements;
    }

    // JUST HOLD
    if let Some(hold_piece) = bot.game.hold {
        bot.game.active = Piece::new(hold_piece);
        bot.game.hold = Some(hold_piece);
    } else {
        bot.game.hold = Some(bot.game.active.r#type);
        bot.game.active = Piece::new(PIECE_L); // sample next piece in queue lmfao
    }

    placements.extend(bot.move_gen().placements);
    placements
}

fn games_from_placements(placements: Vec<Placement>, base_game: &Game) -> Vec<Game> {
    let mut out = Vec::new();
    for placement in placements {
        let mut out_game = *base_game; // copy
        out_game.board.set_piece(&placement.piece);
        // update out_game (line clears, garbage(?), stats)
        // scoring?
        out.push(out_game);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_api::functions::*;

    // build and execute T spin

    // downstack

    // PC
}
