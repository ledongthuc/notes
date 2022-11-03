fn main() {
    let str_conv: u32 = "42".parse().expect("Not a number!");
    println!("str_conv: {}", str_conv);

    let str_conv_f64: f64 = "42".parse().expect("Not a number!");
    println!("str_conv_64: {}", str_conv_f64);

    // Following line will got error because "str_conv_without_type" missing type declare
    // let str_conv_without_type = "42".parse().expect("Not a number!");

    // Scalar types: integers
    println!("min i8: {}, max i8: {}", i8::min_value(), i8::max_value());
    println!("min u8: {}, max u8: {}", u8::min_value(), u8::max_value());
    println!(
        "min i16: {}, max i16: {}",
        i16::min_value(),
        i16::max_value()
    );
    println!(
        "min u16: {}, max u16: {}",
        u16::min_value(),
        u16::max_value()
    );
    println!(
        "min i32: {}, max i32: {}",
        i32::min_value(),
        i32::max_value()
    );
    println!(
        "min u32: {}, max u32: {}",
        u32::min_value(),
        u32::max_value()
    );
    println!(
        "min i64: {}, max i64: {}",
        i64::min_value(),
        i64::max_value()
    );
    println!(
        "min u64: {}, max u64: {}",
        u64::min_value(),
        u64::max_value()
    );
    println!(
        "min i128: {}, max i128: {}",
        i128::min_value(),
        i128::max_value()
    );
    println!(
        "min u128: {}, max u128: {}",
        u128::min_value(),
        u128::max_value()
    );
    println!(
        "min isize: {}, max isize: {}",
        isize::min_value(),
        isize::max_value()
    );
    println!(
        "min usize: {}, max usize: {}",
        usize::min_value(),
        usize::max_value()
    );
    println!("decimal: {}", 10_000_000);
    println!("hex: {}", 0xff);
    println!("octal: {}", 0o77);
    println!("binary: {}", 0b1111001);
    println!("byte: {}", b'A');
    // Must to use: cargo run --release to support following overflow value
    // let overflow_u8: u8 = u8::max_value() + 1;
    // println!("overflow_u8: {}", overflow_u8);
    println!("i8::wrapping_add(127, 1): {}", i8::wrapping_add(127, 1));
    // Will panic
    // i8::checked_add(127, 1).expect("overflow");
    println!(
        "i8::overflowing_add(127, 1): {:?}",
        i8::overflowing_add(127, 1)
    );

    // Scalar types: floating points
    println!("min f64: {}, max f64: {}", f64::MIN, f64::MAX);
    println!("min f32: {}, max f32: {}", f32::MIN, f32::MAX);

    // Numberic operation
    println!("1 + 2: {}", 1 + 2);
    println!("1.0 + f64::from(2): {}", 1.0 + f64::from(2));
    println!("1.0 + 2.1: {}", 1.0 + 2.1);
    println!("3.0 - 2.0: {}", 3.0 - 2.0);
    println!("3 * 2: {}", 3 * 2);
    println!("4 / 2: {}", 4 / 2);
    println!("5 / 2: {}", 5 / 2);
    println!("5 % 2: {}", 5 % 2);

    // Scalar types: booleans
    println!("bool: {} and {}", true, false);

    // Scalar types: chars
    println!("chars: {} {} {}", 'z', 'ℤ', '😻');

    // scalar types: compound types - tuples
    let tupes: (i32, f64, u8) = (500, 6.4, 1);
    println!("types: {:?}", tupes);
    let (x, y, z) = tupes;
    println!("x: {}, y: {}, z: {}", x, y, z);
    println!("tupes.0: {}", tupes.0);

    // scalar types: compound types - array
    let arr = [1, 2, 3, 4, 5];
    println!("arr: {:?}", arr);
    let testArr: [u8; 5] = [1, 2, 3, 4, 5];
    println!("testArr: {:?}, testArr[2]: {}", testArr, testArr[2]);
    let fiveThree = [3; 5];
    println!("fiveThree: {:?}", fiveThree);
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_type_suffix() {
        let a = 32u32;

        assert_eq!("u32", print_type_of(&a));
    }

    fn print_type_of<T>(_: &T) -> &str {
        std::any::type_name::<T>()
    }
}
