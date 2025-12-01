use regex::Regex;

use crate::utils;

pub fn part1()
{
    let re = Regex::new(r"^(L|R){1}(\d+)$").unwrap();
    let mut position: u32 = 50;
    let mut amount: u32 = 0;

    if let Ok(lines) = utils::read_lines("src/day1/input")
    {
        for line in lines
        {
            if let Ok(l) = line
            {
                let caps = re.captures(&l).unwrap();

                let direction = caps.get(1).unwrap().as_str();
                let distance: u32 = caps.get(2).unwrap().as_str().parse::<u32>().unwrap();

                let norm_distance = distance % 100;

                if direction == "L"
                {
                    position = (position + (100 - norm_distance)) % 100;
                }
                else if direction == "R"
                {
                    position = (position + norm_distance) % 100;
                }

                if position == 0
                {
                    amount += 1;
                }
            }
        }
    }

    println!("Amount: {}", amount);
}
