use crate::{Part, Solution};

pub mod part1;
pub mod part2;
mod shared;

pub fn get_input() -> &'static str {
    include_str!("./input/input.txt")
}

pub fn run(part: Part) -> Solution {
    let input = get_input();
    match part {
        Part::One => part1::solve(input),
        Part::Two => part2::solve(input),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        assert_eq!(run(Part::One), "TBVFVDZPN".into());
    }

    #[test]
    fn part2_works() {
        assert_eq!(run(Part::Two), "VLCWHTDSZ".into());
    }
}