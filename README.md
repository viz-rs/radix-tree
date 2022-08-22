# radix-tree

A [radix tree] implementation for router, path search.

[![Latest version](https://img.shields.io/crates/v/radix-tree.svg)](https://crates.io/crates/radix-tree)
[![Documentation](https://docs.rs/radix-tree/badge.svg)](https://docs.rs/radix-tree)
![License](https://img.shields.io/crates/l/radix-tree.svg)

## Features

- Supports many types. Like `char`, `u8` `u32` etc.

## Usage

```rust
use radix_tree::{Node, Vectorable};

let mut tree = Node::<char, bool>::new("", Some(false));

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
let node = Node::<u8, &str>::new("Hello world!", Some("a"));
assert_eq!(
    node.path,
    vec![72, 101, 108, 108, 111, 32, 119, 111, 114, 108, 100, 33]
);
assert_eq!(node.data.unwrap(), "a");

let node = Node::<u8, &str>::new("Hello 世界！", Some("a"));
assert_eq!(
    node.path,
    vec![72, 101, 108, 108, 111, 32, 228, 184, 150, 231, 149, 140, 239, 188, 129]
);
assert_eq!(node.data.unwrap(), "a");

let node = Node::<char, &str>::new("Hello 世界！", Some("a"));
assert_eq!(
    node.path,
    vec!['H', 'e', 'l', 'l', 'o', ' ', '世', '界', '！']
);
assert_eq!(node.data.unwrap(), "a");

let node = Node::<char, u32>::new("你好，世界！", Some(0));
assert_eq!(node.path, vec!['你', '好', '，', '世', '界', '！']);
assert_eq!(node.data.unwrap(), 0);

let node = Node::<u8, u8>::new("abcde", Some(1));
assert_eq!(node.path, vec![97, 98, 99, 100, 101]);
assert_eq!(node.data.unwrap(), 1);

let node = Node::new("abcde".as_bytes().to_vec(), Some(97));
assert_eq!(node.path, vec![97, 98, 99, 100, 101]);
assert_eq!(node.data.unwrap(), 97);

let node = Node::new("abcde".as_bytes(), Some(97));
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

[radix tree]: https://en.wikipedia.org/wiki/Radix_tree
[rax]: https://github.com/antirez/rax
[httprouter]: https://github.com/julienschmidt/httprouter
[echo]: https://github.com/labstack/echo
[trekjs]: https://github.com/trekjs/router
