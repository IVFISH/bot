use polynomial::Polynomial;

#[derive(Clone)]
pub struct Weights {
    pub height_weight: Polynomial<f32>,

    pub adjacent_height_differences_weight: Polynomial<f32>,
    pub total_height_difference_weight: Polynomial<f32>,
    pub num_hole_total_weight: Polynomial<f32>,
    pub num_hole_weighted_weight: Polynomial<f32>,
    pub cell_covered_weight: Polynomial<f32>,

    pub t_slot_weight: Polynomial<f32>,
    pub b2b_weight: Polynomial<f32>,
    pub combo_weight: Polynomial<f32>,
    pub damage_weight: Polynomial<f32>,
    pub clear_weight: Polynomial<f32>,
}

impl Default for Weights {
    fn default() -> Self {
        Self {
            height_weight: Polynomial::new(vec![0.0, 2.0, 5.0]),
            adjacent_height_differences_weight: Polynomial::new(vec![0.0, 3.0, 2.0]),
            total_height_difference_weight: Polynomial::new(vec![0.0, 0.0, 0.0]),
            num_hole_total_weight: Polynomial::new(vec![0.0, 30.0, 5.0]),
            num_hole_weighted_weight: Polynomial::new(vec![0.0, 10.0, 3.0]),
            cell_covered_weight: Polynomial::new(vec![0.0, 5.0, 0.0]),

            t_slot_weight: Polynomial::new(vec![0.0, -150.0, 50.0]),
            b2b_weight: Polynomial::new(vec![0.0, -5.0, -5.0]),
            combo_weight: Polynomial::new(vec![0.0, 5.0, -3.0]),
            damage_weight: Polynomial::new(vec![0.0, 25.0, -8.0]),
            clear_weight: Polynomial::new(vec![0.0, 5.0, -5.0]),
        }
    }
}
