use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use adv_code_2024::*;

const DAY: &str = "02"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";

fn main() -> Result<()> {
    start_day(DAY);

    println!("=== Part 1 ===");

    fn is_safe(levels: Vec<i32>) -> bool {
        let mut sort_asc = levels.clone();
        sort_asc.sort();
        let mut sort_desc = levels.clone();
        sort_desc.sort_by(|a, b| b.cmp(a));
        levels.iter().zip(levels.clone().iter().tail(levels.len() - 1)).all(
            |(a, b)| {
                a <= b
            }
        );

        let res = (sort_desc == levels || sort_asc == levels) && levels.iter().zip(levels.clone().iter().tail(levels.len() - 1)).all(
            |(a, b)| {
                let delta = (b - a).abs();
                delta >= 1 && delta <= 3
            }
        );

        res
    }

    fn line_to_levels(line: &str) -> Vec<i32> {
        line.split_whitespace().map(|level| level.parse().unwrap()).collect()
    }

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        // TODO: Solve Part 1 of the puzzle
        let answer = reader.lines().into_iter()
            .map(|l| {
                let levs = line_to_levels(&l.unwrap());
                is_safe(levs)
            })
            .filter(|safe| *safe).count();
        Ok(answer)
    }

    assert_eq!(2, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);

    println!("\n=== Part 2 ===");

    fn sub_levels(levs: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        for i in 0..levs.len() {
            let mut sub = levs.clone();
            sub.remove(i);
            result.push(sub);
        }
        result
    }

    fn is_safe_tolerant(levs: Vec<i32>) -> bool {
        let res = is_safe(levs.clone()) || sub_levels(levs.clone()).iter().any(|levs| is_safe(levs.to_vec()));
        println!("levels {:?} ...is safe? {}", levs, res);
        res
    }
    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let answer = reader.lines().into_iter()
            .map(|l| {
                line_to_levels(&l.unwrap())
            })
            .map(|levs| is_safe_tolerant(levs))
            .filter(|safe| *safe).count();

        Ok(answer)
    }

    assert_eq!(4, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);

    Ok(())
}
