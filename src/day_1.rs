use crate::helpers;
use std::cmp::Reverse;

fn get_inputs() -> Vec<u32> {
    let inputs = helpers::read_inputs_file(1)
        .into_iter();

    let mut inventories: Vec<u32> = vec![];
    let mut calories: u32 = 0;

    for line in inputs {
        if line.is_empty() {
            inventories.push(calories);
            calories = 0;

            continue;
        }

        calories += line.parse::<u32>().unwrap();
    }

    inventories.push(calories);
    inventories
}

pub fn solve_part(part: u8) -> u32 {
    let mut inventories = get_inputs();

    inventories.sort_by_key(|inventory| Reverse(*inventory));

    match part {
        1 => inventories[0],
        2 => inventories[0..3].iter().sum(),
        _ => panic!("Invalid part number"),
    }
}
