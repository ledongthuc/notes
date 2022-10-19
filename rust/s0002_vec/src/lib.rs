#[cfg(test)]
mod tests {
    #[test]
    fn basic_vec() {
        let mut vec = vec![1,2,3];
        assert_eq!(3, vec.len());
        let old_cap = vec.capacity();
        assert_eq!(3, vec.capacity());

        vec.push(4);
        assert_eq!(4, vec.len());
        assert_eq!(old_cap * 2, vec.capacity());
    }

    #[test]
    fn vec_new() {
        let vec: Vec<i32> = Vec::new();
        assert_eq!(0, vec.len());
        assert_eq!(0, vec.capacity());
    }

    #[test]
    fn vec_with_capacity() {
        let vec: Vec<i32> = Vec::with_capacity(100);
        assert_eq!(0, vec.len());
        assert_eq!(100, vec.capacity());

        let vec: Vec<i32> = Vec::with_capacity(0); // no allocating heap space
        assert_eq!(0, vec.capacity());

        let vec: Vec<()> = Vec::with_capacity(100);
        assert_eq!(usize::MAX, vec.capacity());
    }
}
