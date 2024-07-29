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

    pub perfect_clear_weight: f32,
    pub tspin_weight: f32,
    pub filled_tsd_weight: f32,
    pub too_many_t_slot_weight: f32,

    pub clear_single_weight: f32,
    pub clear_double_weight: f32,
    pub clear_triple_weight: f32,
    pub clear_tetris_weight: f32,
}

impl Default for Weights {
    fn default() -> Self {
        Self::from_params(&vec![
            10.0,
            4.0458274,
            -4.0797834,
            0.8254502,
            -0.32339233,
            10.0,
            -54.007343,
            -10.0,
            4.6619377,
            -9.993479,
            -10.0,
            261.80038,
            0.0,
        ])
        // Self {
        //     height_weight: Polynomial::new(vec![0.0, -10.0, 5.0]),
        //     adjacent_height_differences_weight: Polynomial::new(vec![0.0, 3.0, 2.0]),
        //     total_height_difference_weight: Polynomial::new(vec![0.0, 0.0, 0.0]),
        //     num_hole_total_weight: Polynomial::new(vec![0.0, 30.0, 5.0]),
        //     num_hole_weighted_weight: Polynomial::new(vec![0.0, 10.0, 3.0]),
        //     cell_covered_weight: Polynomial::new(vec![0.0, 5.0, 0.0]),

        //     t_slot_weight: Polynomial::new(vec![0.0, -150.0, 50.0]),
        //     b2b_weight: Polynomial::new(vec![0.0, -15.0]),
        //     combo_weight: Polynomial::new(vec![0.0, 8.0, -4.0]),
        //     damage_weight: Polynomial::new(vec![0.0, 28.0, -8.0]),
        //     clear_weight: Polynomial::new(vec![0.0, 49.0, -7.0]),

        //     perfect_clear_weight: -1000000.0,
        //     tspin_weight: -100.0,
        // }
    }
}

impl Weights {
    pub fn from_params(param: &Vec<f32>) -> Self {
        Weights {
            height_weight: Polynomial::new(vec![0.0, param[0]]),
            adjacent_height_differences_weight: Polynomial::new(vec![0.0, param[1]]),
            total_height_difference_weight: Polynomial::new(vec![0.0, param[2]]),
            num_hole_total_weight: Polynomial::new(vec![0.0, param[3]]),
            num_hole_weighted_weight: Polynomial::new(vec![0.0, param[4]]),
            cell_covered_weight: Polynomial::new(vec![0.0, param[5]]),

            t_slot_weight: Polynomial::new(vec![0.0, param[6]]),
            b2b_weight: Polynomial::new(vec![0.0, param[7]]),
            combo_weight: Polynomial::new(vec![0.0, param[8]]),
            damage_weight: Polynomial::new(vec![0.0, param[9]]),
            clear_weight: Polynomial::new(vec![0.0, param[10]]),

            perfect_clear_weight: param[11],
            tspin_weight: param[12],

            filled_tsd_weight: param[13],
            too_many_t_slot_weight: param[14],

            clear_single_weight: param[15],
            clear_double_weight: param[16],
            clear_triple_weight: param[17],
            clear_tetris_weight: param[18],
        }
    }
}
