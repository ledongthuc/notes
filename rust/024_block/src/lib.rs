#[cfg(test)]
mod tests {
    #[test]
    fn label_block() {
        let result = 'block: {
            for i in 0..20 {
                if i == 15 {
                    break 'block i;
                }
            }
            break 'block -1;
        };

        assert_eq!(15, result);
    }
}
