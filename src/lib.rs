#![doc = include_str!("../README.md")]
#![warn(clippy::cargo)]

use std::{ffi::OsString, path::PathBuf};

pub trait FuckBackslash {
    /// Replaces the windows separator `\` with the unix separator `/`.
    fn fuck_backslash(self) -> Self;
}

impl FuckBackslash for PathBuf {
    fn fuck_backslash(self) -> Self {
        unsafe {
            let mut encodded = self.into_os_string().into_encoded_bytes();
            for byte in encodded.iter_mut() {
                if *byte == b'\\' {
                    *byte = b'/';
                }
            }
            PathBuf::from(OsString::from_encoded_bytes_unchecked(encodded))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let testcase = [
            ("./../tmp/asd", "./../tmp/asd"),
            (
                r"C:\Users\xxx\Desktop\test.txt",
                "C:/Users/xxx/Desktop/test.txt",
            ),
            (r"/tmp\aasd", "/tmp/aasd"),
            (
                "C:\\一些中文名字\\とある日本語目録\\.\\\r\t",
                "C:/一些中文名字/とある日本語目録/./\r\t",
            ),
        ]
        .map(|(input, output)| (PathBuf::from(input), PathBuf::from(output)));

        for (input, output) in testcase {
            let replaced = input.fuck_backslash();
            assert_eq!(replaced, output);
        }
    }
}
