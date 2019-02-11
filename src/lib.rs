use std::mem;

pub trait Radix<T> {
    fn insert(&mut self, path: &str, data: T);
    fn find(&self, path: &str) -> Option<&Node<T>>;
    fn remove(&mut self, path: &str);
}

#[derive(Debug)]
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

    pub fn insert_with(&mut self, path: &mut Vec<u8>, data: T) {
        let sl = path.len();
        let pl = self.path.len();
        let max = sl.min(pl);

        if (max | pl | self.indices.len()) == 0 {
            self.path = path.to_vec();
            self.data = Some(data);
            return;
        }

        let mut i = 0;
        while i < max && path[i] == self.path[i] {
            i += 1;
        }

        // Split Node
        if i < pl {
            let mut child = Node {
                nodes: Vec::new(),
                indices: Vec::new(),
                data: self.data.take(),
                path: self.path.split_off(i),
            };

            mem::swap(&mut self.nodes, &mut child.nodes);
            mem::swap(&mut self.indices, &mut child.indices);

            let c = child.path[0];
            self.data = None;
            self.indices.push(c);
            self.nodes.push(child);
        }

        if i == sl {
            self.data = Some(data);
        } else {
            // New Node
            let mut path = path.split_off(i);
            let c = path[0];

            let mut k = 0;
            while k < self.indices.len() {
                if c == self.indices[k] {
                    return self.nodes[k].insert_with(&mut path, data);
                }
                k += 1;
            }

            self.indices.push(c);
            self.nodes.push(Node {
                path,
                data: Some(data),
                nodes: Vec::new(),
                indices: Vec::new(),
            });
        }
    }

    pub fn find_with(&self, path: &mut Vec<u8>) -> Option<&Node<T>> {
        let sl = path.len();
        let pl = self.path.len();

        if sl < pl {
            return None;
        }

        let mut i = 0;
        while i < pl && path[i] == self.path[i] {
            i += 1;
        }

        if i == sl && sl == pl {
            return Some(self);
        }

        let mut path = path.split_off(i);
        let c = path[0];

        let mut k = 0;
        while k < self.indices.len() {
            if c == self.indices[k] {
                return self.nodes[k].find_with(&mut path);
            }
            k += 1;
        }

        None
    }
}

impl<T> Radix<T> for Node<T> {
    fn insert(&mut self, path: &str, data: T) {
        let mut path = path.as_bytes().to_vec();
        self.insert_with(&mut path, data);
    }

    fn find(&self, path: &str) -> Option<&Node<T>> {
        let mut path = path.as_bytes().to_vec();
        self.find_with(&mut path)
    }

    #[allow(unused_variables)]
    fn remove(&mut self, path: &str) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    impl<T> Node<T> {
        pub fn print_nodes(&self, i: usize) {
            if self.nodes.len() == 0 {
                let s = format!(
                    "{}`{}-({}) [{}] --> [{}] []={}",
                    " ".repeat(i),
                    String::from_utf8(self.path.to_owned()).unwrap(),
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
                    "{}`{}-({}) [{}] --> [{}] []={}",
                    " ".repeat(c),
                    String::from_utf8(self.path.to_owned()).unwrap(),
                    *k as char,
                    String::from_utf8(n.path[1..].to_vec()).unwrap(),
                    String::from_utf8(n.indices.to_vec()).unwrap(),
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
    }
}
