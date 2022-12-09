use crate::helpers;

struct Tree {
  position_x: usize,
  position_y: usize,
  height: u8,
}

impl Tree {
  fn is_visible(&self, trees: &Vec<Vec<Tree>>) -> bool {
    if self.position_x == 0 || self.position_x == trees.len() - 1 {
      return true;
    }

    if self.position_y == 0 || self.position_y == trees[0].len() -1 {
      return true;
    }

    let row = &trees[self.position_x];

    if row[0..self.position_y].iter().all(|tree| tree.height < self.height) {
      return true;
    }

    if row[self.position_y + 1..].iter().all(|tree| tree.height < self.height) {
      return true;
    }

    let mut is_tallest = true;

    // check up
    let mut current_x = self.position_x - 1;

    loop {
      if trees[current_x][self.position_y].height >= self.height {
        is_tallest = false;

        break;
      }

      if current_x > 0 {
        current_x -= 1;
      } else {
        break;
      }
    }

    if is_tallest {
      return true
    }

    // check down
    is_tallest = true;
    current_x = self.position_x + 1;

    while current_x < trees[0].len() {
      if trees[current_x][self.position_y].height >= self.height {
        is_tallest = false;

        break;
      }

      current_x += 1;
    }

    is_tallest
  }
}

pub fn solve_part(part: u8) -> u32 {
  let inputs = helpers::read_inputs_file(8);
  let mut trees: Vec<Vec<Tree>> = vec![];

  for x in 0..inputs.len() {
    let mut row: Vec<Tree> = vec![];
    let mut y: usize = 0;

    for digit in inputs[x].clone().chars() {
      row.push(Tree {
        position_x: x,
        position_y: y,
        height: digit as u8,
      });

      y += 1;
    }

    trees.push(row);
  }

  match part {
    1 => {
      let mut count: u32 = 0;

      trees.iter().for_each(|row| row.iter().for_each(|t| if t.is_visible(&trees) { 
        count += 1;
      }));

      count
    },
    _ => panic!("Invalid part number"),
  }
}