#[cfg(test)]
mod test {
    use logo_lib::adds_one;

    #[test]
    fn test_adds_one() {
        assert_eq!(2, adds_one(1));
    }
}
