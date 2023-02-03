mod tests {

    trait Cutter {
        fn half_len(&self) -> f64;
    }

    impl Cutter for str {
        fn half_len(&self) -> f64 {
            self.len() as f64 / 2.0
        }
    }

    #[test]
    fn test_trait_for_type_str() {
        let name = "hello world.This is the text";
        assert_eq!(28.0 / 2.0, name.half_len())
    }

    trait HelloMe {
        fn to_string_me(&self) -> String;
    }

    impl<T: ToString> HelloMe for T {
        fn to_string_me(&self) -> String {
            let mut s = self.to_string();
            s.push_str(". Hello Me");
            s
        }
    }

    #[test]
    fn test_trait_for_trait() {
        let name = "hello world.This is the text";
        assert_eq!(28.0 / 2.0, name.half_len())
    }
}
