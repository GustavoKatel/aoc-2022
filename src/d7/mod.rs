use std::collections::{HashMap, VecDeque};

pub fn process() {
    let mut dir_sizes: HashMap<String, u32> = HashMap::new();

    let input = crate::read_input("d7_pt1.txt");
    let lines = input.lines().peekable();

    let mut current_dir = <VecDeque<String>>::new();

    for line in lines {
        println!("line: {}", line);

        if line.starts_with("$") {
            match line.replace("$ ", "").as_str() {
                "ls" => {}
                // cd <dir>
                l if l.starts_with("cd ") => {
                    let dir = l.replace("cd ", "");
                    if dir == ".." {
                        current_dir.pop_back();
                    } else {
                        current_dir.push_back(dir);
                    }
                    println!("\tcurrent_dir: {:?}", current_dir);
                }
                _ => {}
            }
            continue;
        }

        let ret = line.split_whitespace().collect::<Vec<&str>>();

        let mut tmp_dir: String = "".into();

        if let Ok(size) = ret[0].parse::<u32>() {
            for dir in current_dir.iter() {
                tmp_dir = tmp_dir + "/" + dir;

                let total_size = match dir_sizes.get(&tmp_dir) {
                    Some(v) => *v + size,
                    None => size,
                };
                dir_sizes.insert(tmp_dir.clone(), total_size);
            }
        }

        println!("\tsizes: {:?}", dir_sizes);
    }

    println!("final sizes: {:?}", dir_sizes);

    // pt1
    // let mut top_sum: u32 = 0;
    // for (dir, size) in dir_sizes {
    //     if size <= 100000 {
    //         top_sum += size;
    //         println!("\tdir: {}, size: {}", dir, size);
    //     }
    // }
    //
    // println!("sum: {}", top_sum);

    let mut possible_dirs: Vec<(String, u32)> = Vec::new();
    let total_space = dir_sizes["//"].clone();
    let unused_space = 70000000 - total_space;

    for (dir, size) in dir_sizes {
        if unused_space + size >= 30000000 {
            possible_dirs.push((dir, size));
        }
    }

    possible_dirs.sort_by(|a, b| a.1.cmp(&b.1));

    println!("possible dirs: {:?}", possible_dirs);

    println!("suggested dir size: {:?}", possible_dirs[0]);
}
