pub fn run(integers: Vec<i8>) -> f64 {
  let mut integers = integers;
  let middle_index = integers.len() / 2;

  // sort integers in place
  integers.sort();

  let list_on_the_right = integers.split_off(middle_index);

  if integers.len() < list_on_the_right.len() {
    list_on_the_right[0] as f64
  } else {
    (integers[integers.len() - 1] + list_on_the_right[0]) as f64 / 2 as f64
  }
}
