use crate::helpers;

pub fn solve_part(part: u8) -> usize {
  let input = helpers::read_inputs_file(6).pop().unwrap();
  let bytes = input.as_bytes();

  for i in 0..bytes.len() {
    let mut v = vec![];

    let size = match part {
      1 => 4,
      2 => 14,
      _ => panic!("Invalid part number"),
    };

    for j in 0..size {
      v.push(bytes[i + j]);
    }

    v.sort();
    v.dedup();

    if v.len() == size {
      return i + size;
    }
  }

  panic!("No result found");
}