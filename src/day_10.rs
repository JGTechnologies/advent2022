use crate::helpers;
use std::collections::HashMap;

pub fn solve_part(part: u8) -> u32 {
  let inputs = helpers::read_inputs_file(10);
  let mut cycle = 0;
  let mut cycles = HashMap::from([
    (0, 1)
  ]);

  for _i in 0..inputs.len() {
    cycle += 1;
    let input = inputs[_i].clone();
    let current_value = *cycles.get_key_value(&(cycles.len() - 1)).unwrap().1;

    if input == "noop" {
      cycles.insert(cycle, current_value);

      continue;
    }

    let mut splits = input.split_whitespace();
    splits.next();
    let value = splits.next().unwrap().parse::<i32>().unwrap();

    cycles.insert(cycle, current_value);
    cycle += 1;

    cycles.insert(cycle, current_value + value);
  }

  match part {
    1 => {
      let keys: Vec<i32> = vec![20, 60, 100, 140, 180, 220];
      let mut value: i32 = 0;

      for key in keys {
        let x: i32 = (cycles.get_key_value(&(key as usize - 1)).unwrap().1 * key).try_into().unwrap();
        value += x;
      }

      value as u32
    },
    2 => {
      let mut pixel: i32 = 0;

      for row in 0..6 {
        for col in 0..40 {
          let position = row * 40 + col;
          let sprite_center: i32 = *cycles.get_key_value(&position).unwrap().1;

          if (sprite_center - (position as i32) % 40).abs() <= 1 {
            print!("{}", '#');
          } else {
            print!("{}", '.');
          }

          pixel += 1;
        }

        println!("");
      }

      8
    }
    _ => panic!("Invalid part number"),
  }
}