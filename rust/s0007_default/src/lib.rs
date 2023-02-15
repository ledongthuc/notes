struct You {
    value: i32,
}

impl Default for You {
    fn default() -> Self {
        Self { value: 10 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let d = You{..Default::default()};
        assert_eq!(d.value, 10)
    }
}
