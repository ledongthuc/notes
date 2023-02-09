#[cfg(test)]
mod tests {
    use std::ops::{Add, AddAssign, Neg, Not, Shl, ShlAssign, Index};

    struct Test {
        i: i32,
    }

    impl Neg for Test {
        type Output = bool;
        fn neg(self) -> Self::Output {
            self.i > 2
        }
    }

    impl Not for Test {
        type Output = bool;
        fn not(self) -> Self::Output {
            self.i % 2 == 0
        }
    }

    #[test]
    fn unary_operators() {
        let t = Test { i: 1 };
        assert!(-t == false);
        let t = Test { i: 10 };
        assert!(-t);

        let t = Test { i: 3 };
        assert!(!t == false);
        let t = Test { i: 2 };
        assert!(!t);
    }

    struct FakeStringBuilder {
        i: String,
    }

    impl Add<String> for FakeStringBuilder {
        type Output = FakeStringBuilder;

        fn add(mut self, rhs: String) -> Self::Output {
            self.i.push_str(&rhs);
            self
        }
    }

    impl Shl<usize> for FakeStringBuilder {
        type Output = FakeStringBuilder;

        fn shl(mut self, rhs: usize) -> Self::Output {
            let a = "0".repeat(rhs);
            self.i.push_str(a.as_str());
            self
        }
    }

    #[test]
    fn binary_operators() {
        let mut t = FakeStringBuilder {
            i: String::from("hello"),
        };
        t = t + " world!".to_string();
        assert_eq!(t.i, "hello world!");

        let mut t = FakeStringBuilder {
            i: String::from("1001"),
        };
        t = t << 2;
        assert_eq!(t.i, "100100")
    }

    impl AddAssign<String> for FakeStringBuilder {
        fn add_assign(&mut self, rhs: String) {
            self.i.push_str(&rhs);
        }
    }

    impl ShlAssign<usize> for FakeStringBuilder {
        fn shl_assign(&mut self, rhs: usize) {
            let a = "0".repeat(rhs);
            self.i.push_str(a.as_str());
        }
    }

    #[test]
    fn compound_asignment_operators() {
        let mut t = FakeStringBuilder {
            i: String::from("hello"),
        };
        t += " world!".to_string();
        assert_eq!(t.i, "hello world!");

        let mut t = FakeStringBuilder {
            i: String::from("1001"),
        };
        t <<= 2;
        assert_eq!(t.i, "100100")
    }

    impl PartialEq for FakeStringBuilder {
        fn eq(&self, other: &Self) -> bool {
            self.i == other.i
        }
    }

    #[test]
    fn equivalence_operators() {
        let t1 = FakeStringBuilder {
            i: String::from("Dong Thuc Le"),
        };
        let t2 = FakeStringBuilder {
            i: String::from("Dong Thuc Le"),
        };
        let t3 = FakeStringBuilder {
            i: String::from("Dong Thuc Le 3"),
        };

        assert!(t1 == t2);
        assert!(t1 != t3);
    }

    impl PartialOrd<FakeStringBuilder> for FakeStringBuilder {
        fn partial_cmp(&self, other: &FakeStringBuilder) -> Option<std::cmp::Ordering> {
            Some(self.i.cmp(&other.i))
        }
    }

    #[test]
    fn ordering_operators() {
        let t1 = FakeStringBuilder {
            i: String::from("333"),
        };
        let t2 = FakeStringBuilder {
            i: String::from("22"),
        };
        let t3 = FakeStringBuilder {
            i: String::from("333"),
        };

        assert!(t1 > t2);
        assert!(t2 < t3);
        assert!(t1 == t3);
    }

    impl Index<usize> for FakeStringBuilder {
        type Output = str;
        fn index(&self, index: usize) -> &Self::Output {
            self.i.index(0..index)
        }
    }

    #[test]
    fn index_operators() {
        let t = FakeStringBuilder{i: String::from("Hello world!")};
        assert_eq!(&t[5], "Hello");
    }

    // && || .. ..= ?
    // *val val.val2 = Defer/DeferMut
    // fn() Fn, FnMut
}
