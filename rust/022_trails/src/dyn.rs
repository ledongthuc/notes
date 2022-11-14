mod tests {
    trait Trait1 {
        fn to_string(&self) -> &str;
    }

    struct Struct1 {}
    impl Trait1 for Struct1 {
        fn to_string(&self) -> &str {
            "Struct1"
        }
    }

    struct Struct2 {}
    impl Trait1 for Struct2 {
        fn to_string(&self) -> &str {
            "Struct2"
        }
    }

    fn str_getter(printer: &dyn Trait1) -> &str {
        printer.to_string()
    }

    #[test]
    fn try_dyn() {
        let s1 = Struct1{};
        let s = str_getter(&s1);
        assert_eq!(s, "Struct1")
    }
}
