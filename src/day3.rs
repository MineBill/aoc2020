use crate::Result;

fn find_trees(input: &String, x_step: usize, y_step: usize) -> i64 {
    let mut x = 0usize;
    let mut trees = 0i64;
    for line in input.lines().step_by(y_step) {
        let chars = line.chars().collect::<Vec<char>>();

        let loc = chars.get(x).unwrap();
        if *loc == '#' {
            trees += 1;
        }

        let width = line.len();
        x += x_step;
        x = x % width;
    }
    trees
}

pub fn main() -> Result {
    let input = std::fs::read_to_string("data/day3.txt")?;

    let first = find_trees(&input, 1, 1);
    let second = find_trees(&input, 3, 1);
    let third = find_trees(&input, 5, 1);
    let fourth = find_trees(&input, 7, 1);
    let fifth = find_trees(&input, 1, 2);

    println!("{}", first * second * third * fourth * fifth);

    Ok(())
}