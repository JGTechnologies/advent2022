use crate::helpers;

pub fn solve_part(part: u8) -> usize {
  let input = helpers::read_inputs_file(6).pop().unwrap();
  let bytes = input.as_bytes();

  for i in 0..bytes.len() {
    let mut v = vec![];

    for j in 0..4 {
      v.push(bytes[i + j]);
    }

    v.sort();
    v.dedup();

    if v.len() == 4 {
      return i + 4;
    }
  }

  panic!("No result found");
}