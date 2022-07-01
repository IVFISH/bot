use crate::placement::piece_data::*;

struct Suggestion {
    input_list: Vec<String>,
    num_pieces_placed: usize,
    info: String,
}

impl Suggestion {
    pub fn new(input_list: Vec<String>, num_pieces_placed: usize, info: String) -> Self {
        Self {
            input_list,
            num_pieces_placed,
            info,
        }
    }
}
