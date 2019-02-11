# radix-tree

A [radix tree] implementation for router, path search.

[![Build Status](https://travis-ci.com/trek-rs/radix-tree.svg?branch=master)](https://travis-ci.com/trek-rs/radix-tree)
[![Latest version](https://img.shields.io/crates/v/radix-tree.svg)](https://crates.io/crates/radix-tree)
[![Documentation](https://docs.rs/radix-tree/badge.svg)](https://docs.rs/radix-tree)
![License](https://img.shields.io/crates/l/radix-tree.svg)

## Usage

```rust
use radix_tree::Node;

let mut tree = Node::<bool>::new("", false);

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
