pub fn run(word: &str) -> String {
  let vowels = "aeiou";
  let first_char = word.get(0..1).unwrap();
  let sliced_word = word.get(1..word.len()).unwrap();

  if vowels.contains(first_char) {
    word.to_owned() + "-hay"
  } else {
    sliced_word.to_owned() + "-" + first_char + "ay"
  }
}
