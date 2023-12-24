use std::{fs, path::Path};

fn main() {
    let path = Path::new("./src/input.txt");
    let contents = fs::read_to_string(path)
        .expect("Read file");

    let lines = contents.lines();
    let mut sum: u32 = 0;

    lines.for_each(|line| {
        let first_number_index = line.find(|c: char| c.is_digit(10)).unwrap();
        let last_number_index = line.rfind(|c: char| c.is_digit(10)).unwrap();

        let first_number = line
            .chars()
            .nth(first_number_index)
            .unwrap();
        let last_number = line
            .chars()
            .nth(last_number_index)
            .unwrap();

        let mut total = String::from("");

        total.push(first_number);
        total.push(last_number);

        let total_number = total.parse::<u32>().unwrap();

        sum = sum + total_number;
    });

    println!("{}", sum);
}
