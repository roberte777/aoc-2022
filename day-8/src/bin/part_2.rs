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
    pub fn num_visible(&self) -> usize {
        let length = self.trees[1].len();
        let height = self.trees.len();
        let mut vis_count = 0;

        for y in 0..height {
            for x in 0..length {
                if x == 0 || x == length - 1 || y == 0 || y == height - 1 {
                    vis_count += 1;
                    continue;
                }
                let curr_tree_height = &self.trees[y][x].height;
                let (mut vis_left, mut vis_right, mut vis_up, mut vis_down) =
                    (true, true, true, true);
                for temp_x in (x + 1)..length {
                    if &self.trees[y][temp_x].height >= curr_tree_height {
                        vis_right = false;
                        break;
                    }
                }
                for temp_x in (0..x).rev() {
                    if &self.trees[y][temp_x].height >= curr_tree_height {
                        vis_left = false;
                        break;
                    }
                }

                for temp_y in (y + 1)..height {
                    if &self.trees[temp_y][x].height >= curr_tree_height {
                        vis_down = false;
                        break;
                    }
                }
                for temp_y in (0..y).rev() {
                    if &self.trees[temp_y][x].height >= curr_tree_height {
                        vis_up = false;
                        break;
                    }
                }
                if vis_up || vis_down || vis_left || vis_right {
                    vis_count += 1
                }
            }
        }
        return vis_count;
    }
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
                for temp_x in (x + 1)..length {
                    if self.trees[y][temp_x].height < starting_tree_height {
                        score_right += 1;
                    } else {
                        score_right += 1;
                        break;
                    }
                }
                //left
                for temp_x in (0..x).rev() {
                    if self.trees[y][temp_x].height < starting_tree_height {
                        score_left += 1;
                    } else {
                        score_left += 1;
                        break;
                    }
                }
                //down
                for temp_y in y + 1..height {
                    if self.trees[temp_y][x].height < starting_tree_height {
                        score_down += 1;
                    } else {
                        score_down += 1;
                        break;
                    }
                }
                //up
                for temp_y in (0..y).rev() {
                    if self.trees[temp_y][x].height < starting_tree_height {
                        score_up += 1;
                    } else {
                        score_up += 1;
                        break;
                    }
                }
                scores[y][x] = score_left * score_right * score_up * score_down;
            }
        }
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
