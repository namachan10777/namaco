// copyright (c) 2019 Nakano Masaki <namachan10777@gmail>
use std::usize;

#[derive(Clone, PartialEq, Debug)]
struct Node {
    base: usize,
    check: usize,
    id: usize,
}

#[derive(Debug, PartialEq, Clone)]
enum DecodedNode {
    // base
    Root(usize),
    // check, base, id
    Sec(usize, usize, Option<usize>),
    // check, base
    Term(usize, usize),
}

const STAND1: usize = 0x8000000000000000;
const STAND2: usize = 0x4000000000000000;
const MASK: usize = 0x3fffffffffffffff;

impl Into<DecodedNode> for Node {
    fn into(self) -> DecodedNode {
        // 00 -> Root
        // 01 -> Term
        // 10 -> Sec
        // 11 -> Sec (with data)
        return if self.id & STAND1 == STAND1 {
            if self.id & STAND2 == STAND2 {
                DecodedNode::Sec(self.check, self.base, Some(self.id & MASK))
            }
            else {
                DecodedNode::Sec(self.check, self.base, None)
            }
        }
        else {
            if self.id & STAND2 == STAND2 {
                DecodedNode::Term(self.check, self.id & MASK)
            }
            else {
                DecodedNode::Root(self.base)
            }
        }
    }
}

impl From<DecodedNode> for Node {
    fn from(dnode: DecodedNode) -> Self {
        match dnode {
            DecodedNode::Root(base) => Node {
                base,
                check: 0,
                id: 0,
            },
            DecodedNode::Term(check, id) => Node {
                base: 0,
                check,
                id: STAND2 | id,
            },
            DecodedNode::Sec(check, base, None) => Node {
                base,
                check,
                id: STAND1,
            },
            DecodedNode::Sec(check, base, Some(id)) => Node {
                base,
                check,
                id: STAND1 | STAND2 | id,
            }
        }
    }
}

#[cfg(test)]
mod node_test {
    use super::*;

    #[test]
    fn test_from_into() {
        let root_decoded = DecodedNode::Root(129);
        let root_raw = Node {
            base: 129,
            check: 0,
            id: 0,
        };
        let term_decoded = DecodedNode::Term(2158, 87);
        let term_raw = Node {
            base: 0,
            check: 2158,
            id: STAND2 | 87,
        };
        let sec_no_property_decoded = DecodedNode::Sec(52128, 59182, None);
        let sec_no_property_raw = Node {
            base: 59182,
            check: 52128,
            id: STAND1,
        };
        let sec_has_property_decoded = DecodedNode::Sec(711475, 365123, Some(214));
        let sec_has_property_raw = Node {
            base: 365123,
            check: 711475,
            id: STAND1 | STAND2 | 214,
        };
        assert_eq!(Node::from(root_decoded.clone()), root_raw);
        assert_eq!(Into::<DecodedNode>::into(root_raw), root_decoded);

        assert_eq!(Node::from(term_decoded.clone()), term_raw);
        assert_eq!(Into::<DecodedNode>::into(term_raw), term_decoded);

        assert_eq!(Node::from(sec_no_property_decoded.clone()), sec_no_property_raw);
        assert_eq!(Into::<DecodedNode>::into(sec_no_property_raw), sec_no_property_decoded);

        assert_eq!(Node::from(sec_has_property_decoded.clone()), sec_has_property_raw);
        assert_eq!(Into::<DecodedNode>::into(sec_has_property_raw), sec_has_property_decoded);
    }
}

struct Trie<T> {
    // 圧縮済みの遷移表
    tree: Vec<Node>,
    // 辞書本体
    storage: Vec<T>,
}

const ROW_LEN: usize = 256;
type Row<T> = [T; ROW_LEN];

impl<T> Trie<T> {
    fn new() -> Trie<T> {
        Trie {
            tree: Vec::new(),
            storage: Vec::new(),
        }
    }
}

impl<T> Trie<T> {
    // Ok(idx)
    // Err((passed times, last idx))
    fn explore(&self, way: &[u8]) -> Result<usize, (usize, usize)> {
        let mut here = 0usize;
        let mut octet_count = 0usize;
        for octet in way {
            let check = here;
            here = self.tree[here].base + (*octet as usize);
            if self.tree[here].check != check {
                return Err((octet_count, check))
            }
            octet_count += 1;
        }
        Ok(here)
    }
}

#[cfg(test)]
mod pursue_test {
    fn test_pursue() {
        let trie = Trie::new();
        let tree = Vec::new();
        tree.resize(1024, 
    }
}
