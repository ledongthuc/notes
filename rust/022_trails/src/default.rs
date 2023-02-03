mod tests {

    trait DefaultTraits {
        fn get_me(&self) -> &str;

        fn print_me(&self) {
            print!("Hello {}", self.get_me());
        }
    }

    struct S {}

    impl DefaultTraits for S {
        fn get_me(&self) -> &str {
            "MR.S"
        }
    }

    #[test]
    fn test_vec() {
        let s = S {};
        assert_eq!("MR.S", s.get_me());
    }
}
