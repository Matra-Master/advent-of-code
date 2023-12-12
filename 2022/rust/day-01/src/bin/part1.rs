fn main() {
    let input = include_str!("./input1.txt");
    let output = resolve(input);
    dbg!(output);
}

fn resolve(input: &str) -> String {
    let result = input
        .split("\n\n")
        .map(|elf_load| {
            elf_load
                .split("\n")
                .map(|item| 
                    item.parse::<u32>().unwrap_or(0)
                )
                .sum::<u32>()
        })
        .max()
        .unwrap();
    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = resolve(
            "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000",
        );
        assert_eq!(result, "24000".to_string());
    }
}
