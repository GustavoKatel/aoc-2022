use std::collections::HashMap;

#[derive(Hash, Debug, PartialEq, Eq, Clone)]
enum GameValue {
    R,
    P,
    S,
    X,
    Y,
    Z,
    Unknown(String),
}

fn calculate_shape(c1: GameValue, c2: GameValue) -> GameValue {
    match (c1, c2) {
        // lose
        (GameValue::R, GameValue::X) => GameValue::S,
        (GameValue::P, GameValue::X) => GameValue::R,
        (GameValue::S, GameValue::X) => GameValue::P,

        // draw
        (v, GameValue::Y) => v,

        // win
        (GameValue::R, GameValue::Z) => GameValue::P,
        (GameValue::P, GameValue::Z) => GameValue::S,
        (GameValue::S, GameValue::Z) => GameValue::R,

        (c1, c2) => panic!("unexpected combination: {:?}, {:?}", c1, c2),
    }
}

fn calculate_points(c1: GameValue, c2: GameValue) -> i32 {
    let score_map: HashMap<GameValue, i32> =
        HashMap::from([(GameValue::R, 1), (GameValue::P, 2), (GameValue::S, 3)]);

    let shape = calculate_shape(c1.clone(), c2.clone());

    println!("c1: {:?}, c2: {:?}, shape: {:?}", c1, c2, shape);

    let score = score_map[&shape]
        + match c2 {
            // lose
            GameValue::X => 0,
            // draw
            GameValue::Y => 3,
            // win
            GameValue::Z => 6,
            v => panic!("unexpected guide!: {:?}", v),
        };

    // pt1
    // score += match (c1, c2) {
    //     (GameValue::Unknown(v), _) => panic!("Unknown value: {}", v),
    //     (_, GameValue::Unknown(v)) => panic!("Unknown value: {}", v),
    //     (c1, c2) if c1 == c2 => 3,
    //     // rock
    //     (GameValue::R, GameValue::P) => 6,
    //     (GameValue::R, GameValue::S) => 0,
    //     // paper
    //     (GameValue::P, GameValue::R) => 0,
    //     (GameValue::P, GameValue::S) => 6,
    //     // scissors
    //     (GameValue::S, GameValue::R) => 6,
    //     (GameValue::S, GameValue::P) => 0,
    //     (c1, c2) => panic!("unexpected state: {:?}, {:?}", c1, c2),
    // };

    score
}

pub fn process() {
    let mut total_points: i32 = 0;

    for line in crate::read_input("d2_pt1.txt").lines() {
        if line.trim() == "" {
            break;
        }

        let columns: Vec<&str> = line.split(" ").collect();

        println!("line: {}, v: {:?}", line, columns);
        total_points += calculate_points(columns[0].into(), columns[1].into());
    }

    println!("{}", total_points);
}

impl From<&str> for GameValue {
    fn from(v: &str) -> Self {
        match v {
            "A" => GameValue::R,
            "B" => GameValue::P,
            "C" => GameValue::S,
            "X" => GameValue::X,
            "Y" => GameValue::Y,
            "Z" => GameValue::Z,
            v => GameValue::Unknown(v.into()),
        }
    }
}
