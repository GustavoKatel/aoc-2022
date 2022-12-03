use std::collections::HashSet;

fn calc_priority(c: char) -> i32 {
    let c = c as i32;

    if c > 96 {
        return c - 96;
    }

    c - 38
}

pub struct ChunkIter<T: Iterator> {
    inner: T,

    chunk_size: usize,
}

impl<T: Iterator> Iterator for ChunkIter<T> {
    type Item = Vec<T::Item>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut v = Vec::new();

        for _ in 0..self.chunk_size {
            match self.inner.next() {
                Some(n) => v.push(n),
                None => break,
            }
        }

        if v.len() == 0 {
            None
        } else {
            Some(v)
        }
    }
}

pub trait Chunked<T: Iterator> {
    fn chunks(self, size: usize) -> ChunkIter<T>;
}

impl<T: Iterator> Chunked<T> for T {
    fn chunks(self, size: usize) -> ChunkIter<T> {
        ChunkIter {
            inner: self,
            chunk_size: size,
        }
    }
}

pub fn process() {
    let mut priority = 0;

    for group in crate::read_input("d3_pt1.txt").lines().chunks(3) {
        let priority_group: i32 = group
            .iter()
            .map(|line| line.chars().collect::<HashSet<char>>())
            .reduce(|acc, item| acc.intersection(&item).cloned().collect())
            .unwrap()
            .into_iter()
            .map(|v| {
                let p = calc_priority(v);
                println!("v: {:?}, priority: {}", v, p);
                p
            })
            .sum();

        priority += priority_group;

        // part 1
        // for line in group {
        //     println!("line: {}", line);
        //
        //     let mut priority_line = 0;
        //
        //     let (mut comp1, mut comp2) = line.split_at(line.len() / 2);
        //
        //     let comp1: HashSet<char> = comp1.chars().collect();
        //     let comp2: HashSet<char> = comp2.chars().collect();
        //
        //     println!("\tcomp1: {:?}, comp2: {:?}", comp1, comp2);
        //
        //     for c in comp2.intersection(&comp1) {
        //         println!("\tc: {}", c);
        //         priority_line += calc_priority(*c);
        //     }
        //
        //     priority += priority_line;
        // }
    }

    println!("priority: {}", priority);
}
