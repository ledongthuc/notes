use std::path::{Path, PathBuf};

struct MyString {}

impl AsRef<str> for MyString {
    fn as_ref(&self) -> &'static str {
        "my hello"
    }
}

fn get_my_string<'a, T>(t: &'a T) -> &'a str where T : AsRef<str> {
    t.as_ref()
}

fn join_path<T: AsRef<Path>>(path: T) -> PathBuf {
    path.as_ref().join("file.txt")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string() {
        let s = "Hello".to_string();
        let b = s.as_ref();

        let expected : [u8; 5] = [72, 101, 108, 108, 111];
        assert_eq!(b, expected);
    }

    #[test]
    fn test_my_string() {
        let my_s = MyString{};
        let s = get_my_string(&my_s);
        assert_eq!(s, "my hello")
    }

    #[test]
    fn test_path() {
        let string_path = String::from("path/to/dir");
        let pathbuf = join_path(string_path);
        assert_eq!(pathbuf.to_str().unwrap(), "path/to/dir/file.txt")
    }
}
