use std::io::{self, prelude::*, BufReader};
use std::fs::File;
use itertools::Itertools;

pub fn parse_input(filename: &str) -> Vec<String> {
    let f = File::open(filename).unwrap();
    let reader = BufReader::new(f);

    let mut input: Vec<String> = Vec::new();
    for l in reader.lines(){
        input.push(l.unwrap());
    }
    input
}

pub fn findOneDimension(lowerChar: char, upperChar: char, min: usize, max: usize, commands: &str) -> usize {
    commands.chars().fold((min, max), |(mut currMin, mut currMax), i| {
        let diff = (currMax - currMin)/2 + 1;
        match i {
            k if (k == lowerChar) => { currMax -= diff; },
            k if (k == upperChar) => { currMin += diff; },
            _ => {}
        }
        (currMin, currMax)
    }).0 
}

pub fn calculateSeatIds(input: &Vec<String>) -> Vec<usize> {
    input.iter()
        .map(|x| x.split_at(7)).collect::<Vec<(&str, &str)>>().iter()
        .map(|y| {
        let rowNum = findOneDimension('F', 'B', 0, 127, y.0);
        let colNum = findOneDimension('L', 'R', 0, 7, y.1);
         //   println!("{:?}, {:?}", rowNum, colNum);
        8 * rowNum +  colNum
    }).collect::<Vec<usize>>()
}

pub fn solve_part1(seatIds: &Vec<usize>) -> usize {
    *seatIds.iter().max().unwrap()
}

pub fn find_missing(v: &mut[usize]) -> usize {
    println!{"{:?}", v};
    let length = v.len();
    if (length == 2)
    {
        return v[0] + 1;
    }
    let last = v.last().unwrap();
    let expected_mid_value = (last - v[0])/2 + v[0];
    let mid_index = v.len()/2;
    let mid_value = v[mid_index];
    if mid_value == expected_mid_value {
        find_missing(&mut v[mid_index..length])
    }
    else {
        find_missing(&mut v[0..mid_index+1])
    }
}

pub fn solve_part2(seatIds: &mut Vec<usize>) -> usize {

    seatIds.sort();
    println!{"{:?}", seatIds}; 
    let length = seatIds.len();

    find_missing(&mut seatIds[0..length])
}


pub fn solve() {
    let input = parse_input("inputs/input5.txt");
    let mut seatIds = calculateSeatIds(&input);

    let part1_solution  = solve_part1(&seatIds);
    let part2_solution  = solve_part2(&mut seatIds);

    println!{"Part 1: {:?}, Part 2: {:?}", part1_solution, part2_solution};
}
