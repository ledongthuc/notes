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
        iter::{from_fn, successors}, ops::Add,
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

    #[test]
    fn test_by_ref() {
        let a = 0..10;
        let _b: Vec<i32> = a.take(5).collect();
        // next row got error because of take() takes ownership of a
        // assert_eq!(a.len(), 5);

        let mut c = 0..10;
        let d : Vec<i32> = c.by_ref().take(2).collect();
        assert_eq!(d.len(), 2);
        assert_eq!(d[0], 0);
        assert_eq!(d[1], 1);
    }

    #[test]
    fn test_cycle() {
        let mut a = [0,1].into_iter().cycle();
        assert_eq!(a.next().unwrap(), 0);
        assert_eq!(a.next().unwrap(), 1);
        assert_eq!(a.next().unwrap(), 0);
        assert_eq!(a.next().unwrap(), 1);
        assert_eq!(a.next().unwrap(), 0);
        assert_eq!(a.next().unwrap(), 1);
        assert_eq!(a.next().unwrap(), 0);
        assert_eq!(a.next().unwrap(), 1);
    }

    #[test]
    fn test_count() {
        let a = [0,1].into_iter().cycle().take(100).count();
        assert_eq!(a, 100);
    }

    #[test]
    fn test_sum() {
        let a : i32 = (1..5).into_iter().sum();
        assert_eq!(a, 10);
    }

    #[test]
    fn test_product() {
        let a : i64 = (1..5).into_iter().inspect(|x| println!("{}", x)).product();
        assert_eq!(a, 2 * 3 * 4 );
    }

    #[test]
    fn test_max() {
        let a : i64 = [1,6,3,2,1,-1,-7,3].into_iter().max().unwrap();
        assert_eq!(a, 6);

        let b  = ["this", "is", "my", "friend"].into_iter().max_by(|x, y| x.len().partial_cmp(&y.len()).unwrap() ).unwrap();
        assert_eq!(b, "friend");

        struct Student {
            name: String,
            age: usize,
            score: usize,
        }
        let students = [
            Student{ name: "A".to_string(), age: 25, score: 7 },
            Student{ name: "B".to_string(), age: 18, score: 3 },
            Student{ name: "C".to_string(), age: 20, score: 10 },
        ];
        let max_age = students.iter().max_by_key(|s| s.age);
        assert_eq!(max_age.unwrap().name, "A".to_string());
        let max_score = students.iter().max_by_key(|s| s.score);
        assert_eq!(max_score.unwrap().name, "C".to_string());
    }

    #[test]
    fn test_min() {
        let a : i64 = [1,6,3,2,1,-1,-7,3].into_iter().min().unwrap();
        assert_eq!(a, -7);

        let b  = ["this", "is", "my", "friend"].into_iter().min_by(|x, y| x.len().partial_cmp(&y.len()).unwrap() ).unwrap();
        assert_eq!(b, "is");

        struct Student {
            name: String,
            age: usize,
            score: usize,
        }
        let students = [
            Student{ name: "A".to_string(), age: 25, score: 7 },
            Student{ name: "B".to_string(), age: 18, score: 3 },
            Student{ name: "C".to_string(), age: 20, score: 10 },
        ];
        let min_age = students.iter().min_by_key(|s| s.age);
        assert_eq!(min_age.unwrap().name, "B".to_string());
        let min_score = students.iter().min_by_key(|s| s.score);
        assert_eq!(min_score.unwrap().name, "B".to_string());
    }

    #[test]
    fn test_any() {
        let mut a = [1,6,3,2,1,-1,-7,3].iter();
        assert!(a.any(|i| *i == 3));
        assert!(!a.any(|i| *i == 4));
    }

    #[test]
    fn test_all() {
        let mut a = [1,6,3,2,1,-1,-7,3].iter();
        assert!(a.all(|i| *i > -10));
        assert!(!a.any(|i| i % 2 == 0));
    }

    #[test]
    fn test_position() {
        let mut a = [1,6,3,2,1,1,-7,3].iter();
        assert_eq!(a.position(|i| *i == 1).unwrap(), 0);
    }

    #[test]
    fn test_rposition() {
        let mut a = [1,6,3,2,1,1,-7,3].iter();
        assert_eq!(a.rposition(|i| *i == 1).unwrap(), 5);
    }

    #[test]
    fn test_fold() {
        let a = (1..5).into_iter().fold(0, |left, i| left + i);
        assert_eq!(a, 10);
    }

    #[test]
    fn test_rfold() {
        let a = ["!", "world", "hello"].into_iter().rfold("".to_string(), |right, i| right.add(i));
        assert_eq!(a, "helloworld!");
    }

    #[test]
    fn test_nth() {
        let mut a = [1,6,3,2,1,1,-7,3].iter();
        assert_eq!(a.nth(3).unwrap().to_owned(), 2);
    }

    #[test]
    fn test_last() {
        let a = [1,6,3,2,1,1,-7,3].iter();
        assert_eq!(a.last().unwrap().to_owned(), 3);
    }

    #[test]
    fn test_partition() {
        let (a, b) : (Vec<i32>, Vec<i32>) = (1..10).into_iter().partition(|i| i % 2 == 0);
        assert_eq!(a, vec![2,4,6,8]);
        assert_eq!(b, vec![1,3,5,7,9]);
    }
}
