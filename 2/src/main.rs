use std::{fs, io::BufRead, fmt};

struct Game {
    id: u32,
    sets: Vec<Cube>
}

struct Cube {
    color: CubeColor,
    amount: u32
}

impl PartialEq for Cube {
    fn eq(&self, other: &Self) -> bool {
        self.amount == other.amount
    }
}

impl PartialOrd for Cube {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.amount.cmp(&other.amount))
    }
}

impl Ord for Cube {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.amount.cmp(&other.amount)
    }
}

impl Eq for Cube {}

#[derive(PartialEq)]
enum CubeColor {
    RED,
    GREEN,
    BLUE
}

impl fmt::Display for CubeColor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       match self {
            CubeColor::BLUE => write!(f, "Blue"),
            CubeColor::RED => write!(f, "Red"),
            CubeColor::GREEN => write!(f, "Green"),
       }
    }
}

const MAX_RED_CUBES: u32 = 12;
const MAX_GREEN_CUBES: u32 = 13;
const MAX_BLUE_CUBES: u32 = 14;

fn main() {
    let file = fs::read("./src/input.txt").unwrap();
    let games = parse_file(file);
    let mut valid_games: Vec<u32> = vec![];
    let mut total_power: u32 = 0;

    games.iter().for_each(|game: &Game| {
        if validate_game(game) {
            valid_games.push(game.id)
        }

        total_power = total_power + get_minimum_amount_cube_set(game);
    });

    let valid_games_total: u32 = valid_games.iter().sum();

    println!("{}", valid_games_total);
    println!("{}", total_power);
}

fn parse_file(file: Vec<u8>) -> Vec<Game> {
    let lines = file.lines();
    let mut games: Vec<Game> = vec![];

    lines.for_each(|line| {
        let result = line.unwrap();
        let parts: Vec<_> = result.split(':').collect();
        let game_meta = parts[0].trim();
        let game_meta_parts: Vec<_> = game_meta.split(' ').collect();
        let game_id: u32 = game_meta_parts[1].parse().unwrap();

        let game_result = parts[1].trim();
        let game_set_lines: Vec<_> = game_result.split(";").collect();
        let mut cubes: Vec<Cube> = vec![];

        for game_set_line in game_set_lines {
            let cube_lines: Vec<_> = game_set_line.split(',').collect();
            let mut set_cubes: Vec<Cube> = vec![];

            for cube_line in cube_lines  {
                let cube_line_parts: Vec<_> = cube_line.trim().split(' ').collect();
                let cube_line_amount = cube_line_parts[0];
                let cube_line_color = cube_line_parts[1];

                let cube = Cube {
                    amount: cube_line_amount.parse::<u32>().unwrap(),
                    color: get_cube_color(cube_line_color)
                };

                set_cubes.push(cube);
                            }

            for cube in set_cubes {
                cubes.push(cube);
            }

        }
    
        let game = Game {
            id: game_id,
            sets: cubes
        };

        games.push(game)
    });

    return games
}

fn validate_game(game: &Game) -> bool {
    let mut is_valid = true;

    for cube in &game.sets {
        match cube.color {
            CubeColor::RED => {
                let valid = validate_cube(cube, MAX_RED_CUBES);
                if valid == false {
                    is_valid = false
                } 
            },
            CubeColor::BLUE => {
                let valid = validate_cube(cube, MAX_BLUE_CUBES);
                if valid == false {
                    is_valid = false
                }
            },
            CubeColor::GREEN => {
                let valid = validate_cube(cube, MAX_GREEN_CUBES);
                if valid == false {
                    is_valid = false
                }
            }
        }
    }
    
    is_valid
}

fn validate_cube(cube: &Cube, max: u32) -> bool {
    if cube.amount > max {
        return false
    } else {
        return true
    }
}

fn get_cube_color(color: &str) -> CubeColor {
    match color {
        "red" => CubeColor::RED,
        "blue" => CubeColor::BLUE,
        "green" => CubeColor::GREEN,
        &_ => CubeColor::BLUE,
    }
}

// Retrieve a set of cubes with the minium amount needed for the game
fn get_minimum_amount_cube_set(game: &Game) -> u32 {
    let red_cubes = get_cubes_by_color(&game.sets, CubeColor::RED);
    let blue_cubes = get_cubes_by_color(&game.sets, CubeColor::BLUE);
    let green_cubes = get_cubes_by_color(&game.sets, CubeColor::GREEN);

    let max_red = red_cubes.iter().max().unwrap();
    let max_blue = blue_cubes.iter().max().unwrap();
    let max_green = green_cubes.iter().max().unwrap();

    let power = max_red.amount * max_blue.amount * max_green.amount;

    return power
}

fn get_cubes_by_color(cubes: &Vec<Cube>, color: CubeColor) -> Vec<&Cube> {
    let mut filtered_cubes: Vec<&Cube> = vec![];

    for cube in cubes {
        if cube.color == color {
            filtered_cubes.push(cube)
        }
    }

    return filtered_cubes
}