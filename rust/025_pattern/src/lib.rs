#![feature(exclusive_range_pattern)]

#[cfg(test)]
mod tests {

    #[test]
    fn multi_pattern() {
        let i = Option::Some(1);
        match i {
            Some(1) | Some(2) => println!("1 or 2"),
            Some(4..=10) => println!("from 4 to 10"),
            None | Some(_) => println!("anything else"),
        };

        let arr = (1,2,3,4,5,6,7);
        let (first, .., last) = arr;
        assert_eq!(first, 1);
        assert_eq!(last, 7);
    }

    #[test]
    fn destructure() {
        struct Point {
            x: i32,
            y: i32,
        }
        let p = Point { x: 0, y: 7 };

        let Point { x: a, y: b } = p;
        assert_eq!(a, 0);
        assert_eq!(b, 7);
    }

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
            Message::First {
                id: id_variable @ 4..=6,
            } => id_variable,
            Message::First { .. } => -1,
        };
        assert_eq!(5, result);
    }

    #[test]
    fn match_guard() {
        for i in 1..10 {
            match Some(i) {
                Some(v) if v % 2 == 0 => println!("Even"),
                Some(_) => println!("Odd"),
                None => println!("None"),
            }
        }
    }

    #[test]
    fn binding() {
        for i in 1..10 {
            match Some(i) {
                Some(inner @ 1..2) => assert!(inner == 1 || inner == 2),
                Some(inner @ 3) => assert_eq!(inner, 3),
                Some(inner) => assert_eq!(inner, i),
                None => {},
            }
        }
    }
}
