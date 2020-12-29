use crate::tools;

pub fn run_boot(input_vec : &Vec<(&str, i32)>) -> Result<i32, i32> 
{
    let mut index: usize= 0;
    let mut curr_tuple;
    let mut acc: i32 = 0;

    let mut visited = vec![false; input_vec.len()];

    while (index < input_vec.len()) && (visited[index] != true)
    { 
        curr_tuple = input_vec[index];
        visited[index] = true;
        match &*curr_tuple.0 {
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
    if index == input_vec.len() {
        Ok(acc)
    }
    else {
        Err(acc)
    }
}

pub fn solve_part1(input_vec : &mut Vec<(&str, i32)>) -> i32
{
    match run_boot(input_vec) {
        Ok(_) => { 0 },
        Err(acc) => { acc }
    }
}

pub fn solve_part2(input_vec : &mut Vec<(&str, i32)>) -> i32
{
    for op in input_vec.iter().enumerate() {
        let mut tmp_vec = input_vec.clone();
        
        match &*op.1.0 {
           "nop" => { tmp_vec[op.0].0 = "jmp"; },
           "jmp" => { tmp_vec[op.0].0 = "nop"; },
           _ => {continue; }
        }

        if let Ok(acc) = run_boot(&tmp_vec) {
            // boot program terminated
            return acc;
        }
    }
    0
}


pub fn solve() {
    let input = tools::parser::parse_input("inputs/input8.txt");
    let mut input_tuples : Vec<(&str, i32)> = input.iter()
        .map(|x| {
            let mut splitx = x.split_whitespace();
            (splitx.next().unwrap(), 
             splitx.next().unwrap().parse::<i32>().unwrap()
            )
        })
        .collect();
    let part1_solution = solve_part1(&mut input_tuples);
    let part2_solution = solve_part2(&mut input_tuples);
    println!{"{:?}, {:?}", part1_solution, part2_solution};
}
