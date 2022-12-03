pub fn process() {
    let mut max_calories: Vec<i32> = vec![0, 0, 0];

    let mut cumulative = 0;
    for line in crate::read_input("d1_pt1.txt").lines().chain([""]) {
        match line.parse::<i32>() {
            Ok(v) => {
                cumulative += v;
            }
            Err(_) => {
                if max_calories[0] < cumulative {
                    max_calories[0] = cumulative;
                    max_calories.sort();
                }

                cumulative = 0;
            }
        }
    }

    println!("{:?}", max_calories);
    println!("{}", max_calories.iter().sum::<i32>());
}
