use std::mem;

pub trait Radix<T> {
    fn remove(&mut self, path: &str);
    fn insert(&mut self, path: &str, data: T) -> &mut Self;
    fn find(&self, path: &str) -> Option<&Self>;
    fn add_node(&mut self, path: &str, data: T) -> &mut Self;
    fn find_node(&self, path: &str) -> Option<&Self>;
}

#[derive(Clone, Debug)]
pub struct Node<T> {
    pub path: Vec<u8>,
    pub data: Option<T>,
    pub indices: Vec<u8>,
    pub nodes: Vec<Node<T>>,
}

impl<T> Node<T> {
    pub fn new(path: &str, data: T) -> Node<T> {
        Node {
            data: Some(data),
            nodes: Vec::new(),
            indices: Vec::new(),
            path: path.as_bytes().to_vec(),
        }
    }

    pub fn insert_with(
        &mut self,
        path: &mut Vec<u8>,
        data: Option<T>,
        force_update: bool,
    ) -> &mut Self {
        let pl = path.len();
        let sl = self.path.len();

        // empty input path
        if pl == 0 {
            if force_update {
                self.data = data;
            }
            return self;
        }

        // empty node
        if sl == 0 && self.indices.len() == 0 {
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
            self.indices.push(child.path[0]);
            self.nodes.push(child);
        }

        if i == pl {
            if force_update {
                self.data = data;
            }
            return self;
        }

        self.add_node_with(path, data, i, force_update)
    }

    pub fn find_with(&self, path: &mut Vec<u8>) -> Option<&Self> {
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

    pub fn add_node_with(
        &mut self,
        path: &mut Vec<u8>,
        data: Option<T>,
        i: usize,
        force_update: bool,
    ) -> &mut Self {
        let l = self.indices.len();
        let c = path[i];
        let mut j = 0;
        while j < l {
            if c == self.indices[j] {
                return self.nodes[j].insert_with(&mut path.split_off(i), data, force_update);
            }
            j += 1;
        }

        self.indices.push(c);
        self.nodes.push(Node {
            data,
            nodes: Vec::new(),
            indices: Vec::new(),
            path: path.split_off(i),
        });

        &mut self.nodes[l]
    }

    pub fn find_node_with(&self, path: &mut Vec<u8>, i: usize) -> Option<&Self> {
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

impl<T> Radix<T> for Node<T> {
    #[allow(unused_variables)]
    fn remove(&mut self, path: &str) {}
    fn insert(&mut self, path: &str, data: T) -> &mut Self {
        self.insert_with(&mut path.as_bytes().to_owned(), Some(data), true)
    }
    fn find(&self, path: &str) -> Option<&Self> {
        self.find_with(&mut path.as_bytes().to_owned())
    }
    // Add node to parent
    fn add_node(&mut self, path: &str, data: T) -> &mut Self {
        self.add_node_with(&mut path.as_bytes().to_owned(), Some(data), 0, true)
    }
    // Find node from parent
    fn find_node(&self, path: &str) -> Option<&Self> {
        self.find_node_with(&mut path.as_bytes().to_owned(), 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl<T> Node<T> {
        pub fn print_nodes(&self, i: usize) {
            if self.nodes.len() == 0 {
                let s = format!(
                    "{}`{:?}-({}) [{}] --> [{}] []={}",
                    " ".repeat(i),
                    self.path.to_vec(),
                    "",
                    "",
                    "",
                    self.data.is_some(),
                );
                println!("{}", s);
                return;
            }

            for (j, k) in self.indices.iter().enumerate() {
                let n = &self.nodes[j];
                let mut c = i;
                let s = format!(
                    "{}`{:?}-({}) {:?} --> {:?} []={}",
                    " ".repeat(c),
                    self.path.to_vec(),
                    *k as u32,
                    n.path[1..].to_vec(),
                    n.indices.to_vec(),
                    n.data.is_some(),
                );
                c += s.len() - 5 - c;
                println!("{}", s);
                for m in &n.nodes {
                    m.print_nodes(c);
                }
            }
        }
    }

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
    fn new_node() {
        let mut tree = Node::<bool>::new("", false);

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

        // println!("{:#?}", tree);

        tree.print_nodes(0);

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
    }

    #[test]
    fn insert_then_return_new_node() {
        let mut tree = Node::<u8>::new("", b' ');

        let a = tree.insert("a", b'a');
        let b = a.add_node("b", b'b');
        let c = b.add_node("c", b'c');
        let d = c.add_node("d", b'd');
        let e = d.add_node("e", b'e');

        e.print_nodes(0);
        d.print_nodes(0);
        c.print_nodes(0);
        b.print_nodes(0);
        a.print_nodes(0);

        tree.print_nodes(0);

        println!("{:#?}", tree);

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
}
