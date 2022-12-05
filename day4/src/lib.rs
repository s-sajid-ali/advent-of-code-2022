use fs_err as fs;
use nom::bytes::complete::tag;
use nom::character::complete::u32;
use nom::sequence::separated_pair;
use nom::IResult;
use std::collections::HashSet;
use std::error::Error;

fn parse_integer_pair(input: &str) -> IResult<&str, (u32, u32)> {
    separated_pair(u32, tag("-"), u32)(input)
}

fn parse(input: &str) -> IResult<&str, ((u32, u32), (u32, u32))> {
    separated_pair(parse_integer_pair, tag(","), parse_integer_pair)(input)
}

pub fn run_part2(filename: String) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(filename)?;
    let component_lines = contents.lines().collect::<Vec<_>>();

    let result: u32 = component_lines
        .iter()
        .map(|item| {
            if item.is_empty() {
                0
            } else {
                let (_, parsed) = parse(item).expect("parsing error, please check input");
                let (range1_start, range1_end) = parsed.0;
                let (range2_start, range2_end) = parsed.1;

                let set1: HashSet<u32> =
                    HashSet::from_iter((range1_start..=range1_end).collect::<Vec<_>>());
                let set2: HashSet<u32> =
                    HashSet::from_iter((range2_start..=range2_end).collect::<Vec<_>>());

                if set1.intersection(&set2).count() > 0 {
                    1
                } else {
                    0
                }
            }
        })
        .sum();

    println!("result is {}", result);

    Ok(())
}

pub fn run_part1(filename: String) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(filename)?;
    let component_lines = contents.lines().collect::<Vec<_>>();
    let result: u32 = component_lines
        .iter()
        .map(|item| {
            if item.is_empty() {
                0
            } else {
                let (_, parsed) = parse(item).unwrap();
                let (range1_start, range1_end) = parsed.0;
                let (range2_start, range2_end) = parsed.1;

                let range1_size = range1_end - range1_start;
                let range2_size = range2_end - range2_start;

                let set1: HashSet<u32> =
                    HashSet::from_iter((range1_start..=range1_end).collect::<Vec<_>>());
                let set2: HashSet<u32> =
                    HashSet::from_iter((range2_start..=range2_end).collect::<Vec<_>>());

                if range1_size < range2_size {
                    if set1.is_subset(&set2) {
                        1
                    } else {
                        0
                    }
                } else {
                    if set2.is_subset(&set1) {
                        1
                    } else {
                        0
                    }
                }
            }
        })
        .sum();

    println!("result is {}", result);

    Ok(())
}
