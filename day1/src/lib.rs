use std::error::Error;
use std::fs;

pub fn run(filename: String) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(filename)?;
    let mut component_lines = contents.lines();

    let mut counts: Vec<i64> = Vec::new();
    let mut calories: i64 = 0;

    loop {
        if let Some(item) = component_lines.next() {
            if let Ok(calorie) = item.parse::<i64>() {
                calories = calories + calorie;
            }
            if item.is_empty() {
                counts.push(calories);
                calories = 0;
            }
            continue;
        }
        break;
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
