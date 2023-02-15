use std::borrow::Borrow;

struct A {
    value: Value
}

struct Value(usize);

impl Borrow<Value> for A {
    fn borrow(&self) -> &Value {
        &self.value
    }
}

impl ToOwned for Value {
    type Owned = A;

    fn to_owned(&self) -> Self::Owned {
        A{
            value: Value(self.0),
        }
    }
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_owned_a() {
        let v = Value(5);
        let a = v.to_owned();
        assert_eq!(a.value.0, v.0);
    }
}
