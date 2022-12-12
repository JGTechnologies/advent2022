use std::cmp::Reverse;

#[derive(Clone)]
enum OperationType {
  ADD,
  MULTIPLY,
  POWER,
}

#[derive(Clone)]
struct Monkey {
  id: usize,
  items: Vec<u32>,
  operation_type: OperationType,
  operation_amount: u32,
  test: u32,
  test_pass_target: u8,
  test_fail_target: u8,
  items_inspected: u32,
}

impl Monkey {
  fn execute_turn(&mut self, monkeys: &mut Vec<Monkey>) {
    for item in &self.items {
      let worry_level = match &self.operation_type {
        OperationType::ADD => (item + self.operation_amount) / 3,
        OperationType::MULTIPLY => (item * self.operation_amount) / 3,
        OperationType::POWER => item.pow(self.operation_amount.into()) / 3,
      };

      monkeys[self.id].items_inspected += 1;

      if worry_level % self.test == 0 {
        monkeys[self.test_pass_target as usize].items.push(worry_level);
      } else {
        monkeys[self.test_fail_target as usize].items.push(worry_level);
      }
    }

    monkeys[self.id].items.clear();
  }
}

pub fn solve_part(part: u8) -> u32 {
  /*
  let mut monkeys = vec![
    Monkey {
      id: 0,
      items: vec![79, 98],
      operation_type: OperationType::MULTIPLY,
      operation_amount: 19,
      test: 23,
      test_pass_target: 2,
      test_fail_target: 3,
      items_inspected: 0,
    },
    Monkey {
      id: 1,
      items: vec![54, 65, 75, 74],
      operation_type: OperationType::ADD,
      operation_amount: 6,
      test: 19,
      test_pass_target: 2,
      test_fail_target: 0,
      items_inspected: 0,
    },
    Monkey {
      id: 2,
      items: vec![79, 60, 97],
      operation_type: OperationType::POWER,
      operation_amount: 2,
      test: 13,
      test_pass_target: 1,
      test_fail_target: 3,
      items_inspected: 0,
    },
    Monkey {
      id: 3,
      items: vec![74],
      operation_type: OperationType::ADD,
      operation_amount: 3,
      test: 17,
      test_pass_target: 0,
      test_fail_target: 1,
      items_inspected: 0,
    },
  ];
  */

  let mut monkeys = vec![
    Monkey {
      id: 0,
      items: vec![83, 97, 95, 67],
      operation_type: OperationType::MULTIPLY,
      operation_amount: 19,
      test: 17,
      test_pass_target: 2,
      test_fail_target: 7,
      items_inspected: 0,
    },
    Monkey {
      id: 1,
      items: vec![71, 70, 79, 88, 56, 70],
      operation_type: OperationType::ADD,
      operation_amount: 2,
      test: 19,
      test_pass_target: 7,
      test_fail_target: 0,
      items_inspected: 0,
    },
    Monkey {
      id: 2,
      items: vec![98, 51, 51, 63, 80, 85, 84, 95],
      operation_type: OperationType::ADD,
      operation_amount: 7,
      test: 7,
      test_pass_target: 4,
      test_fail_target: 3,
      items_inspected: 0,
    },
    Monkey {
      id: 3,
      items: vec![77, 90, 82, 80, 79],
      operation_type: OperationType::ADD,
      operation_amount: 1,
      test: 11,
      test_pass_target: 6,
      test_fail_target: 4,
      items_inspected: 0,
    },
    Monkey {
      id: 4,
      items: vec![68],
      operation_type: OperationType::MULTIPLY,
      operation_amount: 5,
      test: 13,
      test_pass_target: 6,
      test_fail_target: 5,
      items_inspected: 0,
    },
    Monkey {
      id: 5,
      items: vec![60, 94],
      operation_type: OperationType::ADD,
      operation_amount: 5,
      test: 3,
      test_pass_target: 1,
      test_fail_target: 0,
      items_inspected: 0,
    },
    Monkey {
      id: 6,
      items: vec![81, 51, 85],
      operation_type: OperationType::POWER,
      operation_amount: 2,
      test: 5,
      test_pass_target: 5,
      test_fail_target: 1,
      items_inspected: 0,
    },
    Monkey {
      id: 7,
      items: vec![98, 81, 63, 65, 84, 71, 84],
      operation_type: OperationType::ADD,
      operation_amount: 3,
      test: 2,
      test_pass_target: 2,
      test_fail_target: 3,
      items_inspected: 0,
    },
  ];

  let target_rounds = 20;

  for _round in 0..target_rounds {
    for i in 0..monkeys.clone().len() {
      let mut monkey = monkeys[i].clone();

      monkey.execute_turn(&mut monkeys);
    }
  }

  match part {
    1 => {
      monkeys.sort_by_key(|m| Reverse(m.items_inspected));

      return (monkeys[0].items_inspected * monkeys[1].items_inspected).into();
    },
    2 => 0,
    _ => panic!("Invalid part number"),
  }
}