use crate::placement::piece_data::*;

pub struct Suggestion {
    pub input_list: Vec<String>,
    pub info: String,
}

impl Suggestion {
    pub fn new(input_list: Vec<String>, info: String) -> Self {
        Self { input_list, info }
    }
}
