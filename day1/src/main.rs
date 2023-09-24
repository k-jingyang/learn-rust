use std::fs;

fn main() {

    let contents = fs::read_to_string("./input.txt").expect("Should have been able to read file");
    let mut max = 0;
    let lines = contents.split("\n");

    let mut elf_calorie = 0;
    for line in lines  {

        if line != "" {
            let calories = line.trim().parse::<i32>().unwrap();
            elf_calorie += calories;
            // println!("{}", calories);
        } else {
            if elf_calorie > max {
                max = elf_calorie
            }
            elf_calorie = 0
        }
    }

    println!("{}", max)

}

