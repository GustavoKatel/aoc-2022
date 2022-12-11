use std::collections::{HashSet, VecDeque};

// pt1
// const MARKER_LEN: usize = 4;
// pt2
const MARKER_LEN: usize = 14;

pub fn process() {
    let input = crate::read_input("d6_pt1.txt");
    let lines = input.lines().peekable();

    for (i, line) in lines.enumerate() {
        println!("{}: line: {}", i, line);

        let mut offset = 0;

        let iter = line.chars().peekable();

        let mut marker: VecDeque<char> = VecDeque::new();
        let mut marker_set: HashSet<char> = HashSet::new();

        for ch in iter {
            if marker_set.len() == MARKER_LEN {
                break;
            }

            if marker.len() == MARKER_LEN {
                marker.pop_front();
            }

            marker.push_back(ch);
            marker_set.clear();
            for ch in marker.iter() {
                marker_set.insert(*ch);
            }
            offset += 1;

            // println!("\tmarker: {:?}", marker);
            // println!("\tset: {:?}", marker_set);
        }

        println!("\tmarker: {:?}", marker);
        println!("\tset: {:?}", marker_set);
        println!("\toffset: {:?}", offset);
    }
}
