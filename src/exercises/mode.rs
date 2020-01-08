use std::collections::HashMap;

pub fn run(integers: Vec<i8>) -> i8 {
  let mut hash = HashMap::new();

  for i in &integers {
      // Avoid write & read warning with cloned()
      let value = hash.get(i).cloned();
      match value {
          Some(n) => hash.insert(i, n + 1),
          None => hash.insert(i, 1)
      };
  }

  let first_key = hash.keys().nth(0).unwrap_or(&&0);
  let first_value = hash.get(first_key).unwrap_or(&0);

  let mut mode: (i8, i8) = (**first_key, *first_value);
  for (key, val) in hash {
      if val > mode.1 {
          mode.0 = *key;
          mode.1 = val;
      }
  }

  mode.0
}
