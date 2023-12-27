use std::{fs, io::BufRead};

struct Grid {
    areas: Vec<Area>
}

struct Area {
    x: usize,
    y: usize,
    content: String
}

fn main() {
    let file = fs::read("./input.txt").unwrap();

    let grid = parse_file_into_grid(file);

    grid.areas.iter().for_each(|area| {
        println!("x: {}, y: {}, content: {}", area.x, area.y, area.content)
    })
}

fn parse_file_into_grid(file: Vec<u8>) -> Grid {
    let lines = file.lines();
    let mut line_string = String::from("");
    let mut areas: Vec<Area> = vec![];

    for (y, line) in lines.enumerate() {
        line_string = line.unwrap();
        let characters = line_string.chars();

        for (x, char) in characters.enumerate() {
            let content = String::from(char);

            areas.push(Area {
                y,
                x, 
                content: content.clone(),
            });
        }
    }

    return Grid {
        areas,
    }
}