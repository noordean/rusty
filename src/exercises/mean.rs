pub fn run(integers: Vec<i8>) -> f64 {
  let mut total = 0;
  for i in &integers {
    total += i;
  }

  total as f64 / integers.len() as f64
}
