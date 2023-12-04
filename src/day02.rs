use crate::utils;
use regex::Regex;

const REGEX_NTH_GAME: &str = r"Game (\d+):";
const REGEX_AMOUNT_CUBES: &str = r"(\d+) (blue|green|red)";

pub fn main() {
    let input = utils::string_from_file("02.txt");
    let mut result = 0;

    for line in input.lines() {
        let r_nth_game = Regex::new(REGEX_NTH_GAME).unwrap();
        let sets: String = r_nth_game.replace(line, "").into();
        result += minimum_cubes_game(sets.as_str());
    }

    println!("{}", result);
}

fn minimum_cubes_game(sets_cubes: &str) -> u32 {
    let r_amount_cubes = Regex::new(REGEX_AMOUNT_CUBES).unwrap();
    let sets: Vec<&str> = sets_cubes.split(';').collect();
    let mut minimum_blue: u32 = 0;
    let mut minimum_red: u32 = 0;
    let mut minimum_green: u32 = 0;

    for set_cubes in sets {
        let set: Vec<&str> = set_cubes.split(',').collect();
        for nth_cubes in set {
            let cap = r_amount_cubes.captures(nth_cubes).unwrap();
            let number = cap.get(1).unwrap().as_str().parse::<u32>().unwrap();
            let color = cap.get(2).unwrap().as_str();
            match color {
                "red" => {
                    if number > minimum_red {
                        minimum_red = number;
                    }
                }
                "blue" => {
                    if number > minimum_blue {
                        minimum_blue = number;
                    }
                }
                "green" => {
                    if number > minimum_green {
                        minimum_green = number;
                    }
                }
                _ => panic!("Wrong color"),
            }
        }
    }

    minimum_green * minimum_blue * minimum_red
}
