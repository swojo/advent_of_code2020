use std::fs;
use itertools::Itertools;
use std::collections::HashSet;

pub fn solve_part1(input: &str, expected_keys: &HashSet<&str>) -> i32 {

    let validPassports = input.split("\n\n").collect::<Vec::<&str>>().iter().fold(0, |mut acc, i| {
        let currPass = i.split_whitespace().collect::<Vec::<&str>>().iter().fold(HashSet::new(), |mut hash, field| {
            let (mut key, _) = field.split(":").collect_tuple().unwrap();
            hash.insert(key);
            hash
        });
        let diff : HashSet<_> = expected_keys.difference(&currPass).collect();
        if diff.is_empty() {
            acc += 1;
        }

        acc
    });
    validPassports
}

pub fn is_valid (key: &str, value: &str) -> bool {
    let eye_colors: HashSet<_> = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].iter().cloned().collect();
    match key {
        "byr" => { 
            let year: i32= value.parse().unwrap();
            year >= 1920 && year <= 2002
        },
        "iyr" => {
            let year: i32= value.parse().unwrap();
            year >= 2010 && year <= 2020 
        },
        "eyr" => {
            let year: i32= value.parse().unwrap();
            year >= 2020 && year <= 2030 
        },
        "hgt" => {
            let unitIndex = value.find("in");
            let (height, units) = 
                match unitIndex {
                    Some(x) => {
                        value.split_at(x)
                    }
                    None => {
                        match value.find("cm") {
                            Some(x) => {
                                value.split_at(x)
                            }
                            None => ("0", "cm")
                        }
                    }
                };
            let height_d :i32 = height.parse().unwrap();
            if units == "cm" {
                height_d >= 150 && height_d <= 193
            }
            else {
                height_d >= 59 && height_d <= 76
            }
        },
        "ecl" => {
            eye_colors.contains(value)
        },
        "pid" => {
            let pid = value.parse::<u32>();
            if !pid.is_err() && value.len() == 9 {
                true
            }
            else {
                false
            }
        },
        "hcl" => {
            if value.as_bytes()[0] == b'#'
            {
                let (_, hex) = value.split_at(1);
                let color = i64::from_str_radix(hex, 16);
                if !color.is_err() {
                    return true;
                }
            }
            false
        },
        _ => false
    }
}

pub fn solve_part2(input: &str, expected_keys: &HashSet<&str>) -> i32 {

    let validPassports = input.split("\n\n").collect::<Vec::<&str>>().iter().fold(0, |mut acc, i| {
        let currPass = i.split_whitespace().collect::<Vec::<&str>>().iter().fold(HashSet::new(), |mut hash, field| {
            let (mut key, value) = field.split(":").collect_tuple().unwrap();
            if is_valid(key, value) {
                hash.insert(key);
            }
            hash
        });
        let diff : HashSet<_> = expected_keys.difference(&currPass).collect();
        if diff.is_empty() {
            acc += 1;
        }

        acc
    });
    validPassports
}

pub fn solve() {
    let input : String = fs::read_to_string("inputs/input4.txt").unwrap();
    let expected_keys : HashSet<_> = ["hgt", "iyr", "byr", "eyr", "pid", "ecl", "hcl"].iter().cloned().collect();

    println!("Part 1: {:?}\n Part 2: {:?}", solve_part1(&input, &expected_keys), solve_part2(&input, &expected_keys));
    
}
