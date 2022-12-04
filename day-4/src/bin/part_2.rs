fn main() {
    let input = std::fs::read_to_string("src/input_1.prod").unwrap();
    let assignments = input
        .lines()
        .map(|line| {
            line.split(",")
                .map(|elf| {
                    let ranges = elf
                        .split("-")
                        .map(|e| e.parse::<usize>().unwrap())
                        .collect::<Vec<usize>>();
                    (ranges[0], ranges[1])
                })
                .collect::<Vec<(usize, usize)>>()
        })
        .map(|pairing| {
            let elf_1 = pairing[0];
            let elf_2 = pairing[1];
            println!("{:?} {:?}", elf_1, elf_2);
            if elf_1.0 <= elf_2.0 && elf_1.1 >= elf_2.0 {
                return 1;
            } else if elf_1.0 >= elf_2.0 && elf_1.0 <= elf_2.1 {
                return 1;
            }
            return 0;
        })
        .sum::<usize>();
    println!("{:?}", assignments);
}
