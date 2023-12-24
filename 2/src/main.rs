use std::{fs, io::BufRead};

struct Game {
    id: u32,
    sets: Vec<Cube>
}

struct Cube {
    color: CubeColor,
    amount: u32
}

enum CubeColor {
    RED,
    GREEN,
    BLUE
}

fn main() {
    let file = fs::read("./src/input.txt").unwrap();
    let games = parse_file(file);

    games.iter().for_each(|game| {
        println!("{}", game.id)
    });

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
                    color: CubeColor::RED
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