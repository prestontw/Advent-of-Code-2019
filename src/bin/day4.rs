fn main() {
  let (lo, hi) = (235741, 706948);
  println!("{}", part1(lo, hi));
  println!("{}", part2(lo, hi));
}

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
