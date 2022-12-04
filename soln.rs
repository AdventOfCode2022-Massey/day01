// This program is licensed under the "MIT License".
// Please see the file LICENSE in this distribution
// for license terms.

//! Advent of Code Day 1.  
//! Bart Massey 2022

use aoc::*;

fn main() {
    let mut sums = vec![];
    let mut sum = 0u64;
    for line in input_lines() {
        if line.is_empty() {
            sums.push(sum);
            sum = 0;
        } else {
            sum += line.parse().unwrap_or_else(|e| {
                eprintln!("{line}: {e}");
                0
            });
        }
    }
    if sum > 0 {
        sums.push(sum);
    }

    match get_part() {
        Part1 => {
            if let Some(max_sum) = sums.into_iter().max() {
                println!("{max_sum}");
            } else {
                eprintln!("empty input");
            }
        }
        Part2 => {
            sums.sort();
            let n = sums.len();
            let max_sum = sums[n - 3..].iter().sum::<u64>();
            println!("{max_sum}");
        }
    }
}
