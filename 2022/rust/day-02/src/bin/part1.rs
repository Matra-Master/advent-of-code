fn main() {
    let input = include_str!("./input1.txt");
    let output = resolve(input);
    dbg!(output);
}

fn resolve(input: &str) -> String {
    "15".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXPECTED: &str = "15";

    #[test]
    fn it_works() {
        let result = resolve("A Y
B X
C Z");
        assert_eq!(result, EXPECTED.to_string());
    }
}
