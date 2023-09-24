use std::fs;

fn main() {
    let contents = fs::read_to_string("./input.txt").expect("Should have been able to read file");
    let lines = contents.split("\n");
    let mut score = 0;
    for line in lines {
        let splitted: Vec<&str> = line.split(" ").collect();
        let match_score = get_score(splitted[0], splitted[1]);
        score += match_score
    }
    println!("{}", score)
}

fn get_score(opponent: &str, mine: &str) -> i32 {
    match opponent {
        "A" => match mine {
            "X" => 1 + 3,
            "Y" => 2 + 6,
            "Z" => 3 + 0,
            _ => {
                println!("Unexpected value");
                0
            }
        },
        "B" => match mine {
            "X" => 1 + 0,
            "Y" => 2 + 3,
            "Z" => 3 + 6,
            _ => {
                println!("Unexpected value");
                0
            }
        },
        "C" => match mine {
            "X" => 1 + 6,
            "Y" => 2 + 0,
            "Z" => 3 + 3,
            _ => {
                println!("Unexpected value");
                0
            }
        },
        _ => {
            println!("Unexpected value");
            0
        }
    }
}
