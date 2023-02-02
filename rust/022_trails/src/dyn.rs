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
        let s1 = Struct1 {};
        let s = str_getter(&s1);
        assert_eq!(s, "Struct1");

        let s2 = Struct2 {};
        let s = str_getter(&s2);
        assert_eq!(s, "Struct2");

    }

    trait Food {
        fn get_name(&self) -> &str;
    }

    struct Meat {}
    impl Food for Meat {
        fn get_name(&self) -> &str {
            "Meat"
        }
    }

    struct Vegie {}
    impl Food for Vegie {
        fn get_name(&self) -> &str {
            "Vegie"
        }
    }

    struct Meal {
        foods: Vec<Box<dyn Food>>,
    }

    #[test]
    fn test_vec() {
        let m = Meal {
            foods: vec![Box::new(Meat {}), Box::new(Vegie {})],
        };
        assert_eq!(2, m.foods.len());
        assert_eq!("Meat", m.foods.first().unwrap().get_name());
        assert_eq!("Vegie", m.foods.get(1).unwrap().get_name());
    }
}
