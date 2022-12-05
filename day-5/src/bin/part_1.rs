use std::{fmt::Display, str::FromStr};

#[derive(Debug)]
struct ParseShipError;
#[derive(Debug)]
struct ParseStackError;
#[derive(Debug)]
struct ParseCrateError;
#[derive(Debug)]
struct Ship {
    stacks: Vec<Stack>,
}
impl FromStr for Ship {
    type Err = ParseShipError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut temp: Vec<String> = s.split("\n").map(|x| x.to_string()).collect();
        let num_columns = temp
            .pop()
            .expect("There should be stacks")
            .chars()
            .last()
            .expect("should be a last column")
            .to_digit(10)
            .expect("Last column number should be a number");

        for line in temp.iter_mut() {
            while line.len() < (num_columns * 9) as usize {
                line.push(' ');
            }
        }

        let mut stacks: Vec<Stack> = Vec::new();
        for i in 0..num_columns {
            let mut stack: Vec<Crate> = Vec::new();
            for line in temp.iter() {
                let start = i * 4;
                let end = start + 3;
                let crate_str = &line[start as usize..end as usize];
                let crate_ = crate_str.parse::<Crate>().unwrap();
                stack.insert(0, crate_);
            }
            stacks.push(Stack { crates: stack });
        }
        for stack in stacks.iter_mut() {
            stack.crates.retain(|x| x.id != ' ');
        }
        return Ok(Ship { stacks });
    }
}
impl Display for Ship {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, stack) in self.stacks.iter().enumerate() {
            write!(f, "stack num {}: {} ", i + 1, stack)?;
        }
        Ok(())
    }
}
#[derive(Debug, Clone)]
struct Stack {
    crates: Vec<Crate>,
}
impl Display for Stack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for crate_ in self.crates.iter() {
            write!(f, "{}", crate_)?;
        }
        Ok(())
    }
}
#[derive(Debug, Clone)]
struct Crate {
    id: char,
}
impl Display for Crate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id)
    }
}
impl FromStr for Crate {
    type Err = ParseStackError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Crate {
            id: s.chars().nth(1).unwrap(),
        })
    }
}

fn main() {
    let input = std::fs::read_to_string("src/input_1.prod").unwrap();
    let (ship, instructions) = input.split_once("\n\n").expect("aoc is not a liar");
    let mut ship_obj = ship.parse::<Ship>().expect("should parse ship");

    let instructions = instructions.lines().map(|x| {
        x.split(" ")
            .filter(|s| {
                !s.chars()
                    .filter(|c| c.is_numeric())
                    .collect::<String>()
                    .is_empty()
            })
            .map(|s| s.parse::<usize>().expect("should be only nums left"))
            .collect::<Vec<usize>>()
    });

    for instruction in instructions {
        println!("{}\n\n", ship_obj);
        let (num, from, to) = (instruction[0], instruction[1], instruction[2]);
        let split_nmbr = ship_obj.stacks[from - 1].crates.len() - num;
        let mut crates = ship_obj.stacks[from - 1].crates.split_off(split_nmbr);
        ship_obj.stacks[to - 1].crates.append(&mut crates);
    }

    // print the last crate in each stack
    for stack in &ship_obj.stacks {
        println!("{:?}", stack.crates.last().unwrap().id);
    }
}
