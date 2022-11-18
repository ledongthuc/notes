#![feature(exclusive_range_pattern)]

#[cfg(test)]
mod tests {

    #[test]
    fn binding_subpattern_with_name() {
        let i = 10;
        let result = match i {
            val @ 0..11 => val * 10,
            _ => -1,
        };
        assert_eq!(100, result);
    }

    #[test]
    fn borrow() {
        let i = 10;
        let result = match i {
            0..10 => -1,
            ref val => {
                // val += 10; // can't += on &{integer}
                val * 10
            }
        };
        assert_eq!(100, result);
    }


        enum Message {
            First { id: i32 },
        }

    #[test]
    fn string() {
        let i = Message::First { id: 5 };
        let result = match i {
            Message::First { id: id_variable @ 4..=6 } => id_variable,
            Message::First { .. } => -1,
        };
        assert_eq!(5, result);
    }
}
