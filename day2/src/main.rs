extern crate itertools;

use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

use itertools::Itertools;


fn main() {
    let f = File::open("input.txt").unwrap();
    let f2 = File::open("input.txt").unwrap();

    let buff = BufReader::new(f);
    let buff2 = BufReader::new(f2);
    let checksum : i32 = buff.lines().map(|x|difference(x.unwrap())).sum();

    println!("Checksum: {}", checksum);

    let sum_of_divisibles : i32 = buff2.lines().map(|x|evenly_divisible(x.unwrap())).sum();

    println!("Sum of divisibles: {}", sum_of_divisibles);

}

fn difference(nums_strings: String) -> i32{
    let mut nums : Vec<i32> = nums_strings.split_whitespace()
                .map(|x|x.parse().unwrap()).collect();

    nums.sort();
    nums.last().unwrap() - nums.first().unwrap()
}

fn evenly_divisible(nums_strings:String) -> i32{
    let nums : Vec<i32> = nums_strings.split_whitespace()
        .map(|x|x.parse().unwrap()).collect();

    nums.iter().tuple_combinations()
            .filter(|&(x,y)| x%y==0 || y%x==0) //assume only 1 value from this list
            .map(|(x,y)| if x<y {(y,x)} else {(x,y)})
            .map(|(x,y)| *x / *y)
            .sum()
}
