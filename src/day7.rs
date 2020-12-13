use std::io::{self, prelude::*, BufReader};
use std::fs::File;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::hash_map::Entry;
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

pub fn trim_description(original: &str) -> Option<(i32, String)> {
    let tmp = original.split(" ").collect::<Vec::<&str>>();
    if let Some((_, mut tmp)) = tmp.split_last() {
        
        match  tmp[1].parse::<i32>() {
            Ok(x) => {
                return Some((x, tmp[2..].iter().join(" ")));
            }
            Err(_) => { return None; }
        }
    }
    None
}

pub fn trim_outer(original: &str) -> Option<String> {
    let tmp = original.split("bag").collect::<Vec::<&str>>();
    if let Some((_, mut tmp)) = tmp.split_last() {
        return Some(tmp.iter().join(" ").trim().to_string());
    }
    None
}
    
pub fn get_map_part1(input: &Vec<String>) -> HashMap<String, Vec<String>> {
    let mut map : HashMap<String, Vec<String>> = HashMap::new();
    for line in input.iter() {
        let (mut outer_bag, mut inner_bag) = line.split("contain").collect_tuple().unwrap();
        let outer_bag= trim_outer(outer_bag).unwrap();
     
        let values  = inner_bag.split(",").collect::<Vec::<&str>>();
        for v in values
        {
           match trim_description(v) {
               Some((_, i)) => {

                   match map.entry(i) {
                       Entry::Vacant(e) => { e.insert(vec![outer_bag.clone()]); },
                       Entry::Occupied(mut e) => { e.get_mut().push(outer_bag.clone()); }
                   } 
               }
               None => {}
           }
        }
    }
    map
}

pub fn get_map_part2(input: &Vec<String>) -> HashMap<String, Vec<(String, i32)>> {
    let mut map : HashMap<String, Vec<(String, i32)>> = HashMap::new();
    for line in input.iter() {
        let (mut outer_bag, mut inner_bag) = line.split("contain").collect_tuple().unwrap();
        let outer_bag= trim_outer(outer_bag).unwrap();
     
        let values  = inner_bag.split(",").collect::<Vec::<&str>>();
        for v in values
        {
            match trim_description(v)
            {
               Some((num_of, bag_color)) => {
                   match map.entry(outer_bag.clone()) {
                       Entry::Vacant(e) => { e.insert(vec![(bag_color.clone(), num_of)]); },
                       Entry::Occupied(mut e) => { e.get_mut().push((bag_color.clone(), num_of)); }
                   } 
               },
               None => {}
            }
        }
    }
    map
}

pub fn solve_part1(map: &HashMap<String, Vec<String>>, curr_bag: &str, ret: &mut HashSet<String>) {
    if !map.contains_key(curr_bag) { return; }
    for b in &map[curr_bag]{
        ret.insert(b.to_string());
        solve_part1(map, b, ret);
    }
}

pub fn solve_part2(map: &HashMap<String, Vec<(String, i32)>>, curr_bag: &str) -> i32 {
    if !map.contains_key(curr_bag) { return 1; }

    let mut total = 0;
    for b in &map[curr_bag]{
        let color = &b.0;
        total +=   solve_part2(map, &color) * b.1 ;
    }
    return total+1;
}

pub fn solve() {
    let input = parse_input("inputs/input7.txt");
    let mut result_hash : HashSet<String> = HashSet::new();
    solve_part1(&get_map_part1(&input), "shiny gold", &mut result_hash);
    let part1_solution = result_hash.len();

    // Subtract -1 because shiny gold is also counted
    let mut part2_solution = solve_part2(&get_map_part2(&input), "shiny gold") -1;

    println!("Part 1: {:?}\n Part 2: {:?}", part1_solution, part2_solution);
}
