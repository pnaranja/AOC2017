use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn main() {
    let f = File::open("input.txt").unwrap();

    let buff = BufReader::new(f);
    let checksum : i32 = buff.lines().map(|x|difference(x.unwrap())).sum();

    println!("Checksum: {}", checksum);


}

fn difference(nums_strings: String) -> i32{
    let mut nums : Vec<i32> = nums_strings.split_whitespace()
                .map(|x|x.parse().unwrap()).collect();

    nums.sort();
    nums.last().unwrap() - nums.first().unwrap()
}

