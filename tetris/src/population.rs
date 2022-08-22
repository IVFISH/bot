use crate::players::*;
use crate::bot::*;
use std::cmp;

pub struct Population {
    // this can just be an array
    bots: Vec<Bot>,
}

impl Population {
    fn reset_population(&self) -> Self {
        let scores: Vec<f32> = self.bots.iter().map(
            |player| Self::cost(player)
        ).collect();

        let min_score = scores.iter().fold(f32::INFINITY, |a, &b| a.min(b));
        let num = scores.iter().position(|x| x == &min_score).unwrap();
        let best = &self.bots[num];
        let population_size = self.bots.len();

        let mut bots = Vec::new();
        for _ in 0..population_size {
            bots.push(best.give_birth());
        }

        Self {
            bots
        }
    }

    fn run_all(&mut self) {
        todo!()
    }

    fn cost(player: &Bot) -> f32 {
        // some measure of board state and versus stats
        todo!()
    }

}