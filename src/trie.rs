// copyright (c) 2019 Nakano Masaki <namachan10777@gmail>
use std::usize;

#[derive(Clone)]
struct Node {
    base: usize,
    check: usize,
    id: usize,
}

enum DecodedNode {
    Root(usize),
    Sec(usize, usize, Option<usize>),
    Term(usize, usize),
}

impl Node {
    fn decode(&self) -> DecodedNode {
        // 00 -> Root
        // 01 -> Term
        // 10 -> Sec
        // 11 -> Sec (with data)
        let stand1 = 0x8000000000000000;
        let stand2 = 0x4000000000000000;
        return if self.id & stand1 == stand1 {
            if self.id & stand2 == stand2 {
                DecodedNode::Term(self.check, self.id & 0x2fffffffffffffff)
            }
            else {
                DecodedNode::Root(self.base)
            }
        }
        else {
            if self.id & stand2 == stand2 {
                DecodedNode::Sec(self.base, self.check, Some(self.id & 0x2fffffffffffffff))
            }
            else {
                DecodedNode::Sec(self.base, self.check, None)
            }
        }
    }
}

struct Trie<T> {
    // 圧縮済みの遷移表
    tree: Vec<Node>,
    // 辞書本体
    storage: Vec<T>,
}

impl<T> Trie<T> {
}
