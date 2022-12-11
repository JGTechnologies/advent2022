use crate::helpers;
use std::collections::HashMap;

pub fn solve_part(part: u8) -> u32 {
  let inputs = helpers::read_inputs_file(10);
  let mut cycle = 0;
  let mut cycles = HashMap::from([
    (0, 1)
  ]);

  for _i in 0..inputs.len() {
    let input = inputs[_i].clone();
    let current_value = cycles.get_key_value(&(cycles.len() - 1)).unwrap().1;

    if input == "noop" {
      cycles.insert(cycle, *current_value);

      cycle += 1;
      continue;
    }

    let mut splits = input.split_whitespace();
    splits.next();
    let value = splits.next().unwrap().parse::<i32>().unwrap();

    let next = current_value + value;

    cycles.insert(cycle, *current_value);
    cycles.insert(cycle + 1, next);

    cycle += 2;
  }

  match part {
    1 => {
      let keys: Vec<i32> = vec![20, 60, 100, 140, 180, 220];
      let mut value: i32 = 0;

      for key in keys {
        let x: i32 = (cycles.get_key_value(&(key as usize - 2)).unwrap().1 * key).try_into().unwrap();
        value += x;
      }

      value as u32
    },
    _ => panic!("Invalid part number"),
  }
}