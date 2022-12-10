use crate::helpers;
use std::collections::HashMap;

fn get_2d_distance(position_1: &[i16; 2], position_2: &[i16; 2]) -> u8 {
  let x_diff = position_2[0] - position_1[0];
  let y_diff = position_2[1] - position_1[1];

  f64::sqrt((x_diff.pow(2) + y_diff.pow(2)).into()) as u8
}

pub fn solve_part(part: u8) -> usize {
  let mut head_position = [0, 0];
  let mut tail_position = [0, 0];

  let inputs = helpers::read_inputs_file(9);
  let mut tail_positions_visited = HashMap::new();

  for input in inputs {
    let mut splits = input.split_whitespace();
    let direction: char = splits.next().unwrap().parse::<char>().unwrap();
    let distance: u8 = splits.next().unwrap().parse::<u8>().unwrap();

    for _i in 0..distance {
      let starting_head_position = head_position.clone();

      match direction {
        'U' => head_position[0] -= 1,
        'R' => head_position[1] += 1,
        'D' => head_position[0] += 1,
        'L' => head_position[1] -= 1,
        _ => panic!("Invalid direction"),
      };

      if get_2d_distance(&head_position, &tail_position) > 1 {
        tail_position = starting_head_position;
      }

      tail_positions_visited.entry(tail_position).or_insert(0);
    }
  }

  match part {
    1 => tail_positions_visited.len(),
    2 => 0,
    _ => panic!("Invalid part number"),
  }
}