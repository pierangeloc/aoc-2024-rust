use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use regex::Regex;
use adv_code_2024::*;

const DAY: &str = "03"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");


    fn extract_muls(s: &str) -> Vec<i32> {
        let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
        let mut muls = vec![];
        for (_, [l, r]) in re.captures_iter(s).map(|c| c.extract()) {
            let il = l.parse::<i32>().unwrap();
            let ir = r.parse::<i32>().unwrap();
            muls.push(il * ir);
        }
        muls
    }
    fn part1<R: BufRead>(reader: R) -> Result<i32> {
        let answer = reader.lines().into_iter().map (|row| {
            let row = row.unwrap();
            let muls = extract_muls(&row);
            muls.iter().sum::<i32>()
        }).sum::<i32>();
        Ok(answer)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(161, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");


    #[derive(Debug)]
    enum Op {
        Mul(i32, i32),
        Do,
        Dont,
    }
    fn extract_ops(s: String) -> Vec<Op> {
        let general_re = Regex::new(r"(mul\(\d+,\d+\))|(do\(\))|(don't\(\))").unwrap();
        let  mul_re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

        let mut ops = vec![];

        for (_, [id]) in general_re.captures_iter(s.as_str()).map(|c| c.extract()) {
            println!("extract_ops ---  id={}", id);
            if id == "do()" {
                println!("pushing do()");
                ops.push(Op::Do);
            } else if id == "don't()" {
                println!("pushing don't()");
                ops.push(Op::Dont);
            } else  {
                for (_, [l, r]) in mul_re.captures_iter(id).map(|c| c.extract()) {
                    let il = l.parse::<i32>().unwrap();
                    let ir = r.parse::<i32>().unwrap();
                    println!("pushing mul({}, {})", il, ir);
                    ops.push(Op::Mul(il, ir));
                }
            }
        }
        ops
    }
    fn part2<R: BufRead>(reader: R) -> Result<i32> {
        let answer = reader.lines().into_iter().flat_map (|row| {
            let row = row.unwrap();
            let res = extract_ops(row);
            println!("part2 --- res={:?}", res);
            res
        }).into_iter().fold((0, true), |(acc, mul), op| {
            println!("folding: acc={}, mul={}, op={:?}", acc, mul, op);
            match op {
                Op::Mul(l, r) => if  mul {
                    (acc + l * r, mul)
                } else {
                    (acc, mul)
                },
                Op::Do => (acc, true),
                Op::Dont => (acc, false),
            }
        }).0;
        Ok(answer)    }

    assert_eq!(48, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
