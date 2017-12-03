// 17  16  15  14  13
//18   5   4   3  12
//19   6   1   2  11
//20   7   8   9  10
//21  22  23---> ...

//For example:
//Data from square 1 is carried 0 steps, since it's at the access port.
//Data from square 12 is carried 3 steps, such as: down, left, left.
//Data from square 23 is carried only 2 steps: up twice.
//Data from square 1024 must be carried 31 steps.

//Puzzle input is 361527

//Just analyzing the pattern 1,2,11,...
//Distance between each number is 1+(8*n) where n starts at 0
fn get_distance(n:i32) -> i32{ 1+(8*n) }

//Need to get size of "ring" of numbers to get to a certain number
//Where the ring starts to the right, starting from 1
fn minimum_size_of_ring_nums(max_num : i32) -> (i32,i32){

    fn calc_max(max_num: i32, current_num: i32, count: i32) -> (i32,i32){
        if current_num >= max_num {(count,current_num)}
        else{
            calc_max(max_num, current_num+get_distance(count), count+1)
        }
    }

    calc_max(max_num, 0, 0)
}

fn main() {
    let (min_rings,current_num) = minimum_size_of_ring_nums(361527);
    println!("min size of ring for 11: {} and current num is {}", min_rings, current_num);
    println!("min size of ring for 11: {:?}", minimum_size_of_ring_nums(10));
}
