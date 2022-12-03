use fs_err as fs;
use std::error::Error;

pub fn run_iter(filename: String) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(filename)?;
    let component_lines = contents.lines().collect::<Vec<_>>();
    let component_groups = component_lines.split(|x| x.is_empty()).collect::<Vec<_>>();
    let mut calories: Vec<i64> = component_groups
        .into_iter()
        .map(|x| {
            x.into_iter()
                .map(|y| y.parse::<i64>().unwrap())
                .sum::<i64>()
        })
        .collect();

    // sort the calories vector
    calories.sort();

    println!(
        "elf with most calories has {} calories",
        calories[calories.len() - 1]
    );

    println!(
        "elf with second most calories has {} calories",
        calories[calories.len() - 2]
    );

    println!(
        "elf with third most calories has {} calories",
        calories[calories.len() - 3]
    );

    println!(
        "three elves with most calories have {} calories combined",
        calories[calories.len() - 1] + calories[calories.len() - 2] + calories[calories.len() - 3]
    );

    Ok(())
}

#[allow(dead_code)]
pub fn run(filename: String) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(filename)?;
    let component_lines = contents.lines();

    let mut counts: Vec<i64> = Vec::new();
    let mut calories: i64 = 0;

    for item in component_lines {
        if let Ok(calorie) = item.parse::<i64>() {
            calories = calories + calorie;
        }
        if item.is_empty() {
            counts.push(calories);
            calories = 0;
        }
        continue;
    }

    // sort the counts vector
    counts.sort();

    println!(
        "elf with most calories has {} calories",
        counts[counts.len() - 1]
    );

    println!(
        "elf with second most calories has {} calories",
        counts[counts.len() - 2]
    );

    println!(
        "elf with third most calories has {} calories",
        counts[counts.len() - 3]
    );

    println!(
        "three elves with most calories have {} calories combined",
        counts[counts.len() - 1] + counts[counts.len() - 2] + counts[counts.len() - 3]
    );

    Ok(())
}
