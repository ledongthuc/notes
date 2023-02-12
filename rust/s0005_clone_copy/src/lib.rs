struct Test {
    dirty_ptr: *const i32,
    ptr_counter: i32,
}

// TODO: not real clone dirty ptr with counter. Just for test the cloning
impl Clone for Test {
    fn clone(&self) -> Self {
        Self{
            dirty_ptr: self.dirty_ptr,
            ptr_counter: self.ptr_counter+1,
        }

    }
}

impl Copy for Test {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clone() {
        let test = Test{
            dirty_ptr: &1,
            ptr_counter: 1,
        };

        let test2 = test.clone();
        assert_eq!(test.ptr_counter, 1);
        assert_eq!(test2.ptr_counter, 2);
    }

    #[test]
    fn test_copy() {
        let test = Test{
            dirty_ptr: &1,
            ptr_counter: 1,
        };

        // byte to byte copy
        let test2 = test;
        assert_eq!(test.ptr_counter, 1);
        assert_eq!(test2.ptr_counter, 1);
    }
}
