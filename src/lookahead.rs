use crate::bot::Bot;
use crate::constants::piece_constants::PIECE_L;
use crate::game::Game;
use crate::piece;
use crate::piece::Piece;
use crate::placement::{Placement, PlacementList};

// TODO: decide WHERE to score, decide WHERE to prune, priority queue?
// KEEP MUTLITHREADING IN MIND

/// More than one depth lookahead.
/// Given a starting game, generate all possible game states a given depth away
/// and link them to the initial placement in their sequence aka their "base Placement".
/// TODO: implement
pub fn many_lookahead(start_game: Game, depth: u8) -> Vec<Game> {
    // base call of movegen on start_game, THIS WILL GENERATE BASE PLACEMENTS
    let b = Bot { game: start_game };
    let placements = b.move_gen().placements;

    let mut base_games = Vec::new();
    for placement in placements {
        let mut base_game = b.game;
        base_game.board.set_piece(&placement.piece);
        base_games.push(base_game);
    }

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
        // give each iteration of this loop to a different thread
        // each thread can handle many such iterations

        let mut bot: Bot = Bot { game };
        let mut placements = bot.move_gen().placements;

        if let Some(hold_piece) = bot.game.hold {
            bot.game.active = Piece::new(hold_piece);
        }
        else {
            bot.game.active = Piece::new(PIECE_L);
        }

        placements.extend(bot.move_gen().placements);

        for placement in placements.into_iter() {
            let mut out_game = game;
            out_game.board.set_piece(&placement.piece);
            // update out_game (line clears, garbage(?), stats)
            // scoring?
            out.push(out_game);
        }
    }
    out // could make this function recursive quite easily, TBD :)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_api::functions::*;

    // build and execute T spin

    // downstack

    // PC
}
