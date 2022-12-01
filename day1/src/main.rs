use std::collections::HashMap;

const INPUT: &str = include_str!("input.txt");

fn puzzle1(elves_calories: &HashMap<usize, usize>) -> (usize, usize) {
    let mut max_calories = 0;
    let mut elf_id = 1;
    for (id, calories) in elves_calories.iter() {
        if calories > &max_calories {
            max_calories = *calories;
            elf_id = *id;
        }
    }

    (elf_id, max_calories)
}

fn puzzle2(elves_calories: &HashMap<usize, usize>) -> usize {
    // find sum of top 3 elves calories
    let mut top_calories = elves_calories.values().cloned().collect::<Vec<usize>>();
    top_calories.sort();
    top_calories.reverse();
    let top_calories_sum = top_calories[0] + top_calories[1] + top_calories[2];

    top_calories_sum
}

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

    let (elf_id, max_calories) = puzzle1(&elves_calories);
    println!("Elf {} caries the most calories: {}", elf_id, max_calories);
    let top_calories_sum = puzzle2(&elves_calories);
    println!("Top 3 elves calories sum: {}", top_calories_sum);
}

