//A = X = Rock
//B = Y = Paper
//C = Z = Sciscors

fn main() {
    let input = include_str!("../input.txt");
    let games: Vec<Vec<Move>> = input
        .lines()
        .map(|x| x.split(" ").map(|x| str_to_enum(x)).collect())
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
fn str_to_enum(input: &str) -> Move {
    match input {
        "A" => Move::Rock,
        "X" => Move::Rock,
        "B" => Move::Paper,
        "Y" => Move::Paper,
        "C" => Move::Sciscors,
        "Z" => Move::Sciscors,

        _ => panic!("Unexpected Input!"),
    }
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
