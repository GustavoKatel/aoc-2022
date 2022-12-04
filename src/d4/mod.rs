fn parse_range(input: &str) -> (i32, i32) {
    let mut values = input.split("-");

    (
        values.next().unwrap().parse().unwrap(),
        values.next().unwrap().parse().unwrap(),
    )
}

pub fn process() {
    let mut total_overlap = 0;

    for line in crate::read_input("d4_pt1.txt").lines() {
        let mut ranges = line.split(",");

        let range1 = parse_range(ranges.next().unwrap());
        let range2 = parse_range(ranges.next().unwrap());

        // pt1
        // if range1.0 >= range2.0 && range1.1 <= range2.1
        //     || range2.0 >= range1.0 && range2.1 <= range1.1
        // {
        //     total_overlap += 1;
        //     println!("overlap! range1: {:?}, range2: {:?}", range1, range2);
        // }

        // pt2
        if range1.0 <= range2.0 && range1.1 >= range2.0
            || range2.0 <= range1.0 && range2.1 >= range1.0
        {
            total_overlap += 1;
            println!("overlap! range1: {:?}, range2: {:?}", range1, range2);
        }
    }

    println!("overlaps: {}", total_overlap);
}
