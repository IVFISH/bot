use crate::constants::versus_constants::*;
use crate::game::game_rules_and_data::GameData;

// d, t, q, ts, td, tt
const ATTACK_TYPE_CONVERSION: [usize; 7] = [1, 2, 4, 2, 4, 6, 0];

fn attack_type_to_index(attack: AttackType) -> usize {
    match attack {
        AttackType::D => 0,
        AttackType::T => 1,
        AttackType::Q => 2,
        AttackType::TS => 3,
        AttackType::TD => 4,
        AttackType::TT => 5,
        _ => 6,
    }
}

const BACK_TO_BACK_CONVERSION: [f32; 8] = [1.0, 1.25, 1.5, 1.75, 2.0, 2.25, 2.5, 2.75];
const WEIRD_DAMAGE_TABLE: [[usize; 20]; 3] = [
    [0, 0, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 3, 3, 3, 3],
    [1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5],
    [0; 20],
];

fn attack_type_to_index_special(attack: AttackType) -> usize {
    match attack {
        AttackType::S => 0,
        AttackType::TSM => 0,
        AttackType::TDM => 1,
        _ => 2,
    }
}

pub const BACK_TO_BACK_TYPES: [AttackType; 6] = [
    AttackType::TS,
    AttackType::TD,
    AttackType::TT,
    AttackType::TSM,
    AttackType::TDM,
    AttackType::Q,
];

fn b2b_to_level(chain_length: i8) -> usize {
    match chain_length {
        0 => 0,
        1..=2 => 1,
        3..=7 => 2,
        8..=23 => 3,
        24..=66 => 4,
        67..=100 => 5,
        _ => 6,
    }
}

pub fn calc_damage(game: &mut GameData, attack_type: AttackType, num_cleared: usize) -> usize {
    if num_cleared == 0 {
        return 0;
    }

    let combo = game.combo as usize;
    let b2b = game.b2b;
    let all_clear_damage = 10 * game.all_clear as usize;

    let without_b2b = (combo + 4) * ATTACK_TYPE_CONVERSION[attack_type_to_index(attack_type)] / 4
        + WEIRD_DAMAGE_TABLE[attack_type_to_index_special(attack_type)][combo];

    (without_b2b as f32 * BACK_TO_BACK_CONVERSION[b2b_to_level(b2b)]) as usize + all_clear_damage
}

pub fn attack_type(t_spin: TSpinType, lines_cleared: usize) -> AttackType {
    match lines_cleared {
        0 => AttackType::None,
        4 => AttackType::Q,
        3 => match t_spin {
            TSpinType::None => AttackType::T,
            _ => AttackType::TT,
        },
        2 => match t_spin {
            TSpinType::None => AttackType::D,
            TSpinType::Mini => AttackType::TDM,
            _ => AttackType::TD,
        },
        _ => match t_spin {
            TSpinType::None => AttackType::S,
            TSpinType::Mini => AttackType::TSM,
            _ => AttackType::TS,
        },
    }
}
