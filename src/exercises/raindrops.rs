// Instruction: https://exercism.io/my/solutions/ea68b7c37fce4621bdae551ca1bb3ac2
pub fn run(n: u32) -> String {
    let mut result = String::from("");

    if n % 3 != 0 && n % 5 != 0 && n % 7 != 0 {
        result.push_str(&n.to_string());
        return result;
    }

    if n % 3 == 0 {
        result.push_str("Pling");
    };
    if n % 5 == 0 {
        result.push_str("Plang");
    };
    if n % 7 == 0 {
        result.push_str("Plong");
    };

    result
}
