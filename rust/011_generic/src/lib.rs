mod tests {

    struct Point<T> {
        x: T,
        y: T,
    }

    impl<T> Point<T> {
        fn x(&self) -> &T {
            &self.x
        }
    }

    #[test]
    fn test_generic() {
        let integer = Point { x: 5, y: 10 };
        assert_eq!(*integer.x(), 5);

        let float = Point { x: 1.0, y: 2.0 };
        assert_eq!(*float.x(), 1.0);
    }

    use std::fmt::Debug;
    use std::hash::Hash;

    fn print_duplicated_hash<T: Debug + Hash + Eq>(_values: &Vec<T>) {}

    fn do_something<M, R>(map: M, reduce: R)
    where
        M: Debug + Eq,
        R: Hash + Eq,
    {
    }

    fn generate_vec<const N: usize, T>(a: [T; N]) -> Vec<T> {
        let mut vec = Vec::new();
        for i in a {
            vec.push(i);
        }
        vec
    }
    #[test]
    fn test_const_generic() {
        let v = generate_vec([1,2,3]);
        assert_eq!(v, vec![1,2,3]);
    }
}
