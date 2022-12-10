use crate::helpers;
use std::collections::HashMap;

#[derive(Clone, Eq, Hash, PartialEq)]
struct Position {
  x: i16,
  y: i16,
}

fn get_2d_distance(position_1: &Position, position_2: &Position) -> u8 {
  let x_diff = position_2.x - position_1.x;
  let y_diff = position_2.y - position_1.y;

  f64::sqrt((x_diff.pow(2) + y_diff.pow(2)).into()) as u8
}

struct Rope {
  knots: Vec<Position>,
}

impl Rope {
  fn move_by(&mut self, x_change: i16, y_change: i16) {
    self.knots[0].x += x_change;
    self.knots[0].y += y_change;

    for i in 1..self.knots.len() {
      let distance = get_2d_distance(&self.knots[i], &self.knots[i - 1]);

      if distance <= 1 {
        // once we find a knot that doesn't need to move, the rest of the knots also do not need to move
        return
      }

      let x_diff = self.knots[i - 1].x - self.knots[i].x;
      let y_diff = self.knots[i - 1].y - self.knots[i].y;

      if x_diff > 0 {
        self.knots[i].x += 1;
      } else if x_diff < 0 {
        self.knots[i].x -= 1;
      }

      if y_diff > 0 {
        self.knots[i].y += 1;
      } else if y_diff < 0 {
        self.knots[i].y -= 1;
      }
    }
  }
}

pub fn solve_part(part: u8) -> usize {
  let starting_position = Position {
    x: 0,
    y: 0,
  };

  let mut rope = match part {
    1 => Rope {
      knots: vec![starting_position; 2],
    },
    2 => Rope {
      knots: vec![starting_position; 10],
    },
    _ => panic!("Invalid part number"),
  };

  let inputs = helpers::read_inputs_file(9);
  let mut tail_positions_visited = HashMap::new();

  for input in inputs {
    let mut splits = input.split_whitespace();
    let direction: char = splits.next().unwrap().parse::<char>().unwrap();
    let distance: u8 = splits.next().unwrap().parse::<u8>().unwrap();

    let change = match direction {
      'U' => [1, 0],
      'R' => [0, 1],
      'D' => [-1, 0],
      'L' => [0, -1],
      _ => panic!("Invalid direction"),
    };

    for _i in 0..distance {
      rope.move_by(change[0], change[1]);

      tail_positions_visited.entry(rope.knots[rope.knots.len() - 1].clone()).or_insert(0);
    }
  }

  tail_positions_visited.len()
}