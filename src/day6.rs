use std::fs;
use itertools::Itertools;
use std::collections::HashSet;
use rayon::prelude::*;

pub fn solve_part1(input: &str) -> usize {
    let customs_answers= input.split("\n\n").collect::<Vec::<&str>>().iter().fold(0, |mut acc, i| {
        let currGroup = i.chars().fold(HashSet::new(), |mut hash, question| {
            if question != '\n' {
                hash.insert(question);
            }
            hash });
        acc += currGroup.len();
        acc
    });
    customs_answers
}

pub fn solve_part2(input: &str) -> usize {
    let result = input.split("\n\n").collect::<Vec::<&str>>().iter().fold(0, |mut total, group| {
        let group_answers =  group.split_whitespace().collect::<Vec::<&str>>().iter().fold(Vec::<HashSet::<_>>::new(), |mut hashes, p| {
            let person_hash = p.chars().fold(HashSet::new(), |mut hash, question| {
                hash.insert(question);
                hash
            });
            hashes.push(person_hash);
            hashes
        });

        let same_answers = {if (group_answers.len() > 1)
        {
            group_answers.par_iter().cloned().reduce_with(|a, b| a.intersection(&b).cloned().collect()).unwrap().len()
        }
        else {
            group_answers[0].len()
        }};
        total += same_answers;
        total
    });
    
    result
}


pub fn solve() {
    let input : String = fs::read_to_string("inputs/input6.txt").unwrap();
    println!{"Part 1: {:?}\n Part 2: {:?}", solve_part1(&input), solve_part2(&input)};
}
            
