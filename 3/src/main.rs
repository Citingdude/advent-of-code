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

#[derive(Debug, PartialEq)]
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
        let mut temp_number = String::from("");

        for (x, char) in characters.clone().enumerate() {
            let content = String::from(char);
            let char_area_type: AreaType = assign_area_type(content.as_str());
            let mut next_char = 'a';

            match char_area_type {
                AreaType::Number => {
                    if x < characters.clone().count() - 1 {
                        next_char = characters.clone().nth(x + 1).unwrap();
                    }

                    let next_char_area_type: AreaType = assign_area_type(next_char.to_string().as_str());

                    match next_char_area_type {
                        AreaType::Number => {
                            temp_number = temp_number + content.as_str()
                        }

                        _ => {
                            temp_number = temp_number + content.as_str();

                            areas.push(Area {
                                y,
                                x,
                                content: temp_number.clone(),
                                area_type: char_area_type
                            });

                            temp_number = String::from("")
                        }
                    }

                }

                _ => {
                    areas.push(Area {
                        y,
                        x, 
                        content: content.clone(),
                        area_type: char_area_type
                    });
                }
            }

            println!("{}", temp_number);
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