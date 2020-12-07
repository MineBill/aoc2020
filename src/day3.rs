use crate::Result;

pub fn main() -> Result {
    let input = std::fs::read_to_string("data/day3.txt")?;
    let mut trees = 0;
    let mut x = 0;

    for line in input.lines().skip(1) {
        let chars = line.chars().collect::<Vec<char>>();
        let width = line.len();
        x += 3;
        x = x % width;

        let loc = chars.get(x).unwrap();
        if *loc == '#' {
            trees += 1;
        }
    }

    println!("{}", trees);

    Ok(())
}