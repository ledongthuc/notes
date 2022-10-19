#[cfg(test)]
mod tests {
    use core::ptr;
    use std::alloc::{alloc, Layout};

    #[test]
    fn basic_vec() {
        let mut vec = vec![1, 2, 3];
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

    #[test]
    fn vec_from_raw_parts() {
        let layout = Layout::array::<u16>(5);
        let ptr = unsafe { alloc(layout.unwrap()) };
        assert!(!ptr.is_null());
        let ptr = ptr as *mut u16;
        unsafe { ptr::write(ptr, 15) };
        unsafe { ptr::write(ptr.add(1), 14) };
        unsafe { ptr::write(ptr.add(2), 13) };
        unsafe { ptr::write(ptr.add(3), 12) };
        unsafe { ptr::write(ptr.add(4), 11) };
        unsafe { ptr::write(ptr.add(5), 10) };

        let vec = unsafe { Vec::from_raw_parts(ptr, 5, 5) };
        assert_eq!(vec[0], 15);
        assert_eq!(vec[1], 14);
        assert_eq!(vec[2], 13);
        assert_eq!(vec[3], 12);
        assert_eq!(vec[4], 11);
    }
}
