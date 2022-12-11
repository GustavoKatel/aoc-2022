use std::path::Path;

mod d1;
mod d2;
mod d3;
mod d4;
mod d5;
mod d6;
mod d7;

mod chunked;

pub fn read_input(name: &str) -> String {
    let pb = Path::new("inputs").join(name);

    std::fs::read_to_string(pb).unwrap()
}

fn main() {
    // d1::process();
    // d2::process();
    // d3::process();
    // d4::process();
    // d5::process();
    // d6::process();
    d7::process();
}
