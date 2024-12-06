use std::cmp::Ordering;
use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use num::ToPrimitive;
use adv_code_2024::*;

const DAY: &str = "05";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<i32> {
        let mut rules: Vec<(i32, i32)> = vec![];
        let mut updates: Vec<Vec<i32>> = vec![];
        let mut parsing_phase = 0;

        reader.lines().flatten().for_each(|line| {
            if line.is_empty() {
                parsing_phase += 1;
                return;
            }
            if parsing_phase == 0 {
                let parts = line.split('|').collect::<Vec<&str>>();
                let rule = (parts[0].parse::<i32>().unwrap(), parts[1].parse::<i32>().unwrap());
                rules.push(rule);
            } else {
                let update = line.split(',').map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
                updates.push(update);
            }
        });
        println!("input parsed; rules: {:?}\n updates: {:?}", rules, updates);


        fn is_pair_valid(pair: &(&i32, &i32), rules: &Vec<(i32, i32)>) -> bool {
            if rules.contains(&(*(pair.0), *(pair.1))) {
                return true;
            } else if rules.contains(&(*(pair.1), *(pair.0))) {
                return false;
            } else {
                return true;
            }
        }
        fn is_valid_update(update: &Vec<i32>, rules: &Vec<(i32, i32)>) -> i32 {
            let pairs = update.iter().zip(update.iter().skip(1)).collect::<Vec<(&i32, &i32)>>();
            let is_valid = pairs.iter().all(|pair| is_pair_valid(pair, rules));
            if is_valid {
                let ix = ((*update).len() - 1) / 2;
                update[ix]
            } else {
                0
            }
        }

        let mut res = 0;
        for update in &updates {
            let valid = is_valid_update(update, &rules);
            println!("Update {:?} is valid: {}", update, valid);
            res = res + valid;
        }

        Ok(res)
    }

    assert_eq!(143, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    // region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<i32> {
        let mut rules: Vec<(i32, i32)> = vec![];
        let mut updates: Vec<Vec<i32>> = vec![];
        let mut parsing_phase = 0;

        reader.lines().flatten().for_each(|line| {
            if line.is_empty() {
                parsing_phase += 1;
                return;
            }
            if parsing_phase == 0 {
                let parts = line.split('|').collect::<Vec<&str>>();
                let rule = (parts[0].parse::<i32>().unwrap(), parts[1].parse::<i32>().unwrap());
                rules.push(rule);
            } else {
                let update = line.split(',').map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
                updates.push(update);
            }
        });
        println!("input parsed; rules: {:?}\n updates: {:?}", rules, updates);


        fn is_pair_valid(pair: &(&i32, &i32), rules: &Vec<(i32, i32)>) -> bool {
            if rules.contains(&(*(pair.0), *(pair.1))) {
                return true;
            } else if rules.contains(&(*(pair.1), *(pair.0))) {
                return false;
            } else {
                return true;
            }
        }
        fn sort_invalid_update_and_output_middle(update: &Vec<i32>, rules: &Vec<(i32, i32)>) -> i32 {
            let mut update_clone = update.clone();
            update_clone.sort_by(|a, b| if is_pair_valid(&(a, b), rules) { Ordering::Less } else { Ordering::Greater });

            if update_clone == *update {
                0
            } else {
                let ix = ((*update).len() - 1) / 2;
                update_clone[ix]
            }
        }

        let mut res = 0;
        for update in &updates {
            let valid = sort_invalid_update_and_output_middle(update, &rules);
            println!("Update {:?} extract: {}", update, valid);
            res = res + valid;
        }

        Ok(res)
    }

    assert_eq!(123, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    // //endregion

    Ok(())
}
