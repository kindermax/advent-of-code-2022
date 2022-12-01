use std::collections::HashMap;

const INPUT: &str = include_str!("input.txt");

fn main() {
    let mut elves_calories: HashMap<usize, usize>= HashMap::new();
    let mut elf_id = 1;
    for line in INPUT.lines().into_iter() {
        if line.is_empty() {
            elf_id += 1;
            continue;
        }
        let calories = line.parse::<usize>().expect("Failed to parse calories");
        elves_calories.entry(elf_id).and_modify(|e| *e += calories).or_insert(calories);
    }

    let mut max_calories = 0;
    let mut elf_id = 1;
    for (id, calories) in elves_calories.iter() {
        if calories > &max_calories {
            max_calories = *calories;
            elf_id = *id;
        }
    }
    println!("Elf {} caries the most calories: {}", elf_id, max_calories);
}

