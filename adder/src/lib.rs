#[cfg(test)]
mod tests {
    #[test]
    fn add_two() {
        assert_eq!(super::add_two(2), 4, "add_two != 4");
    }
}

pub fn add_two(input: i32) -> i32 {
    add_two_priv(input)
}

fn add_two_priv(input: i32) -> i32 {
    input + 2
}