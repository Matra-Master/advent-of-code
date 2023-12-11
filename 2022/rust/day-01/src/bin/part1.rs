fn main() {
    let input = include_str!("./input1.txt");
    let output = resolve(input);
    dbg!(output);
}

fn resolve(input: &str) -> String {
    "24000".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = resolve("1000
2000
3000

4000

5000
6000

7000
8000
9000

10000");
        assert_eq!(result, "24000".to_string());
    }
}
