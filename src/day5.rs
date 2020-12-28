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

pub fn find_one_dimension(lower_char: char, upper_char: char, min: usize, max: usize, commands: &str) -> usize {
    commands.chars().fold((min, max), |(mut curr_min, mut curr_max), i| {
        let diff = (curr_max - curr_min)/2 + 1;
        match i {
            k if (k == lower_char) => { curr_max -= diff; },
            k if (k == upper_char) => { curr_min += diff; },
            _ => {}
        }
        (curr_min, curr_max)
    }).0 
}

pub fn calculate_seat_ids(input: &Vec<String>) -> Vec<usize> {
    input.iter()
        .map(|x| x.split_at(7)).collect::<Vec<(&str, &str)>>().iter()
        .map(|y| {
        let row_num = find_one_dimension('F', 'B', 0, 127, y.0);
        let col_num = find_one_dimension('L', 'R', 0, 7, y.1);
         //   println!("{:?}, {:?}", row_num, col_num);
        8 * row_num +  col_num
    }).collect::<Vec<usize>>()
}

pub fn solve_part1(seat_ids: &Vec<usize>) -> usize {
    *seat_ids.iter().max().unwrap()
}

pub fn find_missing(v: &mut[usize]) -> usize {
    let length = v.len();
    if length == 2
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

pub fn solve_part2(seat_ids: &mut Vec<usize>) -> usize {

    seat_ids.sort();
    let length = seat_ids.len();

    find_missing(&mut seat_ids[0..length])
}


pub fn solve() {
    let input = parse_input("inputs/input5.txt");
    let mut seat_ids = calculate_seat_ids(&input);

    let part1_solution  = solve_part1(&seat_ids);
    let part2_solution  = solve_part2(&mut seat_ids);

    println!{"Part 1: {:?}, Part 2: {:?}", part1_solution, part2_solution};
}
