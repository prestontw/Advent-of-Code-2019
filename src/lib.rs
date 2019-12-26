pub mod day1 {
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
}

pub mod day2 {
  type Number = usize;

  #[derive(PartialEq)]
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
        99 => OpCode::Terminate,
        _ => panic!("non-op-code"),
      };

      if op == OpCode::Terminate {
        return data[0];
      }

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
}

pub mod day3 {
  // r75 d30 r83 u83 l12 d49 r71 u7 l72
  // u62 r66 u55 r34 d71 r55 d58 r83 => 159

  // r98 u47 r26 d63 r33 u87 l62 d20 r33 u53 r51
  // u98 r91 d20 r16 d67 r40 u7 r15 u6 r7 => 135

  pub fn part1(i: &str) -> usize {
    let mut lines = i.lines();
    let first = lines.next().unwrap();
    let second = lines.next().unwrap();

    let first = parse_line(first);
    let second = parse_line(second);

    let mut board = HashMap::new();
    let origin = (0, 0);

    first
      .iter()
      .fold((origin.clone(), 0), |(old_pos, count), mv| {
        move_direction(old_pos, count, mv, &Player::First, &mut board)
      });
    second
      .iter()
      .fold((origin.clone(), 0), |(old_pos, count), mv| {
        move_direction(old_pos, count, mv, &Player::Second, &mut board)
      });

    closest(intersections(board))
  }

  pub fn part2(i: &str) -> usize {
    let mut lines = i.lines();
    let first = lines.next().unwrap();
    let second = lines.next().unwrap();

    let first = parse_line(first);
    let second = parse_line(second);

    let mut board = HashMap::new();
    let origin = (0, 0);

    first
      .iter()
      .fold((origin.clone(), 0), |(old_pos, count), mv| {
        move_direction(old_pos, count, mv, &Player::First, &mut board)
      });
    second
      .iter()
      .fold((origin.clone(), 0), |(old_pos, count), mv| {
        move_direction(old_pos, count, mv, &Player::Second, &mut board)
      });

    soonest(intersecting_times(board))
  }

  #[test]
  fn test_samples() {
    let input = r#"R75,D30,R83,U83,L12,D49,R71,U7,L72
U62,R66,U55,R34,D71,R55,D58,R83"#;
    assert_eq!(part1(input), 159);
    let input = r#"R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"#;
    assert_eq!(part1(input), 135);
  }

  #[test]
  fn test_samples2() {
    let input = r#"R75,D30,R83,U83,L12,D49,R71,U7,L72
U62,R66,U55,R34,D71,R55,D58,R83"#;
    assert_eq!(part2(input), 610);
    let input = r#"R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"#;
    assert_eq!(part2(input), 410);
  }

  fn parse_line(s: &str) -> Vec<Move> {
    let parse_segment = |s: &str| {
      let (d, amount) = s.split_at(1);
      use Direction::*;
      let d = match d {
        "L" => Left,
        "R" => Right,
        "U" => Up,
        "D" => Down,
        _ => unimplemented!("four cardinal directions"),
      };
      let amount = amount.parse::<usize>().expect("psoitive number");
      Move(d, amount)
    };
    s.split(',').map(parse_segment).collect()
  }

  #[derive(Clone)]
  enum Direction {
    Up,
    Right,
    Left,
    Down,
  }

  struct Move(Direction, usize);

  use std::collections::{HashMap, HashSet};
  type Position = (isize, isize);
  type Board = HashMap<Position, (Option<usize>, Option<usize>)>;

  fn next_position(curpos: &Position, direction: &Direction) -> Position {
    use Direction::*;
    match direction {
      Down => (curpos.0, curpos.1 - 1),
      Right => (curpos.0 + 1, curpos.1),
      Left => (curpos.0 - 1, curpos.1),
      Up => (curpos.0, curpos.1 + 1),
    }
  }

  enum Player {
    First,
    Second,
  }

  fn move_direction(
    curpos: Position,
    count: usize,
    mv: &Move,
    player: &Player,
    mut board: &mut Board,
  ) -> (Position, usize) {
    let current = board.entry(curpos.clone()).or_insert((None, None));
    match player {
      // don't update if been there before
      Player::First => (*current).0 = Some(count),
      Player::Second => (*current).1 = Some(count),
    }
    if mv.1 == 0 {
      (curpos, count)
    } else {
      let next = next_position(&curpos, &mv.0);
      move_direction(
        next,
        count + 1,
        &Move(mv.0.clone(), mv.1 - 1),
        player,
        &mut board,
      )
    }
  }

  fn intersections(board: Board) -> HashSet<Position> {
    board
      .into_iter()
      .filter_map(|(k, v)| v.0.and_then(|_| v.1.and_then(|_| Some(k))))
      .collect()
  }

  fn intersecting_times(board: Board) -> HashSet<usize> {
    board
      .into_iter()
      .filter_map(|(_, v)| {
        v.0
          .and_then(|p1_time| v.1.and_then(|p2_time| Some(p1_time + p2_time)))
      })
      .collect()
  }

  fn closest(poss: HashSet<Position>) -> usize {
    poss
      .into_iter()
      .filter(|(x, y)| (x, y) != (&0, &0))
      .map(|(x, y)| x.abs() as usize + y.abs() as usize)
      .min()
      .expect("at least one intersection")
  }

  fn soonest(poss: HashSet<usize>) -> usize {
    poss
      .into_iter()
      .filter(|x| *x != 0)
      .min()
      .expect("at least one intersection")
  }
}

pub mod day4 {
  fn two_adjacent_same(i: usize) -> bool {
    let digits = num_to_digits(i);
    digits.windows(2).any(|i| i[0] == i[1])
  }

  fn exactly_two_adjacent_same(i: usize) -> bool {
    let digits = num_to_digits(i);
    let mut sames = vec![];
    for i in digits.iter() {
      let last = sames.last().as_ref().map(|v: &&Vec<u8>| v[0]);
      if Some(*i) != last {
        sames.push(vec![*i]);
      } else {
        sames.last_mut().map(|v| v.push(*i));
      }
    }
    sames.into_iter().any(|sub_list| sub_list.len() == 2)
  }

  fn all_increasing(i: usize) -> bool {
    let digits = num_to_digits(i);
    digits.windows(2).all(|i| i[0] <= i[1])
  }

  fn num_to_digits(i: usize) -> [u8; 6] {
    let mut current = i;
    let mut ret = [0; 6];
    for i in 0..6 {
      ret[i] = (current % 10) as u8;
      current = current / 10;
    }

    ret.reverse();

    ret
  }

  pub fn part1(lo: usize, hi: usize) -> usize {
    (lo..=hi)
      .filter(|&i| two_adjacent_same(i) && all_increasing(i))
      .count()
  }

  pub fn part2(lo: usize, hi: usize) -> usize {
    (lo..=hi)
      .filter(|&i| exactly_two_adjacent_same(i) && all_increasing(i))
      .count()
  }
}
