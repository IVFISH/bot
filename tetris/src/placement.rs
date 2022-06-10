pub struct Placement {

    piece_type: char,
    rotation_state: i8,
    center: Point

}

impl Placement {

    pub fn to_list(&self) -> [Point; 4] {
        unimplemented!()
    }
}

pub struct Point {
    row: i8,
    col: i8
}