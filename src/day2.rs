use std::io::{prelude::*, BufReader};
use std::fs::File;
use std::collections::HashMap;
use itertools::Itertools;

pub fn solve_part1() -> i32 {
    let f = File::open("inputs/input2.txt").unwrap();
    let reader = BufReader::new(f);

    let mut input: Vec<String> = Vec::new();
    for l in reader.lines(){
        input.push(l.unwrap());
    }

    let valid_passwords = &input.iter().fold(0, |mut acc_valid, i| {
        let mut iter = i.split_whitespace();
        let (min, max) = iter.next().unwrap().split("-").next_tuple().unwrap();
        let letter = iter.next().unwrap().chars().next().unwrap();
        let pass = iter.next().unwrap().chars().fold(HashMap::new(), |mut acc, c| {
            *acc.entry(c).or_insert(0) += 1;
            acc
        });

        let min = min.to_string().parse::<u32>().unwrap();
        let max= max.to_string().parse::<u32>().unwrap();
    
        let occurrences = pass.get(&letter);
        match occurrences {
            Some(x) => {
                if x >= &min && x <= &max {
                    acc_valid += 1;
                }
            },
            None => {}
        }
        acc_valid
    });

    *valid_passwords
}

pub fn solve_part2() -> i32 {
    let f = File::open("inputs/input2.txt").unwrap();
    let reader = BufReader::new(f);

    let mut input: Vec<String> = Vec::new();
    for l in reader.lines(){
        input.push(l.unwrap());
    }

    fn char_matches(s: &str, letter: char, index: usize) -> bool {
        match s.chars().nth(index) {
            Some(x) => {
                if x == letter{
                    true
                }
                else {
                    false
                }
            }
            None => {false}
        }
    }

    let valid_passwords = &input.iter().fold(0, |mut acc_valid, i| {
        let mut iter = i.split_whitespace();
        let (min, max) = iter.next().unwrap().split("-").next_tuple().unwrap();
        let letter = iter.next().unwrap().chars().next().unwrap();

        let min = min.to_string().parse::<usize>().unwrap()-1;
        let max= max.to_string().parse::<usize>().unwrap()-1;

        let pass = iter.next().unwrap();
        if char_matches(pass, letter, min) ^ char_matches(pass, letter, max) {
            acc_valid +=1;
        }
        acc_valid
    });

    *valid_passwords
}

pub fn solve() {
    println!("Part 1: {:?}\n Part 2: {:?}", solve_part1(), solve_part2());
}

