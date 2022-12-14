pub mod cpu {

    use std::collections::VecDeque;
    use std::fmt;

    #[derive(Copy, Clone, Debug)]
    pub struct State {
        pub cycle: u32,
        pub register: i32,
    }

    pub enum Instruction {
        Noop,
        Addx { v: i32 },
    }

    impl fmt::Display for State {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(
                f,
                "State : cycle: {}, register: {};",
                self.cycle, self.register
            )
        }
    }

    impl State {
        pub fn new(c1: u32, r1: i32) -> State {
            State {
                cycle: c1,
                register: r1,
            }
        }

        pub fn query(&self, queries: &mut VecDeque<u32>) -> Option<i64> {
            if Some(self.cycle) == queries.front().copied() {
                let signal_strength = (self.cycle as i32 * self.register) as i64;
                println!(
                    "Hit query at {}, signal_strength here is {}",
                    self.cycle, signal_strength
                );
                queries.pop_front();
                return Some(signal_strength);
            }
            None
        }
    }
}

pub mod render {
    use crate::crt::cpu::State;
    use std::fmt;

    const SCREENWIDTH: usize = 40;

    pub struct Screen {
        state: [char; SCREENWIDTH],
        activepixel: usize,
    }

    impl fmt::Display for Screen {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.state.iter().collect::<String>())
        }
    }

    impl Screen {
        pub fn new() -> Screen {
            Screen {
                state: ['.'; SCREENWIDTH],
                activepixel: 0,
            }
        }

        pub fn render(&mut self, cpu: &State) {
            let to_draw: i32 = (self.activepixel as i32)
                .checked_sub(cpu.register)
                .expect("subtraction overflow when comparing active pixel with cpu register");

            if to_draw.abs() <= 1 {
                //println!("drawing a # at {}, cpu state is {}", self.activepixel, cpu);
                self.state[self.activepixel] = '#';
            }

            if cpu.cycle % (SCREENWIDTH as u32) == 0 {
                println!("{}", self);
                self.state = ['.'; SCREENWIDTH];
            }

            self.activepixel = (self.activepixel + 1) % SCREENWIDTH;
        }
    }
}
