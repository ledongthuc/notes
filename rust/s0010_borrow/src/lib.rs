use std::borrow::Borrow;
use std::path::PathBuf;
use std::path::Path;

struct A<'a> {
    value: &'a usize
}

impl<'a> Borrow<usize> for A<'a> {
    fn borrow(&self) -> &'a usize {
        self.value
    }
}

fn join_path<T: Borrow<Path>>(path: T) -> PathBuf {
    path.borrow().join("file.txt")
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_borrow() {
        let a = A{value: &5};
        let borrow_usize : &usize = a.borrow();
        assert_eq!(borrow_usize, &5);
    }

    #[test]
    fn test_borrow_string_path() {
        let string_path = String::from("path/to/another/dir");
        let path = PathBuf::from(string_path);
        let pathbuf = join_path(path);
        assert_eq!(pathbuf.to_str().unwrap(), "path/to/another/dir/file.txt")
    }
}
