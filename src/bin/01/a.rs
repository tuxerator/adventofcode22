use std::fs;

fn main() {
    let file_path = "./input.txt";
    let contents = fs::read_to_string(file_path)
        .expect("Wasn't able to read file");

    let elves_food = contents.split("\n\n").map(|x| x.lines());

    let elves = elves_food.map(|x| x.collect::<Vec<&str>>());

    let mut elves_calories: Vec<i32> = Vec::new();

    for elve in elves {
        let calories: Vec<i32> = elve.iter().map(|x| x.parse::<i32>()
            .expect("Couldn't parse String!")).collect();
        let total_calories = sum(calories);
        elves_calories.push(total_calories);
    }

    let mut max_calories: i32 = 0;

    for _ in 0..3 {
        let calories = elves_calories.iter().max()
            .expect("No maximum found");

        dbg!(calories);

        max_calories += calories;

        let index = elves_calories.iter().position(|x| x.eq(calories))
            .expect("Element not found.");
        elves_calories.remove(index);
    }

    println!("{}", max_calories);
}

fn sum(vec: Vec<i32>) -> i32 {
    let mut sum: i32 = 0;
    for i in vec {
        sum += i;
    }
    sum
}
