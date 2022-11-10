#![feature(exclusive_range_pattern)]

#[cfg(test)]
mod tests {

    #[test]
    fn binding_subpattern_with_name() {
        let i = 10;
        let result = match i {
            val @ 0 .. 11 => val * 10,
            _ => -1,
        };
        assert_eq!(100, result);
    }

    #[test]
    fn borrow() {
        let i = 10;
        let result = match i {
            0 .. 10 => -1,
            ref val => {
                // val += 10; // can't += on &{integer}
                val * 10
            },
        };
        assert_eq!(100, result);
    }

    #[test]
    fn string() {
        let i = String::from("Hello");
        let result = match i {
            v @ String("Bye") => -1,
            ref val => {
                // val += 10; // can't += on &{integer}
                val * 10
            },
        };
        assert_eq!(100, result);
    }
}
