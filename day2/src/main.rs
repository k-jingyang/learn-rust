use std::fs;

fn main() {

    let contents = fs::read_to_string("./input.txt").expect("Should have been able to read file");
    let lines = contents.split("\n");

    for line in lines  {

        
        if line != "" {
        } else {
        }
    }

}

fn calculate_score(line: &str) -> i32 {
    let fight = line.split(" ").collect();
    fight[0];
    fight[1];
    return 1
}
