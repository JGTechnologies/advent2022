use crate::helpers;
use std::collections::HashMap;

struct File {
  name: String,
  size: u32,
}

impl File {
  fn get_dir(&self) -> String { 
    let mut splits: Vec<&str> = self.name.split("/").collect();

    splits.pop();

    if splits.len() == 1 {
      // root paths should return "/" not ""
      return "/".to_string()
    }

    let mut result = splits.join("/");
    result.push_str("/");

    result
  }
}

pub fn solve_part(part: u8) -> u32 {
  let inputs = helpers::read_inputs_file(7);
  let mut files: Vec<File> = vec![];
  let mut current_path: Vec<String> = vec!["".to_string()];

  for input in inputs {
    if input == "$ ls" || input.starts_with("dir ") {
      continue;
    }

    if input == "$ cd /" {
      current_path = vec!["".to_string()];

      continue;
    }

    if input == "$ cd .." {
      current_path.pop();

      continue;
    }

    if input.starts_with("$ cd ") {
      let mut splits = input.split("$ cd ");
      splits.next();
      let path = splits.next().unwrap().to_string();
      current_path.push(path);

      continue;
    }

    // only files make it this far
    let mut splits = input.split_whitespace();
    let size = splits.next().unwrap().parse::<u32>().unwrap();
    let name = splits.next().unwrap().to_string();
    let mut path = current_path.join("/");
    path.push_str(&"/".to_string());
    path.push_str(&name);

    files.push(File {
      name: path,
      size,
    });
  }

  let mut tree = HashMap::new();

  for file in files {
    let dir = file.get_dir();
    let mut path = "".to_string();
    let splits = dir.split("/");

    for split in splits {
      if split.len() == 0 {
        continue;
      }

      path.push_str(split);
      path.push_str(&"/".to_string());
      tree.entry(path.clone()).or_insert(0);
      tree.entry(path.clone()).and_modify(|v| *v += file.size);
    }
  }

  let mut result: u32 = 0;

  match part {
    1 => tree.into_values().collect::<Vec<u32>>().iter().filter(|v| **v <= 100000).sum(),
    2 => 0,
    _ => panic!("Invalid part number"),
  }
}