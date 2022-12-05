use crate::helpers;

fn build_vector_from_range(range: &str) -> Vec<u8> {
  let splits = range.split('-').collect::<Vec<&str>>();
  let start = splits[0].parse::<u8>().unwrap();
  let end = splits[1].parse::<u8>().unwrap();

  let mut result = vec![];

  for i in start..(end + 1) {
    result.push(i);
  }

  result
}

pub fn solve_part(part: u8) -> u16 {
  let inputs = helpers::read_inputs_file(4);

  match part {
    1 => {
      let mut total: u16 = 0;

      for input in inputs {
        let mut splits = input.split(',').collect::<Vec<&str>>();
        let first = build_vector_from_range(splits[0]);
        let second = build_vector_from_range(splits[1]);
        let mut combined = [first.clone(), second.clone()].concat();
        combined.sort();
        combined.dedup();

        if combined.len() == first.len() || combined.len() == second.len() {
          total += 1;
        }
      }

      return total;
    },
    _ => panic!("Invalid part number"),
  }
}