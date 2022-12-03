use std::collections::HashSet;
fn main() {
    let alphabet: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
        .chars()
        .collect();
    let binding = include_str!("../input_2.txt")
        .lines()
        .collect::<Vec<&str>>();
    let groups = binding.chunks(3).collect::<Vec<&[&str]>>();
    let mut nums: Vec<usize> = Vec::new();
    for group in &groups {
        let mut sets: Vec<HashSet<char>> = Vec::with_capacity(3);
        for sack in group.iter() {
            let bytes_sack = sack.as_bytes();
            let mut set = HashSet::new();
            for (i, _) in bytes_sack.iter().enumerate() {
                set.insert(bytes_sack[i] as char);
            }
            sets.push(set);
        }
        for char in sets[0].iter() {
            if sets[1].contains(char) && sets[2].contains(char) {
                nums.push(char_to_num(&char, &alphabet));
            }
        }
    }
    let answer = nums.iter().sum::<usize>();

    println!("{:?}", answer);
}

fn char_to_num(x: &char, alphabet: &Vec<char>) -> usize {
    alphabet.iter().position(|val| val == x).unwrap() + 1
}
