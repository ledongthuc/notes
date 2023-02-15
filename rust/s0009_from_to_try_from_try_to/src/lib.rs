struct A {
    value: usize
}

impl From<B> for A {
    fn from(value: B) -> Self {
        Self { value: value.value }
    }
}

impl Into<B> for A {
    fn into(self) -> B {
        B {
            value: self.value,
        }
    }
}

struct B {
    value: usize
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from() {
        let a = A::from(B{value: 5});
        assert_eq!(a.value, 5);

        let b : B = a.into();
        assert_eq!(b.value, 5);
    }
}
