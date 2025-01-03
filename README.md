# fuck-backslash

Did you suffer from the problem that `\` breaks your path? For me, I have to record paths into a configuration file and use it cross-platform. If I create the configuration on windows, it will break my program when I run it on linux.

This is a simple crate to replace the backslash `\` with slash `/` in-place in your `PathBuf`.

## Usage

```rust,ignore
use fuck_backslash::*;

let path = PathBuf::from("C:\\Users\\xxx\\Desktop\\test.txt");
assert_eq!(path.fuck_backslash(), PathBuf::from("C:/Users/xxx/Desktop/test.txt"));
```
