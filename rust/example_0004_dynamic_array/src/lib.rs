use std::alloc::{alloc, dealloc, Layout};

pub struct DynamicArray {
    data: *mut u64,  // Pointer to the array
    capacity: usize, // Total allocated capacity
    size: usize,     // Current number of elements
}

impl DynamicArray {
    /// Initialize a DynamicArray with specified size and capacity
    /// 
    /// # Arguments
    /// * `size` - The initial number of elements (must be <= capacity)
    /// * `capacity` - The total capacity to allocate
    /// 
    /// # Returns
    /// * `Result<DynamicArray, &'static str>` - The initialized DynamicArray or an error message
    pub fn new(size: usize, capacity: usize) -> Result<Self, &'static str> {
        // Validate inputs
        if size > capacity {
            return Err("Size cannot be greater than capacity");
        }
        
        if capacity == 0 {
            return Err("Capacity must be greater than 0");
        }
        
        // Allocate memory
        let layout = Layout::array::<u64>(capacity).map_err(|_| "Failed to create layout")?;
        let ptr = unsafe { alloc(layout) as *mut u64 };
        
        if ptr.is_null() {
            return Err("Failed to allocate memory");
        }
        
        // Initialize all elements to 0
        unsafe {
            for i in 0..capacity {
                *ptr.add(i) = 0;
            }
        }
        
        Ok(DynamicArray {
            data: ptr,
            capacity,
            size,
        })
    }
    
    /// Get the current size of the array
    pub fn size(&self) -> usize {
        self.size
    }
    
    /// Get the capacity of the array
    pub fn capacity(&self) -> usize {
        self.capacity
    }
    
    /// Get a reference to an element at the given index
    pub fn get(&self, index: usize) -> Option<&u64> {
        if index >= self.size {
            None
        } else {
            unsafe { Some(&*self.data.add(index)) }
        }
    }
    
    /// Resize the array by doubling its capacity
    /// Creates a new array with double capacity, copies old data, and updates the pointer
    pub fn resize_array(&mut self) -> Result<(), &'static str> {
        let new_capacity = self.capacity * 2;
        
        // Create new layout for the doubled capacity
        let new_layout = Layout::array::<u64>(new_capacity).map_err(|_| "Failed to create new layout")?;
        let new_ptr = unsafe { alloc(new_layout) as *mut u64 };
        
        if new_ptr.is_null() {
            return Err("Failed to allocate new memory");
        }
        
        // Copy old data to new array
        unsafe {
            for i in 0..self.size {
                *new_ptr.add(i) = *self.data.add(i);
            }
            
            // Initialize remaining elements to 0
            for i in self.size..new_capacity {
                *new_ptr.add(i) = 0;
            }
        }
        
        // Deallocate old memory
        if !self.data.is_null() {
            let old_layout = Layout::array::<u64>(self.capacity).unwrap();
            unsafe { dealloc(self.data as *mut u8, old_layout) };
        }
        
        // Update the struct fields
        self.data = new_ptr;
        self.capacity = new_capacity;
        
        Ok(())
    }
    
    /// Shrink the array by halving its capacity if possible
    /// Capacity cannot be less than the current size
    pub fn shrink_array(&mut self) -> Result<(), &'static str> {
        let new_capacity = self.capacity / 2;
        
        // Check if shrinking is possible
        if new_capacity < self.size {
            return Err("Cannot shrink: new capacity would be less than current size");
        }
        
        if new_capacity == 0 {
            return Err("Cannot shrink: capacity would become 0");
        }
        
        // Create new layout for the halved capacity
        let new_layout = Layout::array::<u64>(new_capacity).map_err(|_| "Failed to create new layout")?;
        let new_ptr = unsafe { alloc(new_layout) as *mut u64 };
        
        if new_ptr.is_null() {
            return Err("Failed to allocate new memory");
        }
        
        // Copy old data to new array (only up to size)
        unsafe {
            for i in 0..self.size {
                *new_ptr.add(i) = *self.data.add(i);
            }
        }
        
        // Deallocate old memory
        if !self.data.is_null() {
            let old_layout = Layout::array::<u64>(self.capacity).unwrap();
            unsafe { dealloc(self.data as *mut u8, old_layout) };
        }
        
        // Update the struct fields
        self.data = new_ptr;
        self.capacity = new_capacity;
        
        Ok(())
    }

    /// Push a value to the end of the array, resizing if necessary
    /// 
    /// # Arguments
    /// * `value` - The value to insert
    /// 
    /// # Returns
    /// * `Result<(), &'static str>` - Success or an error message
    /// 
    pub fn push(&mut self, value: u64) -> Result<(), &'static str> {
        if self.size == self.capacity {
            self.resize_array()?;
        }

        unsafe {
            *self.data.add(self.size) = value;
        }

        self.size += 1;

        Ok(())
    }

    /// Remove and return the last element from the array
    /// 
    /// # Returns
    /// * `Result<u64, &'static str>` - The popped value or an error message if the array is empty
    /// 
    pub fn pop(&mut self) -> Result<u64, &'static str> {
        if self.size == 0 {
            return Err("Cannot pop from an empty array");
        }
        
        let value = unsafe { *self.data.add(self.size - 1) };
        self.size -= 1;

        // Try to shrink if array is significantly underutilized
        // If shrinking fails, continue normally - this is not critical
        if self.size > 0 && self.size <= self.capacity / 4 && self.capacity > 4 {
            let _ = self.shrink_array();
        }

        Ok(value)
    }

    /// Insert a value at the specified index, shifting existing elements to the right
    /// 
    /// # Arguments
    /// * `index` - The position to insert the value (0 <= index <= size)
    /// * `value` - The value to insert
    /// 
    /// # Returns
    /// * `Result<(), &'static str>` - Success or an error message
    /// 
    pub fn insert(&mut self, index: usize, value: u64) -> Result<(), &'static str> {
        if index > self.size {
            return Err("Index out of bounds: cannot insert beyond current size");
        }

        // Resize if necessary
        if self.size == self.capacity {
            self.resize_array()?;
        }

        // Shift elements to the right to make room for the new element
        unsafe {
            for i in (index..self.size).rev() {
                *self.data.add(i + 1) = *self.data.add(i);
            }
            *self.data.add(index) = value;
        }

        self.size += 1;
        Ok(())
    }

    /// Delete the element at the specified index, shifting remaining elements to the left
    /// 
    /// # Arguments
    /// * `index` - The position of the element to delete (0 <= index < size)
    /// 
    /// # Returns
    /// * `Result<u64, &'static str>` - The deleted value or an error message
    /// 
    pub fn delete(&mut self, index: usize) -> Result<u64, &'static str> {
        if self.size == 0 {
            return Err("Cannot delete from an empty array");
        }

        if index >= self.size {
            return Err("Index out of bounds: cannot delete beyond current size");
        }

        // Get the value to be deleted
        let value = unsafe { *self.data.add(index) };

        // Shift elements to the left to fill the gap
        unsafe {
            for i in index..(self.size - 1) {
                *self.data.add(i) = *self.data.add(i + 1);
            }
        }

        self.size -= 1;

        // Try to shrink if array is significantly underutilized
        // If shrinking fails, continue normally - this is not critical
        if self.size > 0 && self.size <= self.capacity / 4 && self.capacity > 4 {
            let _ = self.shrink_array();
        }

        Ok(value)
    }
}

impl Drop for DynamicArray {
    fn drop(&mut self) {
        if !self.data.is_null() {
            let layout = Layout::array::<u64>(self.capacity).unwrap();
            unsafe { dealloc(self.data as *mut u8, layout) };
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let size = 5;
        let capacity = 10;
        
        let dynamic_array = DynamicArray::new(size, capacity).unwrap();
        
        assert_eq!(dynamic_array.size(), size);
        assert_eq!(dynamic_array.capacity(), capacity);
        assert_eq!(dynamic_array.get(0), Some(&0));
        assert_eq!(dynamic_array.get(1), Some(&0));
        assert_eq!(dynamic_array.get(2), Some(&0));
        assert_eq!(dynamic_array.get(3), Some(&0));
        assert_eq!(dynamic_array.get(4), Some(&0));
        assert_eq!(dynamic_array.get(5), None); // Beyond size
    }
    
    #[test]
    fn test_new_validation() {
        // Test size > capacity
        assert!(DynamicArray::new(5, 3).is_err());
        
        // Test capacity = 0
        assert!(DynamicArray::new(0, 0).is_err());
    }
    
    #[test]
    fn test_resize_array() {
        let mut dynamic_array = DynamicArray::new(3, 5).unwrap();
        
        // Set some values
        unsafe {
            *dynamic_array.data.add(0) = 10;
            *dynamic_array.data.add(1) = 20;
            *dynamic_array.data.add(2) = 30;
        }
        
        // Verify initial state
        assert_eq!(dynamic_array.size(), 3);
        assert_eq!(dynamic_array.capacity(), 5);
        assert_eq!(dynamic_array.get(0), Some(&10));
        assert_eq!(dynamic_array.get(1), Some(&20));
        assert_eq!(dynamic_array.get(2), Some(&30));
        
        // Resize the array
        dynamic_array.resize_array().unwrap();
        
        // Verify doubled capacity
        assert_eq!(dynamic_array.size(), 3);
        assert_eq!(dynamic_array.capacity(), 10);
        
        // Verify data is preserved
        assert_eq!(dynamic_array.get(0), Some(&10));
        assert_eq!(dynamic_array.get(1), Some(&20));
        assert_eq!(dynamic_array.get(2), Some(&30));
        
        // Verify elements beyond size are None
        assert_eq!(dynamic_array.get(3), None); // Beyond size
        assert_eq!(dynamic_array.get(4), None);
        assert_eq!(dynamic_array.get(9), None); // Last element in capacity
        assert_eq!(dynamic_array.get(10), None); // Beyond capacity
    }
    
    #[test]
    fn test_shrink_array() {
        let mut dynamic_array = DynamicArray::new(3, 10).unwrap();
        
        // Set some values
        unsafe {
            *dynamic_array.data.add(0) = 10;
            *dynamic_array.data.add(1) = 20;
            *dynamic_array.data.add(2) = 30;
        }
        
        // Verify initial state
        assert_eq!(dynamic_array.size(), 3);
        assert_eq!(dynamic_array.capacity(), 10);
        assert_eq!(dynamic_array.get(0), Some(&10));
        assert_eq!(dynamic_array.get(1), Some(&20));
        assert_eq!(dynamic_array.get(2), Some(&30));
        
        // Shrink the array
        dynamic_array.shrink_array().unwrap();
        
        // Verify halved capacity
        assert_eq!(dynamic_array.size(), 3);
        assert_eq!(dynamic_array.capacity(), 5);
        
        // Verify data is preserved
        assert_eq!(dynamic_array.get(0), Some(&10));
        assert_eq!(dynamic_array.get(1), Some(&20));
        assert_eq!(dynamic_array.get(2), Some(&30));
        assert_eq!(dynamic_array.get(3), None); // Beyond size
        assert_eq!(dynamic_array.get(4), None); // Beyond size
    }
    
    #[test]
    fn test_shrink_array_validation() {
        let mut dynamic_array = DynamicArray::new(5, 6).unwrap();
        
        // Try to shrink when size is close to capacity (should fail)
        assert!(dynamic_array.shrink_array().is_err());
        
        let mut small_array = DynamicArray::new(1, 2).unwrap();
        
        // Try to shrink when capacity would become 0 (should fail)
        assert!(small_array.shrink_array().is_ok());
        assert_eq!(small_array.capacity(), 1);
        assert_eq!(small_array.size(), 1);
        assert_eq!(small_array.get(0), Some(&0));
        assert_eq!(small_array.get(1), None);
    }
    
    #[test]
    fn test_push_basic() {
        let mut array = DynamicArray::new(0, 5).unwrap();
        
        // Push a single value
        array.push(42).unwrap();
        assert_eq!(array.size(), 1);
        assert_eq!(array.capacity(), 5);
        assert_eq!(array.get(0), Some(&42));
        
        // Push another value
        array.push(100).unwrap();
        assert_eq!(array.size(), 2);
        assert_eq!(array.capacity(), 5);
        assert_eq!(array.get(0), Some(&42));
        assert_eq!(array.get(1), Some(&100));
    }
    
    #[test]
    fn test_push_with_resize() {
        let mut array = DynamicArray::new(0, 2).unwrap();
        
        // Fill the array to capacity
        array.push(10).unwrap();
        array.push(20).unwrap();
        assert_eq!(array.size(), 2);
        assert_eq!(array.capacity(), 2);
        
        // Push one more - should trigger resize
        array.push(30).unwrap();
        assert_eq!(array.size(), 3);
        assert_eq!(array.capacity(), 4); // Doubled from 2
        
        // Verify all values are preserved
        assert_eq!(array.get(0), Some(&10));
        assert_eq!(array.get(1), Some(&20));
        assert_eq!(array.get(2), Some(&30));
    }
    
    #[test]
    fn test_push_multiple_resizes() {
        let mut array = DynamicArray::new(0, 1).unwrap();
        
        // Push values that will trigger multiple resizes
        for i in 0..10 {
            array.push(i as u64).unwrap();
        }
        
        assert_eq!(array.size(), 10);
        assert_eq!(array.capacity(), 16); // 1 -> 2 -> 4 -> 8 -> 16
        
        // Verify all values are correct
        for i in 0..10 {
            assert_eq!(array.get(i), Some(&(i as u64)));
        }
    }
    
    #[test]
    fn test_push_large_values() {
        let mut array = DynamicArray::new(0, 3).unwrap();
        
        // Push some large values
        array.push(u64::MAX).unwrap();
        array.push(0).unwrap();
        array.push(123456789).unwrap();
        
        assert_eq!(array.size(), 3);
        assert_eq!(array.get(0), Some(&u64::MAX));
        assert_eq!(array.get(1), Some(&0));
        assert_eq!(array.get(2), Some(&123456789));
    }
    
    #[test]
    fn test_push_after_manual_resize() {
        let mut array = DynamicArray::new(2, 3).unwrap();
        
        // Set initial values
        unsafe {
            *array.data.add(0) = 100;
            *array.data.add(1) = 200;
        }
        
        // Push a value
        array.push(300).unwrap();
        assert_eq!(array.size(), 3);
        assert_eq!(array.capacity(), 3);
        
        // Push another - should trigger resize
        array.push(400).unwrap();
        assert_eq!(array.size(), 4);
        assert_eq!(array.capacity(), 6); // Doubled from 3
        
        // Verify all values
        assert_eq!(array.get(0), Some(&100));
        assert_eq!(array.get(1), Some(&200));
        assert_eq!(array.get(2), Some(&300));
        assert_eq!(array.get(3), Some(&400));
    }
    
    #[test]
    fn test_push_edge_cases() {
        let mut array = DynamicArray::new(0, 1).unwrap();
        
        // Push to exactly capacity
        array.push(1).unwrap();
        assert_eq!(array.size(), 1);
        assert_eq!(array.capacity(), 1);
        
        // Push one more to trigger resize
        array.push(2).unwrap();
        assert_eq!(array.size(), 2);
        assert_eq!(array.capacity(), 2);
        
        // Push to exactly new capacity
        array.push(3).unwrap();
        assert_eq!(array.size(), 3);
        assert_eq!(array.capacity(), 4); // Resized again
    }
    
    #[test]
    fn test_pop_basic() {
        let mut array = DynamicArray::new(0, 5).unwrap();
        
        // Push some values
        array.push(10).unwrap();
        array.push(20).unwrap();
        array.push(30).unwrap();
        
        assert_eq!(array.size(), 3);
        
        // Pop values in reverse order
        assert_eq!(array.pop(), Ok(30));
        assert_eq!(array.size(), 2);
        
        assert_eq!(array.pop(), Ok(20));
        assert_eq!(array.size(), 1);
        
        assert_eq!(array.pop(), Ok(10));
        assert_eq!(array.size(), 0);
        
        // Try to pop from empty array
        assert_eq!(array.pop(), Err("Cannot pop from an empty array"));
    }
    
    #[test]
    fn test_pop_from_empty() {
        let mut array = DynamicArray::new(0, 5).unwrap();
        
        // Try to pop from empty array
        assert_eq!(array.pop(), Err("Cannot pop from an empty array"));
        assert_eq!(array.size(), 0);
        assert_eq!(array.capacity(), 5);
    }
    
    #[test]
    fn test_pop_with_shrinking() {
        let mut array = DynamicArray::new(0, 16).unwrap();
        
        // Fill the array
        for i in 0..8 {
            array.push(i as u64).unwrap();
        }
        
        assert_eq!(array.size(), 8);
        assert_eq!(array.capacity(), 16);
        
        // Pop elements until we trigger shrinking (size <= capacity/4)
        for i in (0..8).rev() {
            assert_eq!(array.pop(), Ok(i as u64));
        }
        
        // After popping all elements, size should be 0
        assert_eq!(array.size(), 0);
        // Capacity might have shrunk depending on the shrinking strategy
        assert!(array.capacity() <= 16);
    }
    
    #[test]
    fn test_pop_preserves_other_elements() {
        let mut array = DynamicArray::new(0, 5).unwrap();
        
        // Push multiple values
        array.push(100).unwrap();
        array.push(200).unwrap();
        array.push(300).unwrap();
        
        // Pop the last element
        assert_eq!(array.pop(), Ok(300));
        
        // Verify other elements are still accessible
        assert_eq!(array.size(), 2);
        assert_eq!(array.get(0), Some(&100));
        assert_eq!(array.get(1), Some(&200));
        assert_eq!(array.get(2), None); // Beyond size
    }
    
    #[test]
    fn test_pop_single_element() {
        let mut array = DynamicArray::new(0, 5).unwrap();
        
        // Push a single element
        array.push(42).unwrap();
        assert_eq!(array.size(), 1);
        
        // Pop the single element
        assert_eq!(array.pop(), Ok(42));
        assert_eq!(array.size(), 0);
        
        // Try to pop again
        assert_eq!(array.pop(), Err("Cannot pop from an empty array"));
    }
    
    #[test]
    fn test_pop_large_values() {
        let mut array = DynamicArray::new(0, 3).unwrap();
        
        // Push some large values
        array.push(u64::MAX).unwrap();
        array.push(0).unwrap();
        array.push(123456789).unwrap();
        
        // Pop in reverse order
        assert_eq!(array.pop(), Ok(123456789));
        assert_eq!(array.pop(), Ok(0));
        assert_eq!(array.pop(), Ok(u64::MAX));
        assert_eq!(array.pop(), Err("Cannot pop from an empty array"));
    }
    
    #[test]
    fn test_pop_after_multiple_operations() {
        let mut array = DynamicArray::new(0, 2).unwrap();
        
        // Push, pop, push, pop sequence
        array.push(10).unwrap();
        array.push(20).unwrap();
        assert_eq!(array.size(), 2);
        
        assert_eq!(array.pop(), Ok(20));
        assert_eq!(array.size(), 1);
        
        array.push(30).unwrap();
        assert_eq!(array.size(), 2);
        
        assert_eq!(array.pop(), Ok(30));
        assert_eq!(array.pop(), Ok(10));
        assert_eq!(array.size(), 0);
        
        // Push after emptying
        array.push(40).unwrap();
        assert_eq!(array.size(), 1);
        assert_eq!(array.pop(), Ok(40));
    }
    
    #[test]
    fn test_pop_with_resize_and_shrink() {
        let mut array = DynamicArray::new(0, 2).unwrap();
        
        // Fill to trigger resize
        array.push(1).unwrap();
        array.push(2).unwrap();
        array.push(3).unwrap(); // This should trigger resize to capacity 4
        
        assert_eq!(array.size(), 3);
        assert_eq!(array.capacity(), 4);
        
        // Pop all elements (this should trigger shrink)
        assert_eq!(array.pop(), Ok(3));
        assert_eq!(array.pop(), Ok(2));
        assert_eq!(array.pop(), Ok(1));
        
        assert_eq!(array.size(), 0);
        // Capacity should have shrunk
        assert!(array.capacity() <= 4);
    }
    
    #[test]
    fn test_pop_consistency() {
        let mut array = DynamicArray::new(0, 10).unwrap();
        
        // Push a sequence of numbers
        for i in 0..5 {
            array.push(i as u64).unwrap();
        }
        
        // Pop them all and verify they come out in reverse order
        for i in (0..5).rev() {
            assert_eq!(array.pop(), Ok(i as u64));
        }
        
        // Verify array is empty
        assert_eq!(array.size(), 0);
        assert_eq!(array.pop(), Err("Cannot pop from an empty array"));
    }

    #[test]
    fn test_insert_basic() {
        let mut array = DynamicArray::new(0, 5).unwrap();
        
        // Insert at the beginning
        array.insert(0, 10).unwrap();
        assert_eq!(array.size(), 1);
        assert_eq!(array.get(0), Some(&10));
        
        // Insert at the end
        array.insert(1, 20).unwrap();
        assert_eq!(array.size(), 2);
        assert_eq!(array.get(0), Some(&10));
        assert_eq!(array.get(1), Some(&20));
        
        // Insert in the middle
        array.insert(1, 15).unwrap();
        assert_eq!(array.size(), 3);
        assert_eq!(array.get(0), Some(&10));
        assert_eq!(array.get(1), Some(&15));
        assert_eq!(array.get(2), Some(&20));
    }

    #[test]
    fn test_insert_with_resize() {
        let mut array = DynamicArray::new(0, 2).unwrap();
        
        // Fill the array
        array.insert(0, 10).unwrap();
        array.insert(1, 20).unwrap();
        assert_eq!(array.size(), 2);
        assert_eq!(array.capacity(), 2);
        
        // Insert one more - should trigger resize
        array.insert(1, 15).unwrap();
        assert_eq!(array.size(), 3);
        assert_eq!(array.capacity(), 4); // Doubled from 2
        
        // Verify all values are in correct positions
        assert_eq!(array.get(0), Some(&10));
        assert_eq!(array.get(1), Some(&15));
        assert_eq!(array.get(2), Some(&20));
    }

    #[test]
    fn test_insert_at_boundaries() {
        let mut array = DynamicArray::new(0, 3).unwrap();
        
        // Insert at index 0 (beginning)
        array.insert(0, 100).unwrap();
        assert_eq!(array.size(), 1);
        assert_eq!(array.get(0), Some(&100));
        
        // Insert at index 1 (end)
        array.insert(1, 200).unwrap();
        assert_eq!(array.size(), 2);
        assert_eq!(array.get(0), Some(&100));
        assert_eq!(array.get(1), Some(&200));
        
        // Insert at index 0 again (beginning)
        array.insert(0, 50).unwrap();
        assert_eq!(array.size(), 3);
        assert_eq!(array.get(0), Some(&50));
        assert_eq!(array.get(1), Some(&100));
        assert_eq!(array.get(2), Some(&200));
    }

    #[test]
    fn test_insert_validation() {
        let mut array = DynamicArray::new(0, 5).unwrap();
        
        // Insert at valid index
        array.insert(0, 10).unwrap();
        
        // Try to insert beyond current size
        assert_eq!(array.insert(2, 20), Err("Index out of bounds: cannot insert beyond current size"));
        
        // Insert at valid index again
        array.insert(1, 20).unwrap();
        assert_eq!(array.size(), 2);
    }

    #[test]
    fn test_insert_multiple_resizes() {
        let mut array = DynamicArray::new(0, 1).unwrap();
        
        // Insert values that will trigger multiple resizes
        for i in 0..10 {
            array.insert(0, i as u64).unwrap(); // Insert at beginning each time
        }
        
        assert_eq!(array.size(), 10);
        assert_eq!(array.capacity(), 16); // 1 -> 2 -> 4 -> 8 -> 16
        
        // Verify values are in reverse order (last inserted first)
        for i in 0..10 {
            assert_eq!(array.get(i), Some(&((9 - i) as u64)));
        }
    }

    #[test]
    fn test_delete_basic() {
        let mut array = DynamicArray::new(0, 5).unwrap();
        
        // Insert some values
        array.insert(0, 10).unwrap();
        array.insert(1, 20).unwrap();
        array.insert(2, 30).unwrap();
        
        assert_eq!(array.size(), 3);
        
        // Delete from middle
        assert_eq!(array.delete(1), Ok(20));
        assert_eq!(array.size(), 2);
        assert_eq!(array.get(0), Some(&10));
        assert_eq!(array.get(1), Some(&30));
        
        // Delete from beginning
        assert_eq!(array.delete(0), Ok(10));
        assert_eq!(array.size(), 1);
        assert_eq!(array.get(0), Some(&30));
        
        // Delete from end
        assert_eq!(array.delete(0), Ok(30));
        assert_eq!(array.size(), 0);
    }

    #[test]
    fn test_delete_with_shrinking() {
        let mut array = DynamicArray::new(0, 16).unwrap();
        
        // Fill the array
        for i in 0..8 {
            array.insert(i, i as u64).unwrap();
        }
        
        assert_eq!(array.size(), 8);
        assert_eq!(array.capacity(), 16);
        
        // Delete elements until we trigger shrinking (size <= capacity/4)
        for i in (0..8).rev() {
            assert_eq!(array.delete(i), Ok(i as u64));
        }
        
        // After deleting all elements, size should be 0
        assert_eq!(array.size(), 0);
        // Capacity might have shrunk depending on the shrinking strategy
        assert!(array.capacity() <= 16);
    }

    #[test]
    fn test_delete_validation() {
        let mut array = DynamicArray::new(0, 5).unwrap();
        
        // Try to delete from empty array
        assert_eq!(array.delete(0), Err("Cannot delete from an empty array"));
        
        // Insert a value
        array.insert(0, 10).unwrap();
        
        // Try to delete beyond current size
        assert_eq!(array.delete(1), Err("Index out of bounds: cannot delete beyond current size"));
        
        // Delete valid index
        assert_eq!(array.delete(0), Ok(10));
        assert_eq!(array.size(), 0);
    }

    #[test]
    fn test_delete_preserves_order() {
        let mut array = DynamicArray::new(0, 5).unwrap();
        
        // Insert values in order
        for i in 0..5 {
            array.insert(i, i as u64).unwrap();
        }
        
        // Delete from middle and verify order is preserved
        assert_eq!(array.delete(2), Ok(2));
        assert_eq!(array.size(), 4);
        assert_eq!(array.get(0), Some(&0));
        assert_eq!(array.get(1), Some(&1));
        assert_eq!(array.get(2), Some(&3));
        assert_eq!(array.get(3), Some(&4));
        
        // Delete from beginning
        assert_eq!(array.delete(0), Ok(0));
        assert_eq!(array.size(), 3);
        assert_eq!(array.get(0), Some(&1));
        assert_eq!(array.get(1), Some(&3));
        assert_eq!(array.get(2), Some(&4));
    }

    #[test]
    fn test_insert_delete_sequence() {
        let mut array = DynamicArray::new(0, 3).unwrap();
        
        // Insert, delete, insert sequence
        array.insert(0, 10).unwrap();
        array.insert(1, 20).unwrap();
        assert_eq!(array.size(), 2);
        
        assert_eq!(array.delete(0), Ok(10));
        assert_eq!(array.size(), 1);
        
        array.insert(0, 30).unwrap();
        array.insert(1, 40).unwrap();
        assert_eq!(array.size(), 3);
        
        // Verify final state
        assert_eq!(array.get(0), Some(&30));
        assert_eq!(array.get(1), Some(&40));
        assert_eq!(array.get(2), Some(&20));
    }

    #[test]
    fn test_insert_delete_edge_cases() {
        let mut array = DynamicArray::new(0, 2).unwrap();
        
        // Insert single element
        array.insert(0, 100).unwrap();
        assert_eq!(array.size(), 1);
        
        // Delete single element
        assert_eq!(array.delete(0), Ok(100));
        assert_eq!(array.size(), 0);
        
        // Insert multiple elements
        array.insert(0, 200).unwrap();
        array.insert(0, 300).unwrap();
        assert_eq!(array.size(), 2);
        
        // Delete all elements
        assert_eq!(array.delete(0), Ok(300));
        assert_eq!(array.delete(0), Ok(200));
        assert_eq!(array.size(), 0);
    }

    #[test]
    fn test_insert_delete_with_resize_and_shrink() {
        let mut array = DynamicArray::new(0, 2).unwrap();
        
        // Fill to trigger resize
        array.insert(0, 1).unwrap();
        array.insert(1, 2).unwrap();
        array.insert(2, 3).unwrap(); // This should trigger resize to capacity 4
        
        assert_eq!(array.size(), 3);
        assert_eq!(array.capacity(), 4);
        
        // Delete all elements (this should trigger shrink)
        assert_eq!(array.delete(0), Ok(1));
        assert_eq!(array.delete(0), Ok(2));
        assert_eq!(array.delete(0), Ok(3));
        
        assert_eq!(array.size(), 0);
        // Capacity should have shrunk
        assert!(array.capacity() <= 4);
    }

    #[test]
    fn test_insert_delete_large_values() {
        let mut array = DynamicArray::new(0, 3).unwrap();
        
        // Insert some large values
        array.insert(0, u64::MAX).unwrap();
        array.insert(1, 0).unwrap();
        array.insert(2, 123456789).unwrap();
        
        // Delete and verify
        assert_eq!(array.delete(1), Ok(0));
        assert_eq!(array.size(), 2);
        assert_eq!(array.get(0), Some(&u64::MAX));
        assert_eq!(array.get(1), Some(&123456789));
        
        // Insert another large value
        array.insert(1, 987654321).unwrap();
        assert_eq!(array.size(), 3);
        assert_eq!(array.get(1), Some(&987654321));
    }
}
