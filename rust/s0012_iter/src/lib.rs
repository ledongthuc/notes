#[derive(Debug)]
struct List {
    values: [String; 2],
}

impl IntoIterator for List {
    type Item = String;
    type IntoIter = ListInter;
    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter {
            list: self.values,
            index: 0,
        }
    }
}

struct ListInter {
    list: [String; 2],
    index: usize,
}

impl Iterator for ListInter {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        if self.list.len() <= self.index {
            return None;
        }

        let v = Some(self.list[self.index].clone());
        self.index += 1;
        v
    }
}

#[cfg(test)]
mod tests {
    use std::{
        collections::BTreeMap,
        iter::{from_fn, successors},
    };

    use super::*;

    #[test]
    fn test_into_iter() {
        let l = List {
            values: ["1".to_string(), "2".to_string()],
        };

        let mut iterator = l.into_iter();

        let o = iterator.next();
        assert_eq!(o.unwrap(), "1".to_string());

        let o = iterator.next();
        assert_eq!(o.unwrap(), "2".to_string());

        let o = iterator.next();
        assert_eq!(o, None);

        // following line failed because l lost ownership after into_iter
        // let l2 = l
    }

    #[test]
    fn test_from_fn() {
        from_fn(|| Some(10))
            .take(10)
            .for_each(|i| assert_eq!(i, 10));
    }

    #[test]
    fn test_successor() {
        let v: Vec<i32> = successors(Some(1), |&z| Some(z + z)).take(3).collect();
        assert_eq!(v.len(), 3);
        assert_eq!(v[0], 1);
        assert_eq!(v[1], 2);
        assert_eq!(v[2], 4);
    }

    #[test]
    fn test_map() {
        let v: Vec<i32> = (1..10).into_iter().map(|i| i + 1).collect();
        let expected: Vec<i32> = (2..11).collect();
        assert_eq!(v.len(), (2..11).len());
        for (i, value) in v.iter().enumerate() {
            assert_eq!(value, &expected[i]);
        }
    }

    #[test]
    fn test_filter() {
        let v: Vec<i32> = (1..10).into_iter().filter(|i| i % 2 == 0).collect();
        let expected: Vec<i32> = vec![2, 4, 6, 8];
        assert_eq!(v.len(), expected.len());
        for (i, value) in v.iter().enumerate() {
            assert_eq!(value, &expected[i]);
        }
    }

    #[test]
    fn test_filter_map() {
        let v: Vec<i32> = (1..10)
            .into_iter()
            .filter_map(|i| {
                if i % 2 == 0 {
                    return Some(i + 1);
                }
                None
            })
            .collect();
        let expected: Vec<i32> = vec![3, 5, 7, 9];
        assert_eq!(v.len(), expected.len());
        for (i, value) in v.iter().enumerate() {
            assert_eq!(value, &expected[i]);
        }
    }

    #[test]
    fn test_flat() {
        let mut country_cities = BTreeMap::new();
        country_cities.insert("Sweden", vec!["Stockholm", "Gothenburg", "Malmo"]);
        country_cities.insert("Vietnam", vec!["Ho Chi Minh", "Ha Noi"]);
        let all_cities: Vec<_> = country_cities.values().flatten().cloned().collect();

        let expected: Vec<_> = vec!["Stockholm", "Gothenburg", "Malmo", "Ho Chi Minh", "Ha Noi"];
        assert_eq!(all_cities.len(), expected.len());
        for (i, value) in all_cities.iter().enumerate() {
            assert_eq!(value, &expected[i]);
        }
    }

    #[test]
    fn test_take_while() {
        let v: Vec<i32> = (0..).take_while(|i| *i < 10).collect();

        let expected: Vec<_> = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        assert_eq!(v.len(), expected.len());
        for (i, value) in v.iter().enumerate() {
            assert_eq!(value, &expected[i]);
        }
    }

    #[test]
    fn test_skip() {
        let v: Vec<i32> = (0..10).skip(5).collect();

        let expected: Vec<_> = vec![5, 6, 7, 8, 9];
        assert_eq!(v.len(), expected.len());
        for (i, value) in v.iter().enumerate() {
            assert_eq!(value, &expected[i]);
        }
    }

    #[test]
    fn test_skip_while() {
        let v: Vec<i32> = (0..10).skip_while(|i| *i < 5).collect();

        let expected: Vec<_> = vec![5, 6, 7, 8, 9];
        assert_eq!(v.len(), expected.len());
        for (i, value) in v.iter().enumerate() {
            assert_eq!(value, &expected[i]);
        }
    }

    #[test]
    fn test_peek() {
        let mut a = (0..10).peekable();
        let mut total = 0;
        loop {
            match a.peek() {
                Some(r) if r == &5 => break,
                Some(_) => total += a.next().unwrap(),
                None => break,
            }
        }
        assert_eq!(total, 10);
    }

    struct SwichTest(bool);

    impl Iterator for SwichTest {
        type Item = String;
        fn next(&mut self) -> Option<Self::Item> {
            if self.0 {
                self.0 = false;
                Some("Done!".to_string())
            } else {
                self.0 = true;
                None
            }
        }
    }

    #[test]
    fn test_fuse() {
        let mut switch = SwichTest(true);
        assert_eq!(switch.next().unwrap(), "Done!".to_string());
        assert_eq!(switch.next(), None);
        assert_eq!(switch.next().unwrap(), "Done!".to_string());

        let mut fuse = SwichTest(true).fuse();
        assert_eq!(fuse.next().unwrap(), "Done!".to_string());
        assert_eq!(fuse.next(), None);
        assert_eq!(fuse.next(), None);
    }

    #[test]
    fn test_reverse() {
        let mut a = (0..5).rev();
        assert_eq!(a.next().unwrap(), 4);
        assert_eq!(a.next().unwrap(), 3);
        assert_eq!(a.next().unwrap(), 2);
        assert_eq!(a.next().unwrap(), 1);
        assert_eq!(a.next().unwrap(), 0);
    }

    #[test]
    fn test_inspect() {
        let mut inspect_counter = 0;
        let mut a = (0..5).inspect(|_| inspect_counter += 1);
        a.next();
        a.next();
        a.next();
        a.next();
        a.next();
        a.next();
        a.next();
        assert_eq!(inspect_counter, 5);
    }

    #[test]
    fn test_chain() {
        let a: Vec<i32> = (0..6).chain(6..10).collect();
        assert_eq!(a, vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }

    #[test]
    fn test_enumerate() {
        let mut checking_index = 0;
        for (i, item) in (1..5).enumerate() {
            assert_eq!(i, checking_index);
            assert_eq!(item, checking_index + 1);
            checking_index += 1;
        }
    }

    #[test]
    fn test_zip() {
        let a1 = 0..10;
        let a2 = ["h", "e", "l", "l", "o"];
        let a3 : Vec<(i32, &str)> = a1.zip(a2).collect();
        assert_eq!(a3[4].0, 4);
        assert_eq!(a3[4].1, "o");
    }
}
