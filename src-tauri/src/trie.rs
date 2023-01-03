pub mod trie {
    use std::collections::HashMap;

    pub struct Trie {
        pub root: Node,
    }

    struct Node {
        children: HashMap<char, Node>,
        is_word: bool,
    }

    impl Trie {
        pub fn new() -> Self {
            Self {
                root: Node {
                    children: HashMap::new(),
                    is_word: false,
                },
            }
        }

        pub fn insert(&mut self, word: &str) {
            let mut current = &mut self.root;
            for c in word.chars() {
                current = current.children.entry(c).or_insert(Node {
                    children: HashMap::new(),
                    is_word: false,
                });
            }
            current.is_word = true;
        }

        pub fn contains(&self, word: &str) -> bool {
            let mut current = &self.root;
            for c in word.chars() {
                match current.children.get(&c) {
                    Some(node) => current = node,
                    None => return false,
                }
            }
            current.is_word
        }

        pub fn print_nodes(&self) {
            self.print_nodes_helper(&self.root, 0);
        }

        pub fn print_nodes_helper(&self, node: &Node, depth: usize) {
            for (c, node) in &node.children {
                println!("{}{}", " ".repeat(depth), c);
                self.print_nodes_helper(node, depth + 1);
            }
        }
    }
}
