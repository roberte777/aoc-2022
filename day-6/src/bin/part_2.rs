fn main() {
    let input = std::fs::read_to_string("src/input_1.prod").unwrap();
    let mut index = None;
    for (i, c) in input.chars().enumerate() {
        let mut seq = true;
        let mut set = std::collections::HashSet::new();
        set.insert(c);
        for j in 1..14 {
            if set.contains(&input.chars().nth(i + j).expect("out of bounds")) {
                println!("{} == {}", input.chars().nth(i + j).unwrap(), c);
                seq = false;
                break;
            }
            set.insert(input.chars().nth(i + j).expect("out of bounds"));
        }
        if seq {
            index = Some(i + 14);
            break;
        }
    }
    println!("{:?}", index);
}
