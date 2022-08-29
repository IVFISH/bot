use crate::bot::*;
use crate::players::*;

pub struct Population {
    // this can just be an array
    bots: Vec<Bot>,
}

impl Population {
    pub fn new(population_size: usize) -> Self {
        let bot = Bot::default();
        let mut bots = Vec::new();
        for _ in 0..population_size {
            bots.push(bot.give_birth());
        }

        Self {
            bots
        }

    }

    fn get_best(&self) -> usize {
        let scores: Vec<f32> = self.bots.iter().map(|player| Self::cost(player)).collect();
        let min_score = scores.iter().fold(f32::INFINITY, |a, &b| a.min(b));
        scores.iter().position(|x| x == &min_score).unwrap()
    }

    fn reset_population(&mut self) {

        let best = &self.bots[self.get_best()];
        let population_size = self.bots.len();

        let mut bots = Vec::new();
        for _ in 0..population_size {
            bots.push(best.give_birth());
        }

        self.bots = bots;
    }

    fn run_all(&mut self, num_pieces: usize)
    {
        // This runs all the bots in the population for a specified number of pieces
        for mut bot in &mut self.bots {
            bot.make_n_moves(num_pieces);
        }
    }

    fn cost(player: & Bot) -> f32 {
        // some measure of board state and versus stats
        player.get_game().game_data.app()
    }

    fn save_best_to_json(&self, generation: usize) {
        let best = &self.bots[self.get_best()];
        best.weight.to_json(format!("./weights/{}.json", generation));
    }

    pub fn train(&mut self, num_generations: usize, num_pieces: usize, save_interval: usize) {
        for generation in 1..=num_generations {
            self.run_all(num_pieces);
            if generation % save_interval == 0 {
                self.save_best_to_json(generation);
            }
            self.reset_population();
            println!("Finished generation {}.", generation);
        }
    }
}
