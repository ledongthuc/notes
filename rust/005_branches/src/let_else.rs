mod tests {

    #[test]
    fn let_else() {
        fn get_value(i: i32) -> Option<i32> {
            Some(i % 2)
        }

        let Some(v) = get_value(10) else {
            panic!("Not Some()");
        };
        assert_eq!(0, v);

        let Some(_) = get_value(11) else {
            return;
        };
    }
}
