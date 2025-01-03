use std::path::PathBuf;

use fuck_backslash::*;

fn main() {
    let path = PathBuf::from("C:\\Users\\xxx\\Desktop\\test.txt");
    let replaced = path.fuck_backslash();
    assert_eq!(replaced, PathBuf::from("C:/Users/xxx/Desktop/test.txt"));
}
