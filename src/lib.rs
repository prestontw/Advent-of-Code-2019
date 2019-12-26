pub fn fuel_for_module(u: usize) -> usize {
  (u / 3) - 2
}

pub fn fuel_for_fuel_for_module(u: usize) -> Option<usize> {
  if u > 8 {
    let fuel = fuel_for_module(u);
    Some(fuel + fuel_for_fuel_for_module(fuel).unwrap_or(0))
  } else {
    None
  }
}

type Number = usize;

enum OpCode {
  Add,
  Multiply,
  Terminate,
}

pub fn interpret_intcode_program(input: &[Number]) -> Number {
  let mut data = input.to_vec();
  let mut index = 0;
  while index <= data.len() {
    let op = match data[index] {
      1 => OpCode::Add,
      2 => OpCode::Multiply,
      99 => return data[0],
      _ => panic!("non-op-code"),
    };
    let pos1 = data[index + 1];
    let pos2 = data[index + 2];
    let store_pos = data[index + 3];

    let a = data[pos1];
    let b = data[pos2];
    data[store_pos] = match op {
      OpCode::Add => a + b,
      OpCode::Multiply => a * b,
      _ => panic!("should have already terminated"),
    };
    index += 4;
  }
  data[0]
}

fn find_19690720(input: &[Number]) -> (usize, usize) {
  for noun in 0..=99 {
    for verb in 0..=99 {
      let mut input = input.to_vec();
      input[1] = noun;
      input[2] = verb;
      let result = interpret_intcode_program(&input);
      if result == 19_690_720 {
        return (noun, verb);
      }
    }
  }
  (0, 0)
}

pub fn find_noun_verb(input: &[Number]) -> usize {
  let (noun, verb) = find_19690720(input.clone());
  noun * 100 + verb
}

#[test]
fn test_sample_interpret_intcode() {
  assert_eq!(
    interpret_intcode_program(&[1, 1, 1, 4, 99, 5, 6, 0, 99]),
    30
  );
}
