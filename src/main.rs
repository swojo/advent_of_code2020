use std::io::{self, prelude::*, BufReader};
use std::fs::File;

fn main() -> io::Result<()> {
    let f = File::open("inputs/input1.txt")?;
    let reader = BufReader::new(f);

    let mut nums: Vec<i32> = Vec::new();
    for l in reader.lines(){
        nums.push(l?.parse().unwrap());
    }
    println!("{:?}", nums);
    
    nums.sort();

    let mut reversed = nums.clone();
    reversed.reverse();

    'outer: for num_reversed in reversed.iter()
    {
        for num in nums.iter()
        {
            let sum = num + num_reversed;
            println!("{:?}", sum);
            if (sum == 2020)
            {
                println!("found {:?}", (num * num_reversed));
                break 'outer;
            }
            else if sum > 2020
            {
                break;
            }
        }
    }

    Ok(())


//    let differences: Vec<i32> = nums.clone().into_iter().map(|x| 2020 - x).collect();
//
//    for num in differences.iter()
//    {
//
//    
//
//    println!("{:?}", nums);
}
