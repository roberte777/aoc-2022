fn main() {
    //input.txt contains a number on each line. Read in this file
    //and print the sum of the numbers.
    let mut result: Vec<i32> = include_str!("./input.txt")
        .split("\n\n")
        .map(|x| x.split("\n").flat_map(str::parse::<i32>).sum::<i32>())
        .collect::<Vec<i32>>();
    result.sort();

    let part1 = &result[result.len() - 1];
    let part2: i32 = result[result.len() - 3..].iter().sum();

    println!("Result part 1: {:?}\nResult part 2: {:?}", part1, part2);
}
