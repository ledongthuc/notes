struct UseForCaptureRef {
    value: usize,
}

#[derive(Debug)]
struct I {
    value: usize,
}

#[cfg(test)]
mod tests {
    use std::thread;

    use crate::{UseForCaptureRef, I};

    #[test]
    fn test_vec_sort() {
        let mut v = vec![1, 2, 3, 4, 5];
        v.sort_by_key(|x| -x);
        assert_eq!(v, vec![5, 4, 3, 2, 1]);
    }

    #[test]
    fn test_capture_copy() {
        let i = 2;
        let v: Vec<UseForCaptureRef> =
            vec![UseForCaptureRef { value: 1 }, UseForCaptureRef { value: 2 }];
        let vv: Vec<usize> = v.into_iter().map(|x| x.value % i).collect();
        assert_eq!(vv, vec![1, 0]);
    }

    #[test]
    fn test_capture_ref() {
        let i = I { value: 2 };
        let v: UseForCaptureRef = UseForCaptureRef { value: 1 };
        let mut counter = 1;
        let mut builder = String::from("1");
        let mut mapping_fn = || {
            counter += 1;
            builder.push('2');
            // let builder2 = builder; // Can't move
            v.value % i.value
        };

        let result = mapping_fn();
        assert_eq!(result, 1);

        // let _v2 = &counter;
        let _i2 = &i;
        // let _counter_ref = &counter;
        let result = mapping_fn();
        assert_eq!(result, 1);

        assert_eq!(counter, 3);
        assert_eq!(builder, String::from("122"));
    }

    #[test]
    fn test_steal() {
        let i = I { value: 2 };
        test_steal_helper(i)
    }
    fn test_steal_helper(i: I) {
        let v: UseForCaptureRef = UseForCaptureRef { value: 1 };
        let mapping_fn = move |x: UseForCaptureRef| {
            println!("{:?}", i);
            x.value % i.value
        };
        assert_eq!(mapping_fn(v), 1);
        // let _v = v;
        // let _i = i;
    }

    fn accept_fn_trait<F>(f: F)
    where
        F: Fn(&str) -> usize,
    {
        println!("Fn trait: {}", f("hello"))
    }
    fn accept_fn(f: fn(&str) -> usize) {
        println!("fn: {}", f("hello"))
    }
    fn test_fn(s: &str) -> usize {
        s.len()
    }
    #[test]
    fn test_closure_as_parameter() {
        accept_fn_trait(test_fn);
        accept_fn_trait(|s: &str| s.len());

        accept_fn(test_fn);
        accept_fn(|s: &str| s.len());
    }

    #[test]
    fn test_drop() {
        let s = "hehe".to_string();
        let f = || drop(s);
        // let _s = s;
        f();
        // f();
    }

    fn fn_once_call<F>(closure: F)
    where F: FnOnce(),
    // where F: Fn(),
    {
        closure();
        // closure();
    }

    #[test]
    fn test_fn_once() {
        let s = "hehe".to_string();
        let f = || drop(s); // closure move ownership should be FnOnce
        fn_once_call(f)
    }

    fn fn_mut<F>(mut closure: F)
    where F: FnMut(),
    // where F: Fn(),
    {
        closure();
        // closure();
    }

    #[test]
    fn test_fn_mut() {
        let mut s = "hehe".to_string();
        let f = || s.push('!'); // closure mutable capture variable ref will be FnMut
        fn_mut(f)
    }
}
