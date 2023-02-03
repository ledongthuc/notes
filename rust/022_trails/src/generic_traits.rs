mod tests {
    use std::ops::{Add, Mul};

    trait Replacer<'a, T = Self> {
        fn replace(&mut self, another: &'a T);
    }

    struct String<'a> {
        inner: &'a str,
    }

    impl<'a> Replacer<'a> for String<'a> {
        fn replace(&mut self, another: &'a Self) {
            self.inner = another.inner;
        }
    }

    #[test]
    fn test_genertic_trait() {
        let mut a = String { inner: "a" };
        let b = String { inner: "b" };
        a.replace(&b);
        assert_eq!(a.inner, "b");
    }

    fn dot<N>(v1: &[N], v2: &[N]) -> N 
        where N: Add<Output = N> + Mul<Output = N> + Default + Copy
    {
        let mut total = N::default();
        for i in 0..v1.len() {
            total = total + v1[i] + v2[i]
        }
        total
    }

    #[test]
    fn test_dot() {
        let total = dot(&[1,2,3], &[4,5,6]);
        assert_eq!(total, 21);

        let total = dot(&[1.0,2.0,3.0], &[4.0,5.0,6.0]);
        assert_eq!(total, 21.0);
    }
}
