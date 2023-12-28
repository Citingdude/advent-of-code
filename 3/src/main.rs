use std::{fs, io::BufRead};

struct Grid {
    areas: Vec<Area>
}

struct Area {
    x: usize,
    y: usize,
    content: String,
    area_type: AreaType
}

#[derive(Debug)]
enum AreaType {
    Number,
    Symbol,
    Period
}

fn main() {
    let file = fs::read("./input.txt").unwrap();

    let grid = parse_file_into_grid(file);

    grid.areas.iter().for_each(|area| {
        println!("x: {}, y: {}, content: {}, type: {:?}", area.x, area.y, area.content, area.area_type)
    })
}

fn parse_file_into_grid(file: Vec<u8>) -> Grid {
    let lines = file.lines();
    let mut areas: Vec<Area> = vec![];

    for (y, line) in lines.enumerate() {
        let line_string = line.unwrap();
        let characters = line_string.chars();

        for (x, char) in characters.clone().enumerate() {
            let content = String::from(char);
            let char_area_type = assign_area_type(content.as_str());

            areas.push(Area {
                y,
                x, 
                content: content.clone(),
                area_type: char_area_type
            });
        }
    }

    return Grid {
        areas,
    }
}

fn assign_area_type(content: &str) -> AreaType {
    let area_type: AreaType;

    match content {
        "." => area_type = AreaType::Period,
        "*" => area_type = AreaType::Symbol,
        "@" => area_type = AreaType::Symbol,
        "+" => area_type = AreaType::Symbol,
        "/" => area_type = AreaType::Symbol,
        "=" => area_type = AreaType::Symbol,
        "$" => area_type = AreaType::Symbol,
        "#" => area_type = AreaType::Symbol,
        _ => area_type = AreaType::Number
    }

    return area_type
}