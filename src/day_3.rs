use crate::helpers;

struct Rucksack {
  first: Vec<char>,
  second: Vec<char>,
}

impl Rucksack {
  fn find_bad_items(&mut self) -> Vec<char> {
    let mut items: Vec<char> = vec![];

    &self.first.sort();
    &self.first.dedup();

    for item in &self.first {
      if self.second.iter().any(|i| i == item) {
        items.push(*item);
      }
    }

    items
  }
}

fn get_item_priority(item: char) -> u32 {
  if item.is_uppercase() {
    // uppercase ascii starts at 65, but priority starts at 27
    return (item as u32) - (65 - 27);
  }

  return (item as u32) - (97 - 1);
}

pub fn solve_part(part: u8) -> u32 {
  let inputs = helpers::read_inputs_file(3);
  let mut total: u32 = 0;
  let mut rucksacks: Vec<Rucksack> = vec![];

  for input in inputs {
    let length = input.chars().count();

    rucksacks.push(Rucksack {
      first: input[0..length / 2].chars().collect(),
      second: input[length / 2..length].chars().collect(),
    });
  }

  match part {
    1 => {
      for mut rucksack in rucksacks {
        for item in rucksack.find_bad_items() {
          total += get_item_priority(item);
        }
      }

      total
    },
    2 => panic!("Not yet built"),
    _ => panic!("Invalid part number"),
  }
}