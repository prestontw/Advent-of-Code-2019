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
