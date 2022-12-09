use crate::helpers;

struct Tree {
  position_x: usize,
  position_y: usize,
  height: u8,
}

impl Tree {
  fn get_scenic_score(&self, trees: &Vec<Vec<Tree>>) -> u32 {
    let mut score: u32 = 1;

    if self.is_on_edge(&trees) {
      return 0
    }

    // check left
    let mut current_y = self.position_y - 1;
    let mut counter: u32 = 0;

    loop {
      counter += 1;

      if trees[self.position_x][current_y].height >= self.height || current_y == 0 {
        break;
      }

      current_y -= 1;
    }

    score *= counter;
    counter = 0;

    // check right
    current_y = self.position_y + 1;

    while current_y < trees.len() {
      counter += 1;

      if trees[self.position_x][current_y].height >= self.height {
        break;
      }

      current_y += 1;
    }

    score *= counter;
    counter = 0;
    
    let mut current_x = self.position_x - 1;

    // check up
    loop {
      counter += 1;

      if trees[current_x][self.position_y].height >= self.height || current_x == 0 {
        break;
      }

      current_x -= 1;
    }

    score *= counter;
    counter = 0;

    // check down
    current_x = self.position_x + 1;

    while current_x < trees[0].len() {
      counter += 1;

      if trees[current_x][self.position_y].height >= self.height {
        break;
      }

      current_x += 1;
    }

    score * counter
  }

  fn is_on_edge(&self, trees: &Vec<Vec<Tree>>) -> bool {
    if self.position_x == 0 || self.position_x == trees.len() - 1 {
      return true;
    }

    if self.position_y == 0 || self.position_y == trees[0].len() -1 {
      return true;
    }

    false
  }

  fn is_visible(&self, trees: &Vec<Vec<Tree>>) -> bool {
    if self.is_on_edge(&trees) {
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
    2 => {
      let mut max_score: u32 = 0;

      for row in &trees {
        for tree in row {
          let score = tree.get_scenic_score(&trees);

          if score > max_score {
            max_score = score;
          }
        }
      }

      max_score
    },
    _ => panic!("Invalid part number"),
  }
}