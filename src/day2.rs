use crate::Result;

pub fn main() -> Result {
    let input = std::fs::read_to_string("data/day2.txt")?;

    let mut valid = 0;
    for line in input.lines() {
        let split = line.split('-').collect::<Vec<&str>>();
        let rest = split[1].split(' ').collect::<Vec<&str>>();
        let min = split[0].parse::<i32>()?;
        let max = rest[0].parse::<i32>()?;
        let letter = rest[1].trim_end_matches(':');
        let password = rest[2];

        let occurences = password.matches(letter).count() as i32;

        if occurences >= min && occurences <= max {
            valid += 1;
        }
    }
    println!("{}", valid);

    Ok(())
}