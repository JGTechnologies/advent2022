use crate::helpers;

struct Inventory {
    calories: Vec<u32>,
}

fn get_part_1_inputs() -> Vec<Inventory> {
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
    match part {
        1 => {
            let inventories = get_part_1_inputs();
            let mut max: u32 = 0;

            for inventory in inventories {
                let total = inventory.calories.iter().map(|&item| item).sum::<u32>();

                if total > max {
                    max = total;
                }
            }

            max
        },
        _ => panic!("Invalid part number"),
    }
}
