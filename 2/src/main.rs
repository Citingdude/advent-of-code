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
    let game_struct = parse_game();

    const MAX_RED_CUBES: u32 = 12;
    const MAX_GREEN_CUBES: u32 = 13;
    const MAX_BLUE_CUBES: u32 = 14;

    let file = fs::read("./src/input.txt").unwrap();
    let lines = file.lines();

    lines.for_each(|line| {
        let result = line.unwrap();
        let mut game_is_possible = true;

        let parts: Vec<_> = result.split(':').into_iter().collect();
        let game_title = parts[0];

        println!("{}", game_title);

        let game = parts[1];
        let game_sets: Vec<_> = game.split(";").collect();

        for set in game_sets {
            let cube_colors: Vec<_> = set.split(',').collect();
            
            for cube_color in cube_colors {
                let cube_color_parts: Vec<_> = cube_color.split(' ').collect();
                let amount: u32 = cube_color_parts[1].parse().unwrap();
                let color = cube_color_parts[2];

                if amount > MAX_BLUE_CUBES {
                    game_is_possible = false
                }

                // println!("{}", color)
            }
            // println!("{}", set)
        }

        println!("{}", game_is_possible);
    });
}

fn parse_game() -> Game {
    let game = Game {
        id: 0,
        sets: vec![Cube {
            color: CubeColor::RED,
            amount: 10
        }, 
        Cube {
            color: CubeColor::BLUE,
            amount: 5
        }]
    };

    game
}