pub fn adds_one(number: i32) -> i32 {
    number + 1
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_adds_one() {
        assert_eq!(2, adds_one(1));
    }
}