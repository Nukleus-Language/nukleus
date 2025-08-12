use crate::neo_tokens::TokenType;
use std::collections::HashMap;

#[derive(Debug)]
pub struct TrieNode {
    ascii_children: Vec<Option<Box<TrieNode>>>,
    unicode_children: Option<HashMap<char, Box<TrieNode>>>,
    is_end: bool,
    token_type: Option<TokenType>,
}

impl TrieNode {
    pub fn new() -> Self {
        let mut ascii_children = Vec::with_capacity(128);
        ascii_children.resize_with(128, || None);

        TrieNode {
            ascii_children,
            unicode_children: None,
            is_end: false,
            token_type: None,
        }
    }

    pub fn insert(&mut self, word: &str, token_type: TokenType) {
        let mut node = self;
        for ch in word.chars() {
            node = if (ch as u32) < 128 {
                node.ascii_children[ch as usize].get_or_insert_with(|| Box::new(TrieNode::new()))
            } else {
                node.unicode_children
                    .get_or_insert_with(HashMap::new)
                    .entry(ch)
                    .or_insert_with(|| Box::new(TrieNode::new()))
            };
        }
        node.is_end = true;
        node.token_type = Some(token_type);
    }

    #[inline]
    pub fn search(&self, word: &str) -> Option<&TokenType> {
        let mut node = self;
        for ch in word.chars() {
            node = if (ch as u32) < 128 {
                node.ascii_children[ch as usize].as_ref()?
            } else {
                node.unicode_children.as_ref()?.get(&ch)?
            };
        }
        node.token_type.as_ref()
    }
}
