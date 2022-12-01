use crate::helpers;
use std::cmp::Reverse;

struct Inventory {
    calories: Vec<u32>,
}

impl Inventory {
    fn total_calories(&self) -> u32 {
        self.calories.iter().sum()
    }
}

fn get_inputs() -> Vec<Inventory> {
    let inputs = helpers::read_inputs_file(1)
        .into_iter();

    let mut inventories: Vec<Inventory> = vec![];
    let mut inventory = Inventory { calories: vec![] };

    for line in inputs {
        if line.is_empty() {
            inventories.push(inventory);
            inventory = Inventory { calories: vec![] };

            continue;
        }

        inventory.calories.push(line.parse::<u32>().unwrap());
    }

    inventories.push(inventory);
    inventories
}

pub fn solve_part(part: u8) -> u32 {
    let mut inventories = get_inputs();
    let mut max: u32 = 0;

    for inventory in &inventories {
        let total = inventory.total_calories();

        if total > max {
            max = total;
        }
    }

    inventories.sort_by_key(|inventory| Reverse(inventory.total_calories()));

    match part {
        1 => inventories[0].total_calories(),
        2 => inventories[0..3].iter().map(|inventory| inventory.total_calories()).sum(),
        _ => panic!("Invalid part number"),
    }
}
