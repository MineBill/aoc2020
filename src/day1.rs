use crate::Result;

pub fn main() -> Result {
    let input = std::fs::read_to_string("data/day1.txt")?;

    let nums = input.lines().map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>();

    for (i, curr) in nums.iter().enumerate() {
        for next in i..nums.len() {
            for third in next..nums.len() {
                let next = nums.get(next).unwrap();
                let third = nums.get(third).unwrap();
                if curr + next + third == 2020 {
                    println!("{}", curr * next * third);
                    break;
                }
            }
        }
    }

    Ok(())
}