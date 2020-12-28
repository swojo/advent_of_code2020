use crate::tools;

pub fn solve_part1(input_vec : &Vec<(&str, i32)>) -> i32
{
    let mut index: usize= 0;
    let mut curr_tuple;
    let mut acc: i32 = 0;

    let mut visited = vec![false; input_vec.len()];

    while visited[index] != true || index == input_vec.len()
    { 
        curr_tuple = input_vec[index];
        visited[index] = true;
        match curr_tuple.0 {
            "nop" => { index += 1; },
            "acc" => { 
                acc += curr_tuple.1;
                index += 1;
            },
            "jmp" => { 
                let tmp = index as i32;
                index = (tmp + curr_tuple.1) as usize;
            },
            _ => {}
        }
    }
    acc
}


pub fn solve() {
    let input = tools::parser::parse_input("inputs/input8.txt");
    let input_tuples : Vec<(&str, i32)> = input.iter()
        .map(|x| {
            let mut splitx = x.split_whitespace();
            (splitx.next().unwrap(), 
             splitx.next().unwrap().parse::<i32>().unwrap()
            )
        })
        .collect();
    let part1_solution = solve_part1(&input_tuples);
    println!{"{:?}", part1_solution};
}
