#![no_std]
extern crate alloc;

#[allow(unused_imports)] // ambiguity between `vec!` macro and `vec` module doesn't sit well with compiler
use alloc::vec;
use alloc::vec::Vec;
use alloc::borrow::ToOwned;
use core::mem;

const fn pos<K>(l: &usize, _: &K, _: &Vec<K>) -> usize {
    *l
}

pub trait Vectorable<K>
where
    K: Copy + PartialEq + PartialOrd,
{
    fn into(&self) -> Vec<K>;
}

#[macro_use]
pub mod macros;

pub trait Radix<K, V, P: Vectorable<K>>
where
    K: Copy + PartialEq + PartialOrd,
    V: Clone,
{
    fn remove(&mut self, path: P);
    fn insert(&mut self, path: P, data: V) -> &mut Self;
    fn find(&self, path: P) -> Option<&Self>;
    fn add_node(&mut self, path: P, data: V) -> &mut Self;
    fn find_node(&self, path: P) -> Option<&Self>;
}

#[derive(Debug, Clone, PartialEq)]
pub struct Node<K, V> {
    pub path: Vec<K>,
    pub data: Option<V>,
    pub indices: Vec<K>,
    pub nodes: Vec<Node<K, V>>,
}

impl<K, V> Node<K, V>
where
    K: Copy + PartialEq + PartialOrd,
    V: Clone,
{
    pub fn new<P: Vectorable<K>>(path: P, data: Option<V>) -> Self {
        Node {
            data,
            path: (&path).into(),
            nodes: Vec::new(),
            indices: Vec::new(),
        }
    }

    pub fn insert_with<F>(
        &mut self,
        path: &mut Vec<K>,
        data: Option<V>,
        force_update: bool,
        pos: F,
    ) -> &mut Self
    where
        F: Fn(&usize, &K, &Vec<K>) -> usize,
    {
        let pl = path.len();
        let sl = self.path.len();

        // empty input path
        if 0 == pl {
            if force_update {
                self.data = data;
            }
            return self;
        }

        // empty node
        if 0 == sl && 0 == self.indices.len() {
            if force_update {
                self.data = data;
            }
            self.path = path.to_owned();
            return self;
        }

        // pl > 0 && sl >= 0
        let max = pl.min(sl);
        let mut i = 0;
        while i < max && path[i] == self.path[i] {
            i += 1;
        }

        if i < sl {
            let child = Node {
                data: self.data.take(),
                path: self.path.split_off(i),
                nodes: mem::replace(&mut self.nodes, Vec::new()),
                indices: mem::replace(&mut self.indices, Vec::new()),
            };
            let c = child.path[0];
            let index = pos(&self.indices.len(), &c, &self.indices);
            self.indices.insert(index, c);
            self.nodes.insert(index, child);

            // self.indices.push(child.path[0]);
            // self.nodes.push(child);
        }

        if i == pl {
            if force_update {
                self.data = data;
            }
            return self;
        }

        self.add_node_with(path, data, i, force_update, pos)
    }

    pub fn add_node_with<F>(
        &mut self,
        path: &mut Vec<K>,
        data: Option<V>,
        i: usize,
        force_update: bool,
        pos: F,
    ) -> &mut Self
    where
        F: Fn(&usize, &K, &Vec<K>) -> usize,
    {
        let l = self.indices.len();
        let c = path[i];
        let mut j = 0;
        while j < l {
            if c == self.indices[j] {
                return self.nodes[j].insert_with(&mut path.split_off(i), data, force_update, pos);
            }
            j += 1;
        }

        let index = pos(&l, &c, &self.indices);
        self.indices.insert(index, c);
        self.nodes.insert(
            index,
            Node {
                data,
                nodes: Vec::new(),
                indices: Vec::new(),
                path: path.split_off(i),
            },
        );

        &mut self.nodes[index]

        // self.indices.push(c);
        // self.nodes.push(Node {
        //     data,
        //     nodes: Vec::new(),
        //     indices: Vec::new(),
        //     path: path.split_off(i),
        // });
        // &mut self.nodes[l]
    }

    pub fn find_with(&self, path: &mut Vec<K>) -> Option<&Self> {
        let pl = path.len();
        let sl = self.path.len();

        // "abc" < "abcde"
        // not found
        if pl < sl {
            return None;
        }

        // "abcde" > "abc" or "abc" == "abc"
        let mut i = 0;
        while i < sl && path[i] == self.path[i] {
            i += 1;
        }

        // "abc" == "abc"
        if pl == sl {
            if i == pl {
                return Some(self);
            }
            // not found
            return None;
        }

        // "abcde" > "abc"
        self.find_node_with(path, i)
    }

    pub fn find_node_with(&self, path: &mut Vec<K>, i: usize) -> Option<&Self> {
        let l = self.indices.len();
        let c = path[i];
        let mut j = 0;
        while j < l {
            if c == self.indices[j] {
                return self.nodes[j].find_with(&mut path.split_off(i));
            }
            j += 1;
        }
        // not found
        None
    }
}

impl<K, V, P: Vectorable<K>> Radix<K, V, P> for Node<K, V>
where
    K: Copy + PartialEq + PartialOrd,
    V: Clone,
{
    #[allow(unused_variables)]
    fn remove(&mut self, path: P) {}

    fn insert(&mut self, path: P, data: V) -> &mut Self {
        self.insert_with(&mut (&path).into(), Some(data), true, pos)
    }

    fn find(&self, path: P) -> Option<&Self> {
        self.find_with(&mut (&path).into())
    }

    fn add_node(&mut self, path: P, data: V) -> &mut Self {
        self.add_node_with(&mut (&path).into(), Some(data), 0, true, pos)
    }

    fn find_node(&self, path: P) -> Option<&Self> {
        self.find_node_with(&mut (&path).into(), 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! find {
        ($tree:expr, $($path:expr, $data:expr),*,) => {{
            $(
                let node = $tree.find($path);
                assert_eq!(node.is_some(), $data);
                if node.is_some() {
                    assert_eq!(node.unwrap().data.unwrap(), $data);
                }
            )*
        }};
    }

    macro_rules! insert_and_find {
        ($tree:expr, $($path:expr, $data:expr),*,) => {{
            $(
                $tree.insert($path, $data);
                find!($tree, $path, $data,);
            )*
        }};
    }

    #[test]
    fn new_any_type_node() {
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
    }

    #[test]
    fn node_insert_and_find() {
        let mut node = Node::<char, bool>::new("你好，世界！", Some(true));
        node.add_node("Rust", true);

        let n1 = node.find_node("Rust");
        let n2 = node.find("你好，世界！Rust");
        assert_eq!(n1, n2);
    }

    #[test]
    fn node_insert_then_return_new_node() {
        let mut tree = Node::<u8, u8>::new("", Some(b' '));

        let a = tree.insert("a", b'a');
        let b = a.add_node("b", b'b');
        let c = b.add_node("c", b'c');
        let d = c.add_node("d", b'd');
        let _ = d.add_node("e", b'e');

        // println!("{:#?}", tree);

        let node = tree.find("a");
        assert_eq!(node.is_some(), true);
        let a = node.unwrap();
        assert_eq!(a.data.unwrap(), b'a');

        let node = a.find_node("b");
        assert_eq!(node.is_some(), true);
        let b = node.unwrap();
        assert_eq!(b.data.unwrap(), b'b');

        let node = b.find_node("c");
        assert_eq!(node.is_some(), true);
        let c = node.unwrap();
        assert_eq!(c.data.unwrap(), b'c');

        let node = c.find_node("d");
        assert_eq!(node.is_some(), true);
        let d = node.unwrap();
        assert_eq!(d.data.unwrap(), b'd');

        let node = d.find_node("e");
        assert_eq!(node.is_some(), true);
        let e = node.unwrap();
        assert_eq!(e.data.unwrap(), b'e');

        let node = a.find("abcde");
        assert_eq!(node.is_some(), true);
        assert_eq!(node.unwrap().data.unwrap(), b'e');

        let node = tree.find("abcdef");
        assert_eq!(node.is_some(), false);

        let node = tree.find("b");
        assert_eq!(node.is_some(), false);
    }

    #[test]
    fn new_tree() {
        let mut tree = Node::<char, bool>::new("", Some(false));

        insert_and_find!(
            tree,
            "alligator",
            true,
            "alien",
            true,
            "baloon",
            true,
            "chromodynamic",
            true,
            "romane",
            true,
            "romanus",
            true,
            "romulus",
            true,
            "rubens",
            true,
            "ruber",
            true,
            "rubicon",
            true,
            "rubicundus",
            true,
            "all",
            true,
            "rub",
            true,
            "ba",
            true,
            "你好，世界",
            true,
            "你好",
            true,
            "你",
            true,
        );

        find!(
            tree, "rpxxx", false, "chro", false, "chromz", false, "zorro", false, "ro", false,
            "zo", false,
        );

        let node = tree.find("");
        assert_eq!(node.is_some(), true);
        assert_eq!(node.unwrap().data, None);

        tree.insert("", false);
        let node = tree.find("");
        assert_eq!(node.is_some(), true);
        assert_eq!(node.unwrap().data.unwrap(), false);

        let node = tree.find("all");
        assert_eq!(node.is_some(), true);
        assert_eq!(node.unwrap().data.unwrap(), true);

        let node = tree.find("dota2");
        assert_eq!(node.is_none(), true);

        let node = tree.find("你");
        assert_eq!(node.is_some(), true);
        assert_eq!(node.unwrap().data.unwrap(), true);

        let node = tree.find("你好");
        assert_eq!(node.is_some(), true);
        assert_eq!(node.unwrap().data.unwrap(), true);

        let node = tree.find("语言");
        assert_eq!(node.is_some(), false);

        let node = tree.find("你好，世界");
        assert_eq!(node.is_some(), true);

        let node = tree.find("你好，世界 Rust");
        assert_eq!(node.is_some(), false);

        log::info!("{:#?}", tree);
    }

    #[test]
    fn clone_node() {
        let mut node = Node::<char, bool>::new("", Some(false));
        let mut node2 = node.clone();
        assert_eq!(node, node2);

        node.add_node("/", true);
        node2.add_node("/", true);
        assert_eq!(node, node2);

        #[derive(Debug, Clone, PartialEq)]
        struct NodeMetadata {
            is_key: bool,
            params: Option<Vec<&'static str>>,
        }

        let mut node = Node::<char, NodeMetadata>::new(
            "/",
            Some(NodeMetadata {
                is_key: false,
                params: Some(vec![]),
            }),
        );
        let mut node2 = node.clone();
        assert_eq!(node, node2);

        node.add_node(
            "users",
            NodeMetadata {
                is_key: true,
                params: Some(vec!["tree"]),
            },
        );
        node2.add_node(
            "users",
            NodeMetadata {
                is_key: true,
                params: Some(vec!["tree"]),
            },
        );
        assert_eq!(node, node2);
    }
}
