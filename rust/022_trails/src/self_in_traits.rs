mod tests {

    trait Compare {
        fn compare(&self, _: &Self) -> bool;
    }

    struct S<'a> {
        v: &'a str,
    }

    impl<'a> Compare for S<'a> {
        fn compare(&self, another: &Self) -> bool {
            self.v == another.v
        }
    }

    #[test]
    fn test_vec() {
        let s1 = S { v: "haha" };
        let s2 = S { v: "haha" };
        assert!(s1.compare(&s2));

        let s2 = S { v: "huhu" };
        assert!(!s1.compare(&s2))
    }
}
