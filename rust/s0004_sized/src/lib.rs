struct SizedStruct<T: Sized> {
    value: T,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sized_struct() {
        _ = SizedStruct::<String>{
            value: "Sized string".to_string(),
        };

        // str is litteral string that's not know at compile time
        // _ = SizedStruct::<str>{
        //     value: "Sized string",
        // };
    }

}
