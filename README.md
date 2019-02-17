# radix-tree

A [radix tree] implementation for router, path search.

[![Build Status](https://travis-ci.org/trek-rs/radix-tree.svg?branch=master)](https://travis-ci.org/trek-rs/radix-tree)
[![Latest version](https://img.shields.io/crates/v/radix-tree.svg)](https://crates.io/crates/radix-tree)
[![Documentation](https://docs.rs/radix-tree/badge.svg)](https://docs.rs/radix-tree)
![License](https://img.shields.io/crates/l/radix-tree.svg)

## Features

- Supports many types. Like `char`, `u8` etc.

## Usage

```rust
use radix_tree::{Node, Vectorable};

impl Vectorable<char> for &str {
    fn into(self) -> Vec<char> {
        self.chars().collect()
    }
}

let mut tree = Node::<char, bool>::new("", false);

tree.insert("alligator", true);
tree.insert("alien", true);
tree.insert("baloon", true);
tree.insert("chromodynamic", true);
tree.insert("romane", true);
tree.insert("romanus", true);
tree.insert("romulus", true);
tree.insert("rubens", true);
tree.insert("ruber", true);
tree.insert("rubicon", true);
tree.insert("rubicundus", true);
tree.insert("all", true);
tree.insert("rub", true);
tree.insert("ba", true);
tree.insert("你好，世界", true);
tree.insert("你好", true);
tree.insert("你", true);

let node = tree.find("all");
assert_eq!(node.is_some(), true);
assert_eq!(node.unwrap().data.unwrap(), true);

let node = tree.find("dota2");
assert_eq!(node.is_none(), true);
```

**Tree**:

```
`-(a) [l] --> [li] []=false
                      `l-(i) [gator] --> [] []=true
                      `ien-() [] --> [] []=true
`-(b) [a] --> [l] []=true
                    `loon-() [] --> [] []=true
`-(c) [hromodynamic] --> [] []=true
`-(r) [] --> [ou] []=false
                     `om-(a) [n] --> [eu] []=false
                                             `e-() [] --> [] []=true
                                             `us-() [] --> [] []=true
                     `om-(u) [lus] --> [] []=true
                     `ub-(e) [] --> [nr] []=false
                                            `ns-() [] --> [] []=true
                                            `r-() [] --> [] []=true
                     `ub-(i) [c] --> [ou] []=false
                                             `on-() [] --> [] []=true
                                             `undus-() [] --> [] []=true
```

## Examples

```rust
impl Vectorable<char> for &str {
    fn into(self) -> Vec<char> {
        self.chars().collect()
    }
}

impl Vectorable<char> for String {
    fn into(self) -> Vec<char> {
        self.chars().collect()
    }
}

impl Vectorable<u8> for &str {
    fn into(self) -> Vec<u8> {
        self.as_bytes().to_owned()
    }
}

impl Vectorable<u8> for String {
    fn into(self) -> Vec<u8> {
        self.as_bytes().to_owned()
    }
}

impl<T> Vectorable<T> for Vec<T> {
    fn into(self) -> Vec<T> {
        self
    }
}

impl<T> Vectorable<T> for &[T]
where
    T: std::clone::Clone,
{
    fn into(self) -> Vec<T> {
        self.to_owned()
    }
}

let node = Node::<u8, &str>::new("Hello world!", "a");
assert_eq!(
    node.path,
    vec![72, 101, 108, 108, 111, 32, 119, 111, 114, 108, 100, 33]
);
assert_eq!(node.data.unwrap(), "a");

let node = Node::<u8, &str>::new("Hello 世界！", "a");
assert_eq!(
    node.path,
    vec![72, 101, 108, 108, 111, 32, 228, 184, 150, 231, 149, 140, 239, 188, 129]
);
assert_eq!(node.data.unwrap(), "a");

let node = Node::<char, &str>::new("Hello 世界！", "a");
assert_eq!(
    node.path,
    vec!['H', 'e', 'l', 'l', 'o', ' ', '世', '界', '！']
);
assert_eq!(node.data.unwrap(), "a");

let node = Node::<char, u32>::new("你好，世界！", 0);
assert_eq!(node.path, vec!['你', '好', '，', '世', '界', '！']);
assert_eq!(node.data.unwrap(), 0);

let node = Node::<u8, u8>::new("abcde", 1);
assert_eq!(node.path, vec![97, 98, 99, 100, 101]);
assert_eq!(node.data.unwrap(), 1);

let node = Node::new("abcde".as_bytes().to_vec(), 97);
assert_eq!(node.path, vec![97, 98, 99, 100, 101]);
assert_eq!(node.data.unwrap(), 97);

let node = Node::new("abcde".as_bytes(), 97);
assert_eq!(node.path, vec![97, 98, 99, 100, 101]);
assert_eq!(node.data.unwrap(), 97);
```

## Acknowledgements

It is inspired by the:

- [rax]
- [httprouter]
- [echo] router
- [trekjs] router

## License

This project is licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or
  http://opensource.org/licenses/MIT)

at your option.

[radix tree]: https://en.wikipedia.org/wiki/Radix_tree
[rax]: https://github.com/antirez/rax
[httprouter]: https://github.com/julienschmidt/httprouter
[echo]: https://github.com/labstack/echo
[trekjs]: https://github.com/trekjs/router
