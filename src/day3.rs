use std::io::{prelude::*, BufReader};
use std::fs::File;

pub fn parse_input(filename: &str) -> Vec<String> {
    let f = File::open(filename).unwrap();
    let reader = BufReader::new(f);

    let mut input: Vec<String> = Vec::new();
    for l in reader.lines(){
        input.push(l.unwrap());
    }
    input
}

pub fn traverse(input: &Vec<String>, x: usize, y: usize) -> i32 {
    let num_of_trees = &input.iter().step_by(y).skip(1).fold((0,0), |(mut tree_count, mut index), i| {
        let str_len = i.len();
        index += x; 

        if i.as_bytes()[index % str_len] == b'#'{
            tree_count += 1;
        }
        (tree_count, index)
    });
    num_of_trees.0
}

pub fn solve_part1(input: &Vec<String>) -> i32 {
    traverse(input, 3,1)
}

pub fn solve_part2(input: &Vec<String>) -> i64 {
    let slopes = vec![(1,1), (3,1), (5,1), (7,1), (1,2)];

    let product = slopes.iter().fold(1, |p, i| {
        let tree_count = traverse(input, i.0, i.1) as i64;
        p * tree_count
    });

    product
}

pub fn solve() {
    let input = parse_input("inputs/input3.txt");
    let part1_solution = solve_part1(&input);
    let part2_solution = solve_part2(&input);


    println!("Part 1: {:?}\n Part 2: {:?}", part1_solution, part2_solution);
}
