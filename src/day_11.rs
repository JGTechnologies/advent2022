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
  items: Vec<u64>,
  operation_type: OperationType,
  operation_amount: u64,
  test: u64,
  test_pass_target: u8,
  test_fail_target: u8,
  items_inspected: u64,
}

impl Monkey {
  fn execute_turn(&mut self, monkeys: &mut Vec<Monkey>, divisor: u64) {
    for item in &self.items {
      let mut worry_level = match &self.operation_type {
        OperationType::ADD => item + self.operation_amount,
        OperationType::MULTIPLY => item * self.operation_amount,
        OperationType::POWER => item.pow(self.operation_amount as u32),
      };

      if divisor == 3 {
        worry_level /= 3;
      } else {
        worry_level %= divisor;
      }

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

pub fn solve_part(part: u8) -> u64 {
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

  let target_rounds = match part {
    1 => 20,
    2 => 10000,
    _ => panic!("Invalid part number"),
  };

  let divisor = match part {
    1 => 3,
    2 => {
      let mut result = 1;

      for monkey in &monkeys {
        result *= monkey.test;
      }

      result
    },
    _ => panic!("Invalid part number"),
  };

  for _round in 0..target_rounds {
    for i in 0..monkeys.clone().len() {
      let mut monkey = monkeys[i].clone();

      monkey.execute_turn(&mut monkeys, divisor);
    }
  }

  monkeys.sort_by_key(|m| Reverse(m.items_inspected));

  monkeys[0].items_inspected * monkeys[1].items_inspected
}