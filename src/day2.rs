use crate::Result;

pub fn main() -> Result {
    let input = std::fs::read_to_string("data/day2.txt")?;

    let mut valid = 0;
    for line in input.lines() {
        let split = line.split('-').collect::<Vec<&str>>();
        let rest = split[1].split(' ').collect::<Vec<&str>>();
        let first = split[0].parse::<usize>()?;
        let second = rest[0].parse::<usize>()?;
        let letter = rest[1].trim_end_matches(':').parse()?;
        let password = rest[2].chars().collect::<Vec<char>>();

        if password[first - 1] == letter {
            if password[second - 1] != letter {
                valid += 1;
            }
        }
        else if password[second - 1] == letter {
            valid += 1;
        }
    }
    println!("{}", valid);

    Ok(())
}