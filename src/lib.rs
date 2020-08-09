type Number = i32;

pub struct IntcodeInterpreter();
#[derive(Copy, Clone)]
enum Advance {
  Relative(usize),
  Absolute(usize),
}
enum Output {
  Continue {
    display: Option<Number>,
    positions_to_step: Advance,
  },
  Terminate,
}
#[derive(Copy, Clone)]
enum ParameterMode {
  Immediate,
  Position,
}

impl std::ops::AddAssign<Advance> for usize {
  fn add_assign(&mut self, rhs: Advance) {
    match rhs {
      Advance::Absolute(new_pos) => *self = new_pos,
      Advance::Relative(offset) => *self += offset,
    }
  }
}
impl std::ops::Add<Advance> for usize {
  type Output = usize;
  fn add(self, rhs: Advance) -> Self::Output {
    match rhs {
      Advance::Absolute(new_pos) => new_pos,
      Advance::Relative(offset) => self + offset,
    }
  }
}

impl IntcodeInterpreter {
  fn int_to_code(input: Number) -> OpCode {
    use OpCode::*;
    match input {
      1 => Add,
      2 => Multiply,
      3 => Store,
      4 => Read,
      5 => JumpIfNonZero,
      6 => JumpIfZero,
      7 => LessThan,
      8 => Equals,
      99 => Terminate,
      _ => panic!("non-op-code"),
    }
  }
  fn one_parameter_mode(input: Number) -> [ParameterMode; 1] {
    [if input / 100 % 10 == 1 {
      ParameterMode::Immediate
    } else {
      ParameterMode::Position
    }]
  }
  fn two_parameter_mode(input: Number) -> [ParameterMode; 2] {
    [
      if input / 100 % 10 == 1 {
        ParameterMode::Immediate
      } else {
        ParameterMode::Position
      },
      if input / 1000 % 10 == 1 {
        ParameterMode::Immediate
      } else {
        ParameterMode::Position
      },
    ]
  }
  fn eval_param(mode: ParameterMode, offset: usize, tape: &[Number]) -> Number {
    match (mode, tape[offset]) {
      (ParameterMode::Immediate, val) => val,
      (ParameterMode::Position, loc) => tape[loc as usize],
    }
  }
  /// Interpret the opcode at the current position and return the number of steps to move
  fn interpret_at_location(tape: &mut [Number], location: usize, input: Option<Number>) -> Output {
    use OpCode::*;
    let opcode = tape[location];
    let operation = opcode % 100;
    let op = Self::int_to_code(operation);
    match op {
      Add | Multiply => {
        // get param modes of first, second arguments
        let param_modes = Self::two_parameter_mode(opcode);
        let first_arg_value = Self::eval_param(param_modes[0], location + 1, tape);
        let second_arg_value = Self::eval_param(param_modes[1], location + 2, tape);
        let store_location = tape[location + 3] as usize;
        use std::ops::{Add, Mul};
        let op = match op {
          Add => i32::add,
          Multiply => i32::mul,
          _ => unreachable!(),
        };
        tape[store_location] = op(first_arg_value, second_arg_value);

        Output::Continue {
          display: None,
          positions_to_step: Advance::Relative(4),
        }
      }
      OpCode::LessThan | OpCode::Equals => {
        let param_modes = Self::two_parameter_mode(opcode);
        let first_arg = Self::eval_param(param_modes[0], location + 1, tape);
        let second_arg = Self::eval_param(param_modes[1], location + 2, tape);
        let store_location = tape[location + 3] as usize;
        let op = match op {
          OpCode::LessThan => i32::lt,
          OpCode::Equals => i32::eq,
          _ => unreachable!(),
        };
        tape[store_location] = if op(&first_arg, &second_arg) { 1 } else { 0 };

        Output::Continue {
          display: None,
          positions_to_step: Advance::Relative(4),
        }
      }
      OpCode::JumpIfNonZero | OpCode::JumpIfZero => {
        let param_modes = Self::two_parameter_mode(opcode);
        let condition = Self::eval_param(param_modes[0], location + 1, tape);
        let potential_jump_location = Self::eval_param(param_modes[1], location + 2, tape);
        Output::Continue {
          display: None,
          positions_to_step: match (op, condition) {
            (JumpIfNonZero, 0) => Advance::Relative(3),
            (JumpIfNonZero, _) => Advance::Absolute(potential_jump_location as usize),
            (JumpIfZero, 0) => Advance::Absolute(potential_jump_location as usize),
            (JumpIfZero, _) => Advance::Relative(3),
            (_, _a) => unreachable!(),
          },
        }
      }
      Store => {
        tape[tape[location + 1] as usize] = input.unwrap();
        Output::Continue {
          display: None,
          positions_to_step: Advance::Relative(2),
        }
      }
      Read => {
        let param_mode = Self::one_parameter_mode(opcode);
        Output::Continue {
          display: Some(Self::eval_param(param_mode[0], location + 1, tape)),
          positions_to_step: Advance::Relative(2),
        }
      }
      Terminate => Output::Terminate,
    }
  }
  pub fn interpret_intcode_program(input: &[Number]) -> Number {
    let mut data = input.to_vec();
    let mut index = 0;
    while index <= data.len() {
      let res = Self::interpret_at_location(&mut data, index, None);
      match res {
        Output::Continue {
          display,
          positions_to_step,
        } => {
          if let Some(output) = display {
            debug_assert!(output == 0);
          }
          match positions_to_step {
            Advance::Absolute(new_pos) => index = new_pos,
            Advance::Relative(offset) => index += offset,
          }
        }
        Output::Terminate => break,
      }
    }
    data[0]
  }
  pub fn return_final_display(input: &[Number], arg: Number) -> Number {
    let mut data = input.to_vec();
    let mut index = 0;
    while index <= data.len() {
      let res = Self::interpret_at_location(&mut data, index, Some(arg));
      match res {
        Output::Continue {
          display: Some(final_display),
          positions_to_step,
        } if data[index + positions_to_step] == 99 => {
          return final_display;
        }
        Output::Continue {
          display: Some(display),
          positions_to_step,
        } => {
          debug_assert!(display == 0);
          index += positions_to_step;
        }
        Output::Continue {
          display: None,
          positions_to_step,
        } => index += positions_to_step,
        Output::Terminate => break,
      }
    }
    0
  }

  fn find_19690720(input: &[Number]) -> (usize, usize) {
    for noun in 0..=99 {
      for verb in 0..=99 {
        let mut input = input.to_vec();
        input[1] = noun;
        input[2] = verb;
        let result = Self::interpret_intcode_program(&input);
        if result == 19_690_720 {
          return (noun as usize, verb as usize);
        }
      }
    }
    (0, 0)
  }

  pub fn find_noun_verb(input: &[Number]) -> usize {
    let (noun, verb) = Self::find_19690720(input.clone());
    noun * 100 + verb
  }
}
#[derive(PartialEq)]
enum OpCode {
  Add,
  Multiply,
  Terminate,
  Store,
  Read,
  JumpIfNonZero,
  JumpIfZero,
  LessThan,
  Equals,
}

#[test]
fn test_sample_interpret_intcode() {
  assert_eq!(
    IntcodeInterpreter::interpret_intcode_program(&[1, 1, 1, 4, 99, 5, 6, 0, 99]),
    30
  );
}

#[test]
fn test_parameter_mode() {
  assert_eq!(
    IntcodeInterpreter::interpret_intcode_program(&[1002, 4, 3, 0, 99]),
    297
  )
}
