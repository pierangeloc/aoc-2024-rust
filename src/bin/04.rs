use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use num::ToPrimitive;
use adv_code_2024::*;

const DAY: &str = "04";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");


    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let lines_vec = reader.lines().flatten().collect::<Vec<String>>().iter().map(|line| line.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
        let h_size = lines_vec[0].len().to_i32().unwrap();
        let v_size = lines_vec.len().to_i32().unwrap();
        let char_in_pos = |i: i32, j: i32, lines_matrix: &Vec<Vec<char>>| -> char {
            if i >= 0 && i < v_size && j >= 0 && j < h_size {
                return lines_matrix[i.to_usize().unwrap()][j.to_usize().unwrap()];
            } else {
                '*'
            }
        };

        println!("Horizontal size: {}", h_size);
        println!("Vertical size: {}", v_size);
        let mut result = 0;
        for i in 0..v_size {
            for j in 0..h_size {
                let mut words = vec![];
                let n = (i - 3..i + 1).rev().map(|x| char_in_pos(x, j, &lines_vec)).collect::<String>();
                let e = (j..j + 4).map(|x| char_in_pos(i, x, &lines_vec)).collect::<String>();
                ;
                let s = (i..i + 4).map(|x| char_in_pos(x, j, &lines_vec)).collect::<String>();
                let w = (j - 3..j + 1).rev().map(|x| char_in_pos(i, x, &lines_vec)).collect::<String>();
                let ne = (i - 3..i + 1).rev().zip(j..j + 4).map(|(x, y)| char_in_pos(x, y, &lines_vec)).collect::<String>();
                let se = (i..i + 4).zip(j..j + 4).map(|(x, y)| char_in_pos(x, y, &lines_vec)).collect::<String>();
                let sw = (i..i + 4).zip((j - 3..j + 1).rev()).map(|(x, y)| char_in_pos(x, y, &lines_vec)).collect::<String>();
                let nw = (i - 3..i + 1).rev().zip((j - 3..j + 1).rev()).map(|(x, y)| char_in_pos(x, y, &lines_vec)).collect::<String>();
                words.push(n);
                words.push(e);
                words.push(s);
                words.push(w);
                words.push(ne);
                words.push(se);
                words.push(sw);
                words.push(nw);
                println!("Words around position ({}, {}): {:?}", i, j, words);
                result = result + words.iter().filter(|w| w.as_str().eq("XMAS")).collect_vec().len();
            }
        }

        Ok(result)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(18, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let lines_vec = reader.lines().flatten().collect::<Vec<String>>().iter().map(|line| line.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
        let h_size = lines_vec[0].len().to_i32().unwrap();
        let v_size = lines_vec.len().to_i32().unwrap();
        let char_in_pos = |i: i32, j: i32, lines_matrix: &Vec<Vec<char>>| -> char {
            if i >= 0 && i < v_size && j >= 0 && j < h_size {
                return lines_matrix[i.to_usize().unwrap()][j.to_usize().unwrap()];
            } else {
                '*'
            }
        };

        println!("Horizontal size: {}", h_size);
        println!("Vertical size: {}", v_size);
        let mut result = 0;
        for i in 0..v_size {
            for j in 0..h_size {
                let se = (i..i + 3).zip(j..j + 3).map(|(x, y)| char_in_pos(x, y, &lines_vec)).collect::<String>();
                let ne_from_below = (i..i + 3).rev().zip(j..j + 3).map(|(x, y)| char_in_pos(x, y, &lines_vec)).collect::<String>();
                if (se.as_str() == "MAS" || se.as_str() == "SAM") && (ne_from_below.as_str() == "MAS" || ne_from_below.as_str() == "SAM") {
                    result = result + 1;
                }
            }
        }

        Ok(result)
    }

    assert_eq!(9, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
