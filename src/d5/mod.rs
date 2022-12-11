use std::{collections::VecDeque, iter::Peekable, str::Lines};

use crate::chunked::*;

fn read_draw(draw: &mut Vec<VecDeque<char>>, iter: &mut Peekable<Lines>) {
    while let Some(line) = iter.peek() {
        if line.starts_with(" 1") {
            break;
        }

        let line = iter.next().unwrap();

        let boxes = line
            .chars()
            .chunks(4)
            .map(|b| {
                if b.len() >= 2 {
                    return b[1];
                }
                ' '
            })
            .collect::<Vec<char>>();

        while draw.len() < boxes.len() {
            draw.push(VecDeque::new());
        }

        for (i, label) in boxes.into_iter().enumerate() {
            if label != ' ' {
                draw[i].push_front(label);
            }
        }
    }

    // remove number row
    if iter.peek().is_some() {
        iter.next();
        iter.next();
    }
}

pub fn process() {
    let mut draw: Vec<VecDeque<char>> = Vec::new();

    let input = crate::read_input("d5_pt1.txt");
    let mut lines = input.lines().peekable();

    read_draw(&mut draw, &mut lines);

    println!("len: {}", draw.len());
    println!("draw: {:?}", draw);

    for line in lines {
        println!("line: {}", line);
        let instructions = line
            .replace("move ", "")
            .replace("to ", "")
            .replace("from ", "")
            .split_whitespace()
            .map(|vs| vs.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        println!("instructions: {:?}", instructions);

        // part2
        let mut elements: VecDeque<char> = VecDeque::new();

        for _ in 0..instructions[0] {
            // part1
            //     let element = draw[instructions[1] - 1].pop_back().unwrap();
            //
            //     let stack_to = &mut draw[instructions[2] - 1];
            //
            //     stack_to.push_back(element);

            // part2
            elements.push_front(draw[instructions[1] - 1].pop_back().unwrap());
        }

        println!("elements: {:?}", elements);
        for element in elements {
            draw[instructions[2] - 1].push_back(element);
        }
    }

    for stack in draw {
        print!("{}", stack.back().unwrap());
    }

    println!("");
}
