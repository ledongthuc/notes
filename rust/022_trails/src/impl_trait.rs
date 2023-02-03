mod tests {
    use std::fmt::Display;


    trait Shape {
        type Item<'a>: Display where Self: 'a;
        fn new() -> Self;
        fn to_string(&self) -> Self::Item<'_>;
    }

    struct Square <'a> {
        name: &'a str
    }

    impl<'t> Shape for Square <'t> {
        type Item<'a> = &'a str where Self: 'a;
        fn new() -> Self {
            Square { name: "square"}
        }
        fn to_string(&self) -> Self::Item<'t> {
           self.name
        }
    }

    struct Rectangle <'a> {
        name: &'a str
    }

    impl<'t> Shape for Rectangle <'t> {
        type Item<'a> = &'a str where Self: 'a;
        fn new() -> Self {
            Rectangle { name: "rectangle"}
        }
        fn to_string(&self) -> Self::Item<'t> {
           self.name
        }
    }

    fn make_square() -> impl Shape {
        Square{name:"square"}
    }

    fn make_rectangle() -> impl Shape {
        Square{name:"rectangle"}
    }

    fn get_string(shape_name: impl Display) {
        println!("{}", shape_name);
    }

    #[test]
    fn test_impl_trait() {
        let s = make_square();
        get_string(s.to_string());

        let r = make_rectangle();
        get_string(r.to_string());
    }
}
