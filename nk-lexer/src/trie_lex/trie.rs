use crate::trie_tokens::TokenType;

#[derive(Debug, Eq, PartialEq)]
pub struct TrieNode {
    children: Vec<Option<Box<TrieNode>>>,
    is_end: bool,
    token_type: Option<TokenType>,
}

impl TrieNode {
    fn new() -> Self {
        TrieNode {
            children: Vec::new(),
            is_end: false,
            token_type: None,
        }
    }

    // Add a method to ensure the children vector has the correct capacity
    fn ensure_children_capacity(&mut self) {
        if self.children.len() < 128 {
            self.children.resize_with(128, || None);
        }
    }

    // Update other methods that work with children to use ensure_children_capacity
    fn insert(&mut self, word: &str, token_type: TokenType) {
        self.ensure_children_capacity();
        let mut node = self;
        for &ch in word.as_bytes() {
            node = node.children[ch as usize].get_or_insert_with(|| Box::new(TrieNode::new()));
        }
        node.is_end = true;
        node.token_type = Some(token_type);
    }

    fn get_child(&mut self, c: char) -> Option<&mut Box<TrieNode>> {
        self.ensure_children_capacity();
        self.children[c as usize].as_mut()
    }

    #[inline]
    pub fn search(&self, word: &str) -> Option<&TokenType> {
        let mut node = self;
        for &ch in word.as_bytes() {
            node = node.children[ch as usize].as_ref()?;
        }
        if node.is_end {
            node.token_type.as_ref()
        } else {
            None
        }
    }
}
