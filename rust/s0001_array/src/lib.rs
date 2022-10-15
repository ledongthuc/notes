#![feature(array_zip)]
#![feature(array_try_map)]
#![feature(array_methods)]
#![feature(split_array)]
#[cfg(test)]
mod tests {
    use std::cmp::Ordering;
    use std::ops::Index;

    #[test]
    fn basic_array() {
        let mut array: [i32; 3] = [0; 3];
        assert_eq!([0, 0, 0], array);

        array[1] = 1;
        array[2] = 2;
        assert_eq!([0, 1, 2], array);

        let mut i = 0;
        for x in array {
            assert_eq!(i, x);
            i += 1;
        }
    }

    #[test]
    fn array_map() {
        let array = [0, 1, 2, 3];
        let array = array.map(|x| (x + 1) as f32);
        assert_eq!([1.0, 2.0, 3.0, 4.0], array);

        let array: Vec<usize> = array
            .into_iter()
            .map(|x| x.to_string())
            .map(|x| x.len())
            .collect();
        let expected = Vec::from([1, 1, 1, 1]);
        assert_eq!(expected, array);
    }

    #[test]
    fn array_try_map() {
        let array = ["1", "2a", "3"];
        let array = array.try_map(|x| x.parse::<u32>());
        assert!(array.is_err());
    }

    #[test]
    fn array_zip() {
        let array1 = [1, 2, 3, 4];
        let array2 = [99, 98, 97, 96];
        assert_eq!([(1, 99), (2, 98), (3, 97), (4, 96)], array1.zip(array2));
    }

    #[test]
    fn array_as_slice() {
        let array = [1, 2, 3, 4];
        assert_eq!(&[1, 2, 3, 4], array.as_slice());
    }

    #[test]
    fn array_as_mut_slice() {
        let mut array = [1, 2, 3, 4];
        let mut_slice = array.as_mut_slice();
        mut_slice[0] = 0;
        assert_eq!(&mut [0, 2, 3, 4], mut_slice);
    }

    #[test]
    fn array_as_ref() {
        let array = [1, 2, 3, 4];
        let array_ref = array.as_ref();
        assert_eq!(&[1, 2, 3, 4], array_ref);
    }

    #[test]
    fn array_each_ref() {
        let array = [1, 2, 3, 4];
        let array_ref = array.each_ref();
        assert_eq!([&1, &2, &3, &4], array_ref);
    }

    #[test]
    fn array_each_mut() {
        let mut array = [1, 2, 3, 4];
        let array_mut = array.each_mut();
        assert_eq!([&mut 1, &mut 2, &mut 3, &mut 4], array_mut);
    }

    #[test]
    fn array_split_array_ref() {
        let array = [1, 2, 3, 4, 5];
        let (left, right) = array.split_array_ref::<3>();
        assert_eq!(&[1, 2, 3], left);
        assert_eq!(&[4, 5], right);
    }

    #[test]
    fn array_split_array_mut() {
        let mut array = [1, 2, 3, 4, 5];
        let (left, right) = array.split_array_mut::<3>();
        assert_eq!(&mut [1, 2, 3], left);
        assert_eq!(&mut [4, 5], right);

        left[1] = 99;
        right[1] = 88;
        assert_eq!(&mut [1, 99, 3], left);
        assert_eq!(&mut [4, 88], right);
    }

    #[test]
    fn array_rplit_array_ref() {
        let array = [1, 2, 3, 4, 5];
        let (left, right) = array.rsplit_array_ref::<3>();
        assert_eq!(&[1, 2], left);
        assert_eq!(&[3, 4, 5], right);
    }

    #[test]
    fn array_rplit_array_mut() {
        let mut array = [1, 2, 3, 4, 5];
        let (left, right) = array.rsplit_array_mut::<3>();
        assert_eq!(&mut [1, 2], left);
        assert_eq!(&mut [3, 4, 5], right);

        left[1] = 99;
        right[1] = 88;
        assert_eq!(&mut [1, 99], left);
        assert_eq!(&mut [3, 88, 5], right);
    }

    #[test]
    fn array_index() {
        let array = [1, 2, 3, 4, 5];
        let value = array.index(3);
        assert_eq!(&4, value);
    }

    #[test]
    fn array_get() {
        let array = [1, 2, 3, 4, 5];

        let value = array.get(3);
        assert!(value.is_some());
        assert_eq!(&mut 4, value.unwrap());


        let value = array.get(9);
        assert!(value.is_none());
    }

    #[test]
    fn array_get_mut() {
        let mut array = [1, 2, 3, 4, 5];

        let value = array.get_mut(3);
        assert!(value.is_some());
        assert_eq!(&mut 4, value.unwrap());

        let x = array.get_mut(3).unwrap();
        *x = 101;
        assert_eq!(&mut 101, array.get_mut(3).unwrap());

        let value = array.get(9);
        assert!(value.is_none());
    }

    #[test]
    fn array_iter() {
        let array = [1, 2, 3, 4, 5];
        for (index, item) in array.iter().enumerate() {
            assert_eq!(&(index + 1), item);
        }
    }

    #[test]
    fn array_iter_mut() {
        let mut array: [i32; 5] = [1, 2, 3, 4, 5];
        for (index, item) in array.iter_mut().enumerate() {
            let x: &_ = item;
            assert_eq!(&(index as i32 + 1), x);
            *item += 10;
        }
        assert_eq!([11, 12, 13, 14, 15], array);
    }

    #[test]
    fn array_into_iter() {
        let array: [Box<i32>; 5] = [
            Box::new(1),
            Box::new(2),
            Box::new(3),
            Box::new(4),
            Box::new(5),
        ];
        for (index, item) in array.into_iter().enumerate() {
            let another_item = item;
            assert_eq!(Box::new(index as i32 + 1), another_item);
            // Don't work, item ownership is moved away
            // assert_eq!(Box::new(index as i32 + 1), item);
        }
    }

    #[test]
    fn array_ord() {
        let array: [i32; 5] = [1, 2, 3, 4, 5];
        assert_eq!(Ordering::Equal, array.cmp(&[1, 2, 3, 4, 5]));
        assert_eq!(Ordering::Less, array.cmp(&[4, 2, 3, 4, 5]));
        assert_eq!(Ordering::Greater, array.cmp(&[1, 1, 3, 4, 5]));
    }
}
