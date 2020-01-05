mod exercises;
use exercises::mean;
use exercises::median;

fn main() {
  println!("Mean = {}", mean::run(vec![3, 1, 4, 2, 4]));
  println!("Median = {}", median::run(vec![2, 4, 1, 5, 3, 5]));
}
