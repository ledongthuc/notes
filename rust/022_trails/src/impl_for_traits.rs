mod tests {
    trait TestTraits {
        fn hohoho(&self) -> &str {
            "Ho ho ho!"
        }
    }

    use std::clone::Clone;

    // For every type T that's from Clone, implment Test traits
    impl<T: Clone> TestTraits for T {}

    #[test]
    fn test_string() {
        let santa = String::from("Santa");
        assert_eq!(santa.hohoho(), "Ho ho ho!")
    }
}
