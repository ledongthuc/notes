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
}
