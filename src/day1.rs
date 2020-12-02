use std::io::{self, prelude::*, BufReader};
use std::fs::File;
use itertools::Itertools;

pub fn solve_part1() -> Option<i32>{
    let f = File::open("inputs/input1.txt").unwrap();
    let reader = BufReader::new(f);

    let mut nums: Vec<i32> = Vec::new();
    for l in reader.lines(){
        nums.push(l.unwrap().parse().unwrap());
    }
    nums.sort();

    let mut reversed = nums.clone();
    reversed.reverse();

    'outer: for num_reversed in reversed.iter()
    {
        for num in nums.iter()
        {
            let sum = num + num_reversed;
            if (sum == 2020)
            {
                return Some(num*num_reversed)
            }
            else if sum > 2020
            {
                break;
            }
        }
    }
    None
}

pub fn solve_part2() -> Option<i32> {
    let f = File::open("inputs/input1.txt").unwrap();
    let reader = BufReader::new(f);

    let mut nums: Vec<i32> = Vec::new();
    for l in reader.lines(){
        nums.push(l.unwrap().parse().unwrap());
    }

    nums.sort();

    for c in nums.iter().combinations(3)
    {
        if (c[0] + c[1] + c[2]) == 2020 {
            return Some(c[0] * c[1] * c[2]);
        }
    }
    None
}

pub fn solve() {
    println!("Part 1: {:?}, Part 2: {:?} \n", solve_part1().unwrap(), solve_part2().unwrap());;
}

