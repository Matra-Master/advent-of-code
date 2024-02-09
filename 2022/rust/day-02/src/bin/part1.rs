// https://adventofcode.com/2022/day/2
// Opponent: A rock, B paper, C scissors
// Myself:   X rock, Y paper, Z scissors
//

use std::str::FromStr;

enum Move {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl FromStr for Move {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Move::Rock),
            "B" | "Y" => Ok(Move::Paper),
            "C" | "Z" => Ok(Move::Scissors),
            _ => Err("Incorrect input or move".to_string())
        }

    }
}

fn main() {
    let input = include_str!("./input1.txt");
    let output = resolve(input);
    dbg!(output);
}


fn resolve(input: &str) -> String {
    let result = input
        .lines()
        .map(|line| {
            let a_match = line
                .split(" ")
                .collect::<Vec<&str>>();
            let match_result = a_match[0].vs(a_match[1]);
        });
    "14".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXPECTED: &str = "15";

    #[test]
    fn it_works() {
        let result = resolve("A Y
B X
C Z");
        assert_eq!(result, EXPECTED.to_string());
    }
}
