fn main() {
    let input = "sarasa";
    let output = resolve(input);
    dbg!(output);
}

fn resolve(input: &str) -> String {
    "todo!()".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = resolve("");
        assert_eq!(result, "4".to_string());
    }
}
