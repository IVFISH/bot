use crate::placement::{Placement, PlacementList};
use crate::game::Game;
use crate::bot::Bot;

// TODO: KEEP MUTLTITHREADING IN MIND, decide WHERE to score, decide WHERE to prune, priority queue?

/// More than one depth lookahead
/// Given a starting game, generate all possible game states
/// (and their corresponding base Placements) a given depth away
/// TODO: implement
pub fn many_lookahead(start_game: Game, depth: u8) -> Vec<Game> {
    // base call of movegen on start_game, THIS WILL GENERATE BASE PLACEMENTS
    // repeatedly call lookahead, using the output as the input for the next iteration
    // once at depth, stop looking ahead and return the output of the final lookahead
    unimplemented!()
}

/// One depth lookahead
/// Given a list of games and an Rc to their base Placement, do movegen on every game
/// Return a (larger) list of games with an Rc their corresponding base move
/// TODO: finalize inputs (game or bot?), implement base Placement Rc, update game, multithreading
fn lookahead(games: Vec<Game>) -> Vec<Game> {
    let mut out = Vec::new();
    for game in games {
        // give each iteration of this loop to a different thread

        let bot: Bot = Bot{game};
        let placements = bot.move_gen().placements;

        for placement in placements.into_iter() {
            let mut out_game = game;
            out_game.board.set_piece(&placement.piece);
            // update out_game
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