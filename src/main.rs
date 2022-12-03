use std::path::Path;

mod d1;
mod d2;
mod d3;

pub fn read_input(name: &str) -> String {
    let pb = Path::new("inputs").join(name);

    std::fs::read_to_string(pb).unwrap()
}

fn main() {
    // d1::process();
    // d2::process();
    d3::process();
}
