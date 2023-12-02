fn main() {
    part_one();
    part_two();
}

fn part_two() {
    let data = std::fs::read_to_string("games")
        .expect("there is no games file");
    let sum: usize = data
        .lines()
        .map(|x| Game::from(x))
        .map(|x| {
            let (r, g, b) = x.min_num_of_cubes();
            return r * g * b;
        })
        .sum();

    println!("the sum of the power of the minimum value of dice is: {}", sum)
}

fn part_one() {
    let data = std::fs::read_to_string("games")
        .expect("there is no games file");
    let max_red_cubes = 12;
    let max_green_cubes = 13;
    let max_blue_cubes = 14;


    let games: Vec<Game> = data
        .lines()
        .map(|x| Game::from(x))
        .collect();

    let ids_of_bad_games: Vec<usize> = games
        .iter()
        .filter(|x| x.was_valid_game(max_red_cubes, max_blue_cubes, max_green_cubes))
        .map(|x| x.id)
        .collect();

    let total: usize = ids_of_bad_games.iter().sum();

    println!("games: {:?}", total)
}

#[derive(Debug)]
struct Game {
    rounds: Vec<Round>,
    id: usize
}

#[derive(Debug)]
struct Round {
    red: usize,
    green: usize,
    blue: usize
}

impl Game {
    fn highest_quantity_red(&self) -> usize {
        return self.rounds.iter().map(|r| r.red).max().unwrap()
    }

    fn highest_quantity_blue(&self) -> usize {
        return self.rounds.iter().map(|r| r.blue).max().unwrap()
    }

    fn highest_quantity_green(&self) -> usize {
        return self.rounds.iter().map(|r| r.green).max().unwrap()
    }

    fn was_valid_game(&self, max_red: usize, max_blue: usize, max_green: usize) -> bool {
        return self.highest_quantity_blue() <= max_blue
            && self.highest_quantity_red() <= max_red
            && self.highest_quantity_green() <= max_green
    }

    fn min_num_of_cubes(&self) -> (usize, usize, usize) {
        return (self.highest_quantity_red(), self.highest_quantity_blue(), self.highest_quantity_green())
    }
}

impl From<&str> for Game {
    fn from(s: &str) -> Self {
        let mut game: Game = Game{ rounds: vec![], id: 0 };
        let idx = s.find(":").unwrap();
        game.id = s.get(5..idx).unwrap().parse().unwrap();
        game.rounds = s.get(idx+1..s.len()).unwrap().split(";").map(|x| Round::from(x)).collect::<Vec<Round>>();

        return game
    }
}

impl From<&str> for Round {
    fn from(value: &str) -> Self {
        let col_quant = value.split(",")
            .map(|x| x.trim())
            .map(|x| {
                let (quantity, col) = x.split_once(" ").unwrap();
                let quantity: usize = quantity.parse().unwrap_or_default();
                return (col, quantity)
            })
            .collect::<Vec<(&str, usize)>>();

        let mut round = Round { blue: 0, red: 0, green: 0 };
        for (col, quantity) in col_quant {
            match col {
                "red" => round.red = quantity,
                "blue" => round.blue = quantity,
                "green" => round.green = quantity,
                "_" => {},
                _ => {}
            }
        }

        return round;
    }
}
