use crate::helpers;

fn get_initial_state() -> [Vec<char>; 9] {
    [
        vec!['F', 'H', 'B', 'V', 'R', 'Q', 'D', 'P'],
        vec!['L', 'D', 'Z', 'Q', 'W', 'V'],
        vec!['H', 'L', 'Z', 'Q', 'G', 'R', 'P', 'C'],
        vec!['R', 'D', 'H', 'F', 'J', 'V', 'B'],
        vec!['Z', 'W', 'L', 'C'],
        vec!['J', 'R', 'P', 'N', 'T', 'G', 'V', 'M'],
        vec!['J', 'R', 'L', 'V', 'M', 'B', 'S'],
        vec!['D', 'P', 'J'],
        vec!['D', 'C', 'N', 'W', 'V'],
    ]
}

struct Instruction {
    count: u8,
    source: usize,
    destination: usize,
}

impl Instruction {
    fn parse(input: String) -> Instruction  {
        let mut splits = input.split_whitespace();
        splits.next();
        let count = splits.next().unwrap().parse::<u8>().unwrap();
        splits.next();
        let source = splits.next().unwrap().parse::<usize>().unwrap() - 1;
        splits.next();
        let destination = splits.next().unwrap().parse::<usize>().unwrap() - 1;

        Instruction {
            count,
            source,
            destination,
        }
    }
}

pub fn solve_part(part: u8) -> String {
    let inputs = helpers::read_inputs_file(5);
    let mut state = get_initial_state();

    for input in inputs {
        let instruction = Instruction::parse(input);

        match part {
            1 => {
                for _i in 0..instruction.count {
                    let v = state[instruction.source].pop();
                    state[instruction.destination].push(v.unwrap());
                }
            },
            2 => {
                // we can achieve the "move 3 boxes but maintain order" by moving them into a new
                // stack one at a time, before moving them to their destination stack.
                let mut placeholder: Vec<char> = vec![];

                for _i in 0..instruction.count {
                    let x = state[instruction.source].pop();
                    placeholder.push(x.unwrap());
                }

                for _i in 0..placeholder.len() {
                    let v = placeholder.pop();
                    state[instruction.destination].push(v.unwrap());
                }
            },
            _ => panic!("Invalid part number"),
        };
    }

    let mut result = String::from("");

    for stack in state {
        result.push(*stack.last().unwrap());
    }

    result
}
