//A = X = Rock
//B = Y = Paper
//C = Z = Sciscors

fn main() {
    let input = include_str!("../input.txt");
    let games: Vec<Vec<Move>> = input
        .lines()
        .map(|x| x.split(" ").collect::<Vec<&str>>())
        .map(|x| str_to_enum(&x[0], &x[1]))
        .collect();
    let scores: Vec<usize> = games.iter().map(|x| score(&x[0], &x[1])).collect();
    let final_score: usize = scores.iter().sum();
    println!("{:?}", final_score)
}

#[derive(Debug)]
enum Move {
    Rock,
    Paper,
    Sciscors,
}
fn str_to_enum(opponent_move: &str, desired_outcome: &str) -> Vec<Move> {
    let mut moves: Vec<Move> = Vec::with_capacity(2);
    match desired_outcome {
        "Z" => {
            match opponent_move {
                "A" => {
                    moves.push(Move::Rock);
                    moves.push(Move::Paper)
                }
                "B" => {
                    moves.push(Move::Paper);
                    moves.push(Move::Sciscors)
                }
                "C" => {
                    moves.push(Move::Sciscors);
                    moves.push(Move::Rock)
                }

                _ => panic!(),
            };
        }
        "Y" => {
            match opponent_move {
                "A" => {
                    moves.push(Move::Rock);
                    moves.push(Move::Rock)
                }
                "B" => {
                    moves.push(Move::Paper);
                    moves.push(Move::Paper)
                }
                "C" => {
                    moves.push(Move::Sciscors);
                    moves.push(Move::Sciscors)
                }

                _ => panic!(),
            };
        }
        "X" => {
            match opponent_move {
                "A" => {
                    moves.push(Move::Rock);
                    moves.push(Move::Sciscors)
                }
                "B" => {
                    moves.push(Move::Paper);
                    moves.push(Move::Rock)
                }
                "C" => {
                    moves.push(Move::Sciscors);
                    moves.push(Move::Paper)
                }

                _ => panic!(),
            };
        }
        "C" => {
            match opponent_move {
                "A" => {
                    moves.push(Move::Rock);
                    moves.push(Move::Sciscors)
                }
                _ => panic!(),
            };
        }
        "Z" => {
            match opponent_move {
                "A" => {
                    moves.push(Move::Rock);
                    moves.push(Move::Sciscors)
                }
                _ => panic!(),
            };
        }

        _ => panic!("Unexpected Input!"),
    };
    return moves;
}

fn score(opponent_move: &Move, player_move: &Move) -> usize {
    match player_move {
        Move::Rock => {
            let mut score = 1;
            score += match opponent_move {
                Move::Rock => 3,
                Move::Paper => 0,
                Move::Sciscors => 6,
            };
            return score;
        }
        Move::Paper => {
            let mut score = 2;
            score += match opponent_move {
                Move::Rock => 6,
                Move::Paper => 3,
                Move::Sciscors => 0,
            };
            return score;
        }

        Move::Sciscors => {
            let mut score = 3;
            score += match opponent_move {
                Move::Rock => 0,
                Move::Paper => 6,
                Move::Sciscors => 3,
            };
            return score;
        }
    }
}
