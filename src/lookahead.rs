use std::collections::HashSet;
use crate::bot::Bot;
use crate::constants::piece_constants::PIECE_L;
use crate::game::Game;
use crate::piece;
use crate::piece::Piece;
use crate::placement::{Placement, PlacementList};
use itertools::Itertools;

use rayon::prelude::*;

// TODO: decide WHERE to score, decide WHERE to prune, priority queue?
// KEEP MUTLITHREADING IN MIND

/// More than one depth lookahead.
/// Given a starting game, generate all possible game states a given depth away
/// and link them to the initial placement in their sequence aka their "base Placement".
/// TODO: linking to base_placement
pub fn many_lookahead(start_game: Game, depth: u8) -> Vec<Game> {
    // base call of movegen on start_game, THIS WILL GENERATE BASE PLACEMENTS

    let mut b = Bot { game: start_game };

    let placements = b.move_gen(); // TODO use move_gen_1d
    let mut new_games = Vec::new();
    place_and_push(placements, &b.game, &mut new_games);

    let mut b = Bot { game: start_game };
    b.game.hold();
    let placements = b.move_gen(); // TODO use move_gen_1d
    place_and_push(placements, &b.game, &mut new_games);

    // repeatedly call lookahead, using the output as the input for the next iteration
    //let mut new_games = vec![start_game];
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
    let mut temp = Vec::new();
    games.par_iter()
        .map(|game| {
            let mut bot = Bot { game: *game };
            let mut games = place_and_return(bot.move_gen(), &bot.game);
            bot.game.hold();
            place_and_push(bot.move_gen(), &bot.game, &mut games);
            games
        }).collect_into_vec(&mut temp);
    itertools::concat(temp)
}

// helper methods---------------------------------------------

fn place_and_return(placements: HashSet<Piece>, base_game: &Game) -> Vec<Game> {
    let mut out: Vec<Game> = Vec::new();
    for placement in placements {
        let mut out_game = *base_game; // copy
        out_game.active = placement;
        out_game.place_active();
        out.push(out_game)
    }
    out
}

fn place_and_push(placements: HashSet<Piece>, base_game: &Game, push_to: &mut Vec<Game>) {
    for placement in placements {
        let mut out_game = *base_game; // copy
        out_game.active = placement;
        out_game.place_active();
        push_to.push(out_game);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_api::functions::*;

    // build and execute T spin

    // downstack

    // PC
}
