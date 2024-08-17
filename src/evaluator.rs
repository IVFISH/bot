use crate::board::Board;
use crate::game::Game;

pub trait Evaluator {
    /// constructor method
    fn new() -> Self;

    /// evaluates the game
    // TODO: seperate versus and positional eval
    fn eval(game: &Game) -> (i16, i16);
}

pub struct NoEvaluator {}

impl Evaluator for NoEvaluator {
    fn new() -> Self {
        Self {}
    }

    fn eval(_game: &Game) -> (i16, i16) {
        (0, 0)
    }
}

pub struct SimpleEvaluator {}

impl Evaluator for SimpleEvaluator {
    fn new() -> Self {
        Self {}
    }

    fn eval(game: &Game) -> (i16, i16) {
        let board = game.board.arr;
        let versus = game.versus;

        let max = Board::get_max_height(&board);
        let min = Board::get_min_height(&board);
        let holes = Board::get_total_holes(&board);

        let messiness = Board::get_messiness(&board);
        let coveredness = Board::get_cell_coveredness(&board);

        let adj_diff = Board::get_adjacent_height_differences(&board);
        let stack_diff = Board::get_stack_height_differences(&board);

        let tslot = Board::t_slot(&board);

        // let combo = versus.combo;
        // let attk = versus.total_attack;
        let combo = 0;
        let attk = 0;

        (
            5 * (max + min + messiness + adj_diff + stack_diff - 5 * tslot
                + (holes + coveredness) as usize
                - 2 * (combo + attk) as usize) as i16,
            -((2 * versus.combo + versus.attack_chain) as i16),
        )
    }
}
