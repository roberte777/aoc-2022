use std::{fmt::Error, str::FromStr};

fn main() {
    let input = std::fs::read_to_string("src/input_1.prod")
        .unwrap()
        .parse::<Forest>()
        .unwrap();
    let count = input.best_scenic_score();
    println!("{:?}", count);
}

#[derive(Debug)]
struct Forest {
    trees: Vec<Vec<Tree>>,
}

impl Forest {
    // pub fn num_visible(&self) -> usize {
    //     todo!()
    // }
    pub fn best_scenic_score(&self) -> usize {
        let length = self.trees[1].len();
        let height = self.trees.len();
        let mut scores: Vec<Vec<usize>> = vec![vec![0; length]; height];

        for y in 0..height {
            for x in 0..length {
                if x == 0 || x == length - 1 || y == 0 || y == height - 1 {
                    continue;
                }
                let starting_tree_height = self.trees[y][x].height;
                let (mut score_left, mut score_right, mut score_up, mut score_down) = (0, 0, 0, 0);
                //right
                // let mut next_tree_height = self.trees[y][x + 1].height;
                // if next_tree_height < starting_tree_height {
                for temp_x in (x + 1)..length {
                    if self.trees[y][temp_x].height < starting_tree_height {
                        score_right += 1;
                    } else {
                        score_right += 1;
                        break;
                    }
                }
                // }
                //left
                // next_tree_height = self.trees[y][x - 1].height;
                // if next_tree_height < starting_tree_height {
                for temp_x in (0..x).rev() {
                    if self.trees[y][temp_x].height < starting_tree_height {
                        score_left += 1;
                    } else {
                        score_left += 1;
                        break;
                    }
                }
                // }
                //down
                // next_tree_height = self.trees[y + 1][x].height;
                // if next_tree_height < starting_tree_height {
                for temp_y in y + 1..height {
                    if self.trees[temp_y][x].height < starting_tree_height {
                        score_down += 1;
                    } else {
                        score_down += 1;
                        break;
                    }
                }
                // }
                //up
                // next_tree_height = self.trees[y - 1][x].height;
                // if next_tree_height < starting_tree_height {
                for temp_y in (0..y).rev() {
                    if self.trees[temp_y][x].height < starting_tree_height {
                        score_up += 1;
                    } else {
                        score_up += 1;
                        break;
                    }
                }
                // }
                println!(
                    "x,y: {} {}\tscores: {} {} {} {}",
                    x, y, score_left, score_right, score_up, score_down
                );
                scores[y][x] = score_left * score_right * score_up * score_down;
            }
        }
        println!("{:?}", scores);
        return scores
            .into_iter()
            .map(|row| row.into_iter().max().unwrap())
            .max()
            .unwrap();
    }
}

impl FromStr for Forest {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            trees: s
                .lines()
                .map(|row| row.chars().map(|tree| Tree::from(tree)).collect::<Vec<_>>())
                .collect::<Vec<Vec<_>>>(),
        })
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
pub struct Tree {
    height: usize,
}
impl FromStr for Tree {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        println!("test {}", s);
        Ok(Self {
            height: s.parse::<usize>().unwrap(),
        })
    }
}
impl From<char> for Tree {
    fn from(c: char) -> Self {
        Self {
            height: c.to_digit(10).unwrap() as usize,
        }
    }
}
