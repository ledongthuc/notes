#![feature(vec_into_raw_parts)]

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
        let layout = Layout::array::<u16>(6);
        let ptr = unsafe { alloc(layout.unwrap()) };
        assert!(!ptr.is_null());
        let ptr = ptr as *mut u16;
        unsafe { ptr::write(ptr, 15) };
        unsafe { ptr::write(ptr.add(1), 14) };
        unsafe { ptr::write(ptr.add(2), 13) };
        unsafe { ptr::write(ptr.add(3), 12) };
        unsafe { ptr::write(ptr.add(4), 11) };
        unsafe { ptr::write(ptr.add(5), 10) };

        let vec = unsafe { Vec::from_raw_parts(ptr, 5, 6) };
        assert_eq!(vec[0], 15);
        assert_eq!(vec[1], 14);
        assert_eq!(vec[2], 13);
        assert_eq!(vec[3], 12);
        assert_eq!(vec[4], 11);

        let (ptr2, len2, cap2) = vec.into_raw_parts();
        assert_eq!(ptr, ptr2);
        assert_eq!(5, len2);
        assert_eq!(6, cap2);
    }

    #[test]
    fn vec_reserve_and_reserve_exact() {
        let mut vec = vec![1,2,3,4,5];
        assert_eq!(5, vec.capacity());
        assert_eq!(5, vec.len());
        vec.reserve(3);
        assert_eq!(vec.capacity(), 10);

        let mut vec = vec![1,2,3,4,5];
        assert_eq!(5, vec.len());
        vec.reserve_exact(3);
        assert_eq!(vec.capacity(), 8);
    }

    #[test]
    fn vec_shrink_to_fit() {
        let mut vec = Vec::with_capacity(10);
        vec.extend([1,2,3]);
        vec.shrink_to_fit();
        assert_eq!(vec.capacity(), 3);
    }

    #[test]
    fn vec_shrink_to() {
        let mut vec = Vec::with_capacity(10);
        vec.extend([1,2,3]);

        vec.shrink_to(4);
        assert_eq!(vec.capacity(), 4);

        vec.shrink_to(1);
        assert_eq!(vec.capacity(), 3);
    }

    #[test]
    fn vec_into_boxed_slice() {
        let mut vec = Vec::with_capacity(10);
        vec.extend([1,2,3]);
        assert_eq!(10, vec.capacity());

        let slice = vec.into_boxed_slice();
        assert_eq!(3, slice.into_vec().capacity())
    }

    #[test]
    fn vec_truncate() {
        let mut vec = vec![1,2,3,4,5,6];
        vec.truncate(3);
        assert_eq!(vec![1,2,3], vec);

        let ptr = vec.as_ptr();
        assert_eq!(&1, unsafe { &*ptr as &i32 });
        assert_eq!(&2, unsafe { &*ptr.add(1) as &i32 });
        assert_eq!(&3, unsafe { &*ptr.add(2) as &i32 });
        assert_eq!(&4, unsafe { &*ptr.add(3) as &i32 }); // unsafe read, unexpected area
        assert_eq!(&5, unsafe { &*ptr.add(4) as &i32 }); // unsafe read, unexpected area
        assert_eq!(&6, unsafe { &*ptr.add(5) as &i32 }); // unsafe read, unexpected area
    }

    #[test]
    fn vec_remove() {
        let mut vec = vec![1,2,3,4,5,6];

        let removed_item = vec.swap_remove(3);
        assert_eq!(4, removed_item);

        assert_eq!(vec![1,2,3,6,5], vec);
    }

    #[test]
    fn vec_swap_remove() {
        let mut vec = vec![1,2,3,4,5,6];

        let removed_item = vec.remove(3);
        assert_eq!(4, removed_item);

        assert_eq!(vec![1,2,3,5,6], vec);
    }

    #[test]
    fn vec_retain() {
        let mut vec = vec![1,2,3,4,5,6];
        vec.retain(|x| x % 2 == 0);
        assert_eq!(vec![2,4,6], vec);
    }

    #[test]
    fn vec_dedup_by_key() {
        let mut vec = vec![10,11,12,22,23,30,40];
        vec.dedup_by_key(|x| *x/10);
        assert_eq!(vec![10, 22, 30, 40], vec);
    }

    #[test]
    fn vec_dedup_by() {
        let mut vec = vec![10,11,12,22,23,30,40];
        vec.dedup_by(|x, previous| *x/10 == *previous/10);
        assert_eq!(vec![10, 22, 30, 40], vec);
    }
}
