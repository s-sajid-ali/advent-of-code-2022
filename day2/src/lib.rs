use std::error::Error;
use std::fs;

//A for Rock, B for Paper, and C for Scissors
//X for Rock, Y for Paper, and Z for Scissors

enum PlayChoice {
    Rock,
    Paper,
    Scissors,
}

enum GameChoice {
    Win,
    Draw,
    Lose,
}

fn letter_to_play_choice(input: char) -> Option<PlayChoice> {
    if (input == 'A') | (input == 'X') {
        return Some(PlayChoice::Rock);
    } else if (input == 'B') | (input == 'Y') {
        return Some(PlayChoice::Paper);
    } else if (input == 'C') | (input == 'Z') {
        return Some(PlayChoice::Scissors);
    }

    None
}

fn letter_to_game_choice(input: char) -> Option<GameChoice> {
    if input == 'X' {
        return Some(GameChoice::Lose);
    } else if input == 'Y' {
        return Some(GameChoice::Draw);
    } else if input == 'Z' {
        return Some(GameChoice::Win);
    }

    None
}

fn score_game(player1: PlayChoice, game: GameChoice) -> i32 {
    let opponent_choice: i32 = match player1 {
        PlayChoice::Rock => 1,
        PlayChoice::Paper => 2,
        PlayChoice::Scissors => 3,
    };

    let score_for_winning = 6;
    let score_for_draw = 3;

    match game {
        GameChoice::Lose => {
            if opponent_choice == 1 {
                return 3;
            } else if opponent_choice == 2 {
                return 1;
            } else {
                return 2;
            }
        }
        GameChoice::Draw => {
            if opponent_choice == 1 {
                return 1 + score_for_draw;
            } else if opponent_choice == 2 {
                return 2 + score_for_draw;
            } else {
                return 3 + score_for_draw;
            }
        }
        GameChoice::Win => {
            if opponent_choice == 1 {
                return 2 + score_for_winning;
            } else if opponent_choice == 2 {
                return 3 + score_for_winning;
            } else {
                return 1 + score_for_winning;
            }
        }
    }
}

#[allow(dead_code)]
fn score_plays(player1: PlayChoice, player2: PlayChoice) -> i32 {
    let score_played_choice: i32 = match player2 {
        PlayChoice::Rock => 1,
        PlayChoice::Paper => 2,
        PlayChoice::Scissors => 3,
    };

    let opponent_choice: i32 = match player1 {
        PlayChoice::Rock => 1,
        PlayChoice::Paper => 2,
        PlayChoice::Scissors => 3,
    };

    let combined_choice = score_played_choice * opponent_choice;

    let score_for_winning = 6;
    let score_for_draw = 3;

    // check if the match was a draw
    if (combined_choice == 1) | (combined_choice == 4) | (combined_choice == 9) {
        return score_played_choice + score_for_draw;
    }

    // rock + scissors
    if combined_choice == 3 {
        if score_played_choice == 1 {
            return score_played_choice + score_for_winning;
        }
    }

    // rock + paper
    if combined_choice == 2 {
        if score_played_choice == 2 {
            return score_played_choice + score_for_winning;
        }
    }

    // paper + scissors
    if combined_choice == 6 {
        if score_played_choice == 3 {
            return score_played_choice + score_for_winning;
        }
    }

    return score_played_choice;
}

pub fn run(filename: String) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(filename)?;
    let mut component_lines = contents.lines();

    let mut scores: Vec<i32> = Vec::new();

    loop {
        if let Some(item) = component_lines.next() {
            let mut iter = item.split_whitespace();
            // break on empty line, means we have reached the end of the game sequence!
            if item.is_empty() {
                break;
            }
            let element = iter.next().unwrap();
            let elem1: char = element.parse::<char>().unwrap();
            let element = iter.next().unwrap();
            let elem2: char = element.parse::<char>().unwrap();

            let play1: PlayChoice = letter_to_play_choice(elem1).expect("invalid input choice!");
            let game: GameChoice = letter_to_game_choice(elem2).expect("invalid input choice!");

            //let this_round_score: i32 = score_plays(play1, play2);
            let this_round_score: i32 = score_game(play1, game);

            scores.push(this_round_score);

            continue;
        }
        break;
    }

    println!(
        "Total score for this set of games is {}",
        scores.iter().sum::<i32>()
    );

    Ok(())
}
