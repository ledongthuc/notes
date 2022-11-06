mod tests {
    //***************
    // GAT example lifetime
    //***************
    trait Boxer {
        type Item<'a>
        where
            Self: 'a;
        fn unbox(&self) -> Self::Item<'_>;
    }

    struct MyBoxer<'t> {
        internal: &'t i32,
    }

    impl<'t> Boxer for MyBoxer<'t> {
        type Item<'a> = &'a i32 where Self: 'a;

        fn unbox(&self) -> Self::Item<'_> {
            self.internal
        }
    }

    #[test]
    fn gat_lifetime() {
        let val: i32 = 1;
        let ref_val;
        {
            let b = MyBoxer { internal: &val };
            ref_val = b.unbox();
            assert_eq!(&1, ref_val);
        }
        // assert_eq!(&1, ref_val); //Failed because ref_val is bind to lifetime of MyBoxer
    }

    //***************
    // GAT example type
    //***************
    trait Boxer2 {
        type Item<T>;
        fn make_box<T>(&self, _: T) -> Self::Item<T>;
    }

    struct MyBoxer2 {}

    impl Boxer2 for MyBoxer2 {
        type Item<T> = Box<T>;

        fn make_box<T>(&self, x: T) -> Self::Item<T> {
            Box::new(x)
        }
    }

    #[test]
    fn gat_type() {
        let val: i32 = 1;
        let b_val;
        {
            let b = MyBoxer2 {};
            b_val = b.make_box(val);
            assert_eq!(Box::new(1), b_val);
        }
        assert_eq!(Box::new(1), b_val);
    }

    //***************
    // GAT example type
    //***************
    trait Boxer3 {
        type Item<const N: usize>;

        fn make_empty_array<const N: usize>(&self) -> Self::Item<{ N }>;
    }

    struct MyBoxer3 {}

    impl Boxer3 for MyBoxer3 {
        type Item<const N: usize> = [i32; N];

        fn make_empty_array<const N: usize>(&self) -> Self::Item<{ N }> {
            [i32::default(); N]
        }
    }

    #[test]
    fn gat_const() {
        let b = MyBoxer3 {};
        let val: [i32; 5] = b.make_empty_array();
        assert_eq!(5, val.len());
    }
}
