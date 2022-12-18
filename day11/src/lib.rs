use fs_err as fs;
use std::collections::HashMap;

mod monkey;
use crate::monkey::monkey::Monkey;

pub fn run(filename: String) -> Result<(), Box<dyn std::error::Error>> {
    let contents = fs::read_to_string(filename)?;
    let component_lines = contents.lines().collect::<Vec<_>>();

    let mut monkeys: HashMap<u32, Monkey> = HashMap::new();
    let mut monkeyids: Vec<u32> = Vec::new();
    let mut divisors: Vec<u128> = Vec::new();

    for chunk in component_lines.chunks(7) {
        let monkey = Monkey::new(chunk).expect("unable to parse monkey");
        monkeyids.push(monkey.get_id());
        divisors.push(monkey.get_divisor());
        monkeys.insert(monkey.get_id(), monkey);
    }
    // we will get a common divisor for all the monkeys by multiplying their
    // divisors and use it to keep the worry levels from overflowing!
    let common_divisor: u128 = divisors.iter().fold(1, |acc, x| acc * x);
    println!("common divisor is : {}", common_divisor);

    for _ in 1..=10000 {
        for monkeyid in &monkeyids {
            let monkey = monkeys.get_mut(&monkeyid).expect("monkey has escaped!");
            let transfers = monkey.process_items(common_divisor);
            for (monkey_to_transfer_to, item_to_transfer) in transfers.iter() {
                let dst_monkey = monkeys
                    .get_mut(&monkey_to_transfer_to)
                    .expect("monkey has escaped!");
                dst_monkey.add_item(&item_to_transfer);
            }
        }
    }

    let mut inspection_counts: Vec<_> = monkeys
        .into_values()
        .map(|x| x.get_inspection_count())
        .collect();
    inspection_counts.sort();
    println!(
        "monkey business is {} * {} : {}",
        inspection_counts[inspection_counts.len() - 1],
        inspection_counts[inspection_counts.len() - 2],
        inspection_counts[inspection_counts.len() - 1]
            * inspection_counts[inspection_counts.len() - 2]
    );

    Ok(())
}
