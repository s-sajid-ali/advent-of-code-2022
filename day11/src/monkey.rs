type MonkeyOp = fn(u128, Option<u128>) -> u128;

mod parse {

    use crate::monkey;
    use nom::bytes::complete::tag;
    use nom::bytes::complete::take_till;
    use nom::sequence::tuple;
    use nom::IResult;

    fn till_colon(s: &str) -> IResult<&str, &str> {
        take_till(|c| c == ':')(s)
    }

    pub fn parse_monkey_id(s: &str) -> IResult<&str, (&str, &str)> {
        tuple((tag("Monkey "), till_colon))(s)
    }

    pub fn parse_items(s: &str) -> IResult<&str, &str> {
        tag("Starting items:")(s)
    }

    pub fn parse_divisor(s: &str) -> IResult<&str, &str> {
        tag("Test: divisible by ")(s)
    }

    pub fn parse_operation_str(s: &str) -> IResult<&str, &str> {
        tag("Operation: new = old ")(s)
    }

    pub fn parse_true_monkey(s: &str) -> IResult<&str, &str> {
        tag("If true: throw to monkey ")(s)
    }

    pub fn parse_false_monkey(s: &str) -> IResult<&str, &str> {
        tag("If false: throw to monkey ")(s)
    }

    pub fn parse_operation(s: Vec<&str>) -> (Option<u128>, Option<monkey::MonkeyOp>) {
        assert_eq!(s.len(), 2);

        let y: Option<u128> = if s[1] == "old" {
            None
        } else {
            Some(s[1].parse::<u128>().expect("error parsing monkeyop"))
        };

        let monkeyop: Option<monkey::MonkeyOp> = match s[0] {
            "+" => Some(|x: u128, y: Option<u128>| match y {
                Some(yval) => x + yval,
                None => {
                    return x * 2;
                }
            }),
            "*" => Some(|x: u128, y: Option<u128>| match y {
                Some(yval) => x * yval,
                None => x.pow(2),
            }),
            _ => None,
        };

        return (y, monkeyop);
    }
}

pub mod monkey {

    use crate::monkey;
    use crate::monkey::parse;
    use std::collections::VecDeque;

    use std::fmt;

    pub struct Monkey {
        id: u32,
        inspections: u128,
        items: VecDeque<u128>,
        process: monkey::MonkeyOp,
        rhs: Option<u128>,
        divisor: u128,
        transfer_monkeys: (u32, u32),
    }

    impl fmt::Display for Monkey {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let contents: String = self
                .items
                .iter()
                .map(|id| id.to_string() + ",")
                .collect::<Vec<_>>()
                .join("")
                .into();
            write!(
                f,
                "id : {}, rhs: {}, divisor: {}, monkeys to transfer to : {},{}, items: {}",
                self.id,
                match &self.rhs {
                    Some(val) => val.to_string(),
                    None => "None".to_string(),
                },
                self.divisor,
                self.transfer_monkeys.0,
                self.transfer_monkeys.1,
                contents,
            )
        }
    }

    impl Monkey {
        pub fn get_divisor(&self) -> u128 {
            self.divisor
        }

        pub fn get_id(&self) -> u32 {
            self.id
        }

        pub fn get_inspection_count(&self) -> u128 {
            self.inspections
        }

        pub fn add_item(&mut self, item: &u128) {
            self.items.push_back(*item);
        }

        // process items currently held and return a hashmap of items
        // to give to other monkeys
        pub fn process_items(&mut self, common_divisor: u128) -> Vec<(u32, u128)> {
            let mut transfers: Vec<(u32, u128)> = Vec::with_capacity(self.items.len());
            while let Some(item) = self.items.pop_front() {
                self.inspections += 1;
                //let result: u32 = (self.process)(item, self.rhs) / 3;
                let result: u128 = ((self.process)(item, self.rhs))
                    .checked_rem(common_divisor)
                    .expect("division panic");
                if result.checked_rem(self.divisor).expect("division panic") == 0 {
                    transfers.push((self.transfer_monkeys.0, result));
                } else {
                    transfers.push((self.transfer_monkeys.1, result));
                }
            }
            transfers
        }

        pub fn new(lines: &[&str]) -> Option<Monkey> {
            if lines.len() == 6 || lines.len() == 7 {
                let (_, (_, id_str)) =
                    parse::parse_monkey_id(lines[0]).expect("error parsing monkey id!");
                let (items_str, _) =
                    parse::parse_items(lines[1].trim()).expect("error parsing monkey items");
                let (process_str, _) = parse::parse_operation_str(lines[2].trim())
                    .expect("error parsing monkey opeartion");

                let (rhs, monkeyop) =
                    parse::parse_operation(process_str.trim().split_whitespace().collect());

                let (divisor_str, _) =
                    parse::parse_divisor(lines[3].trim()).expect("error parsing divisor");

                let (true_monkey_str, _) = parse::parse_true_monkey(lines[4].trim())
                    .expect("unable to parse monkey to pass to!");

                let (false_monkey_str, _) = parse::parse_false_monkey(lines[5].trim())
                    .expect("unable to parse monkey to pass to!");

                Some(Monkey {
                    id: id_str
                        .parse::<u32>()
                        .expect("non-numeric monkey id encountered"),
                    inspections: 0,
                    items: items_str
                        .split(",")
                        .map(|x| {
                            x.trim()
                                .parse::<u128>()
                                .expect("non-numeric monkey starting item encountered")
                        })
                        .collect(),
                    process: monkeyop.expect("error parsing monkey op"),
                    rhs,
                    divisor: divisor_str
                        .parse::<u128>()
                        .expect("non-numeric divisor encountered"),
                    transfer_monkeys: (
                        true_monkey_str
                            .parse::<u32>()
                            .expect("non-numeric monkey to pass ti"),
                        false_monkey_str
                            .parse::<u32>()
                            .expect("non-numeric monkey to pass ti"),
                    ),
                })
            } else {
                eprintln!("could not parse monkey!");
                None
            }
        }
    }
}
