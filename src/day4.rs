use crate::Result;

pub fn main() -> Result {
    let mut input = std::fs::read_to_string("data/day4.txt")?;
    // This is a hack and should be removed
    input.push_str("\n");

    let mut valid = 0;
    let mut sum = 0;

    for line in input.lines() {
        if line.is_empty() {
            if sum == 7 {
                valid += 1;
            }
            sum = 0;
            continue;
        }
        let entries = line.split(' ').collect::<Vec<&str>>();
        for entry in entries {
            let split = entry.split(':').collect::<Vec<&str>>();
            let field = split[0];

            match field {
                "byr" | "iyr" | "eyr" | "hgt" | "hcl" | "ecl" | "pid" => sum += 1,
                _ => (),
            }
        }
    }

    println!("{}", valid);

    Ok(())
}
