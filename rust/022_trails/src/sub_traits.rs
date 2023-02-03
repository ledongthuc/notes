mod tests {

    trait Compare: Debug {
        // short hand of "Compare where Self: Debug"
        fn compare(&self, _: &Self) -> bool;
    }

    trait Debug {
        fn print(&self);
    }

    struct S<'a> {
        v: &'a str,
    }

    impl<'a> Compare for S<'a> {
        fn compare(&self, another: &Self) -> bool {
            self.v == another.v
        }
    }

    impl<'a> Debug for S<'a> {
        fn print(&self) {
            println!("{}", self.v);
        }
    }

    #[test]
    fn test_vec() {
        let s1 = S { v: "haha" };
        s1.print();
    }
}
