use std::collections::HashSet;
fn main() {
    println!("Hello world");
    let alphabet: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
        .chars()
        .collect();
    let answer: usize = include_str!("../input_1.txt")
        .lines()
        .into_iter()
        .map(|sack| {
            let mut first_half = HashSet::new();
            let byte_sack = sack.as_bytes();
            for i in 0..sack.len() / 2 {
                first_half.insert(byte_sack[i] as char);
            }
            for i in sack.len() / 2..sack.len() {
                if first_half.contains(&(byte_sack[i] as char)) {
                    return char_to_num(&(byte_sack[i] as char), &alphabet);
                }
            }
            return 1;
        })
        .sum();
    println!("{:?}", answer);
}

fn char_to_num(x: &char, alphabet: &Vec<char>) -> usize {
    alphabet.iter().position(|val| val == x).unwrap() + 1
}
