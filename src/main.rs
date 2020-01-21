mod exercises;
use exercises::mean;
use exercises::median;
use exercises::mode;
use exercises::piglatin;

fn main() {
  println!("Mean = {}", mean::run(vec![3, 1, 4, 2, 4]));
  println!("Median = {}", median::run(vec![2, 4, 1, 5, 3, 5]));
  println!("Mode = {}", mode::run(vec![2, 5, 1, 3, 3, 3, 1, 5]));
  println!("Piglatin = {}", piglatin::run("apple"));
}
