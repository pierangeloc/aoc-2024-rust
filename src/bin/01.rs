use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use num::ToPrimitive;
use adv_code_2024::*;

const DAY: &str = "01"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
3   4
4   3
2   5
1   3
3   9
3   3
"; // TODO: Add the test input

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn parse_line(l: &str) -> (i32, i32) {
        let mut parts = l.split_whitespace();
        let a = parts.next().unwrap().parse().unwrap();
        let b = parts.next().unwrap().parse().unwrap();
        (a, b)
    }
    fn part1<R: BufRead>(reader: R) -> Result<i32> {
        // TODO: Solve Part 1 of the puzzle
        let pairs: Vec<(i32, i32)> = reader.lines().into_iter().map(|l| {
            return parse_line(&l.unwrap());
        }).collect();
        let left_values = pairs.clone().into_iter().map(|p| p.0).sorted();
        let right_values = pairs.clone().into_iter().map(|p| p.1).sorted();
        let answer = left_values.zip(right_values).map(|(l, r)| (l - r).abs()).sum::<i32>();
        Ok(answer)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(11, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<i32> {
        let pairs: Vec<(i32, i32)> = reader.lines().into_iter().map(|l| {
            return parse_line(&l.unwrap());
        }).collect();
        let left_values = pairs.clone().into_iter().map(|p| p.0);

        let right_values  = pairs.clone().into_iter().map(|p| p.1);
        let mut acc: i32 = 0;
        for left_value in left_values {
            acc = acc + left_value * right_values.clone().filter(|&r| r == left_value).count().to_i32().unwrap();
        }
        Ok(acc)
    }

    assert_eq!(31, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
