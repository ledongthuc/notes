use std::ops::Deref;

struct Wrapper<T> {
    value: T,
}

impl<T> Deref for Wrapper<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

struct Inner {}

impl Inner {
    fn get_value(&self) -> &'static str {
        "world"
    }
}

struct Single {
    value: String
}

impl Deref for Single {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wrapper() {
        let w = Wrapper::<String> {
            value: "Hello".to_string(),
        };
        assert_eq!(*w, "Hello".to_string());

        let w = Wrapper::<Inner> {
            value: Inner {  },
        };
        assert_eq!(w.get_value(), "world");
    }
}
