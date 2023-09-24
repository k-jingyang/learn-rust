use std::fs;

fn main() {
    let contents = fs::read_to_string("./input.txt").expect("Should have been able to read file");
    let lines = contents.split("\n");

    let mut elf_calorie = 0;

    let mut elf_calories: Vec<i32> = Vec::new();

    for line in lines {
        if line != "" {
            let calories = line.trim().parse::<i32>().unwrap();
            elf_calorie += calories;
        } else {
            elf_calories.push(elf_calorie);
            elf_calorie = 0
        }
    }

    elf_calories.sort();

    let sum = elf_calories[elf_calories.len() - 3]
        + elf_calories[elf_calories.len() - 2]
        + elf_calories[elf_calories.len() - 1];
    println!("{}", sum)
}
