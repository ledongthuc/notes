#[derive(Debug)]
struct List {
    values: [String; 2]
}


impl IntoIterator for List {
    type Item = String;
    type IntoIter = ListInter;
    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter{
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
    use std::{iter::{from_fn, successors}, collections::BTreeMap};

    use super::*;

    #[test]
    fn test_into_iter() {
        let l = List{
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
        from_fn(|| Some(10)).take(10).for_each(|i| assert_eq!(i, 10));
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
        let v: Vec<i32> = (1..10).into_iter().map(|i| i+1).collect();
        let expected : Vec<i32> = (2..11).collect();
        assert_eq!(v.len(), (2..11).len());
        for (i, value) in v.iter().enumerate() {
            assert_eq!(value, &expected[i]);
        }
    }

    #[test]
    fn test_filter() {
        let v: Vec<i32> = (1..10).into_iter().filter(|i| i%2==0).collect();
        let expected : Vec<i32> = vec![2,4,6,8];
        assert_eq!(v.len(), expected.len());
        for (i, value) in v.iter().enumerate() {
            assert_eq!(value, &expected[i]);
        }
    }

    #[test]
    fn test_filter_map() {
        let v: Vec<i32> = (1..10).into_iter().filter_map(|i| {
            if i%2==0 {
                return Some(i+1);
            }
            None
        }).collect();
        let expected : Vec<i32> = vec![3,5,7,9];
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


        let expected : Vec<_> = vec!["Stockholm", "Gothenburg", "Malmo", "Ho Chi Minh", "Ha Noi"];
        assert_eq!(all_cities.len(), expected.len());
        for (i, value) in all_cities.iter().enumerate() {
            assert_eq!(value, &expected[i]);
        }
    }

    #[test]
    fn test_take_while() {
        let v : Vec<i32> = (0..).take_while(|i| *i < 10).collect();

        let expected : Vec<_> = vec![0,1,2,3,4,5,6,7,8,9];
        assert_eq!(v.len(), expected.len());
        for (i, value) in v.iter().enumerate() {
            assert_eq!(value, &expected[i]);
        }
    }

    #[test]
    fn test_skip() {
        let v : Vec<i32> = (0..10).skip(5).collect();

        let expected : Vec<_> = vec![5,6,7,8,9];
        assert_eq!(v.len(), expected.len());
        for (i, value) in v.iter().enumerate() {
            assert_eq!(value, &expected[i]);
        }
    }

    #[test]
    fn test_skip_while() {
        let v : Vec<i32> = (0..10).skip_while(|i| *i < 5).collect();

        let expected : Vec<_> = vec![5,6,7,8,9];
        assert_eq!(v.len(), expected.len());
        for (i, value) in v.iter().enumerate() {
            assert_eq!(value, &expected[i]);
        }
    }

    #[test]
    fn test_peek() {
        let mut v = (5..10).skip(3).peekable();
        assert_eq!(v.peek(), Some(&8));

        let mut v = (5..10).skip(5).peekable();
        assert_eq!(v.peek(), None);

    }
}
