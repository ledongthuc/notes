mod tests {

    trait StringHolder<'a> {
        fn new() -> Self;
        fn add(&mut self, string: &'a str);
    }

    struct Name<'a> {
        inner: &'a str,
    }

    impl<'a> StringHolder<'a> for Name<'a> {
        fn new() -> Self {
            Name { inner: "" }
        }
        fn add(&mut self, string: &'a str) {
            self.inner = string
        }
    }

    #[test]
    fn test_vec() {
        let mut a = Name::new();
        {
            a.add("Test");
        }
        assert_eq!(a.inner, "Test")
    }
}
