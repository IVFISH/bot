use polynomial::Polynomial;

#[derive(Clone)]
pub struct Weights {
    pub height_weight: Polynomial<f32>,

    pub adjacent_height_differences_weight: Polynomial<f32>,
    pub total_height_difference_weight: Polynomial<f32>,
    pub num_hole_total_weight: Polynomial<f32>,
    pub num_hole_weighted_weight: Polynomial<f32>,
    pub holes_per_row_weighted_weight: Polynomial<f32>,
    pub cell_covered_weight: Polynomial<f32>,

    pub t_slot_weight: Polynomial<f32>,
    pub b2b_weight: Polynomial<f32>,
    pub combo_weight: Polynomial<f32>,
    pub damage_weight: Polynomial<f32>,
    pub clear_weight: Polynomial<f32>,

    pub panic_t_slot_weight: Polynomial<f32>,
    pub panic_b2b_weight: Polynomial<f32>,
    pub panic_combo_weight: Polynomial<f32>,
    pub panic_damage_weight: Polynomial<f32>,

    pub waste_t_weight: f32,
    pub tspin_reward: f32,
    pub tspin_reward_expo: f32,

    pub panic_waste_t_weight: f32,
    pub panic_tspin_reward: f32,
    pub panic_tspin_reward_expo: f32,
}

impl Default for Weights {
    fn default() -> Self {
        Self {
            // Default

            height_weight: Polynomial::new(vec![0.0, -40.0, 0.4, 0.8]),
            adjacent_height_differences_weight: Polynomial::new(vec![0.0, 3.0, 2.0]),
            total_height_difference_weight: Polynomial::new(vec![0.0, 0.0, 0.0]),
            num_hole_total_weight: Polynomial::new(vec![0.0, 10.0, 3.0]),
            num_hole_weighted_weight: Polynomial::new(vec![0.0, 30.0, 5.0]),
            holes_per_row_weighted_weight: Polynomial::new(vec![0.0, -100.0, 100.0]),
            cell_covered_weight: Polynomial::new(vec![0.0, 5.0]),
            t_slot_weight: Polynomial::new(vec![0.0, -400.0, 80.0]),

            b2b_weight: Polynomial::new(vec![0.0, -30.0, -2.0]),
            combo_weight: Polynomial::new(vec![0.0, 20.0, -2.0, -1.0]),
            damage_weight: Polynomial::new(vec![0.0, 25.0, -8.0, -1.5]),
            clear_weight: Polynomial::new(vec![0.0, -20.0]),

            waste_t_weight: 600.0,
            tspin_reward: 300.0,
            tspin_reward_expo: 2.0,

            // Panic

            panic_t_slot_weight: Polynomial::new(vec![0.0, -40.0, 20.0]),

            panic_b2b_weight: Polynomial::new(vec![0.0]),
            panic_combo_weight: Polynomial::new(vec![0.0, 10.0, -8.0, -1.5]),
            panic_damage_weight: Polynomial::new(vec![0.0, 25.0, -2.0, -1.5]),

            panic_waste_t_weight: 10.0,
            panic_tspin_reward: 100.0,
            panic_tspin_reward_expo: 1.0,
        }
    }
}