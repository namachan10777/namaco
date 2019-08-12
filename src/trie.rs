// copyright (c) 2019 Nakano Masaki <namachan10777@gmail>
use std::usize;

#[derive(Clone, PartialEq, Debug)]
struct Node {
    base: usize,
    check: usize,
    id: usize,
}
impl Default for Node {
    fn default() -> Node {
        Node::from(DecodedNode::default())
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum DecodedNode {
    // base
    Root(usize),
    // check, base, id
    Sec(usize, usize, Option<usize>),
    // check, base
    Term(usize, usize),
    Blank,
}
impl Default for DecodedNode {
    fn default() -> DecodedNode {
        DecodedNode::Blank
    }
}

const MSB: usize = 0x8000000000000000;
const MASK: usize = 0x7fffffffffffffff;
const NO_PARENT: usize = usize::MAX;
const NO_ITEM: usize = usize::MAX;
const NO_CHILD: usize = usize::MAX;

impl Into<DecodedNode> for Node {
    fn into(self) -> DecodedNode {
        if self.check == NO_PARENT {
            if self.base == NO_CHILD {
                DecodedNode::Blank
            }
            else {
                DecodedNode::Root(self.base)
            }
        }
        else if self.base == NO_CHILD {
            DecodedNode::Term(self.check, self.id)
        }
        else {
            if self.id == NO_ITEM {
                DecodedNode::Sec(self.check, self.base, None)
            }
            else {
                DecodedNode::Sec(self.check, self.base, Some(self.id & MASK))
            }
        }
    }
}

impl From<DecodedNode> for Node {
    fn from(dnode: DecodedNode) -> Self {
        match dnode {
            DecodedNode::Root(base) => Node {
                base,
                check: NO_PARENT,
                id: 0,
            },
            DecodedNode::Term(check, id) => Node {
                base: NO_CHILD,
                check,
                id,
            },
            DecodedNode::Sec(check, base, None) => Node {
                base,
                check,
                id: NO_ITEM,
            },
            DecodedNode::Sec(check, base, Some(id)) => Node {
                base,
                check,
                id: id,
            },
            DecodedNode::Blank => Node {
                base: NO_CHILD,
                check: NO_PARENT,
                id: NO_ITEM,
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
            check: NO_PARENT,
            id: 0,
        };
        let term_decoded = DecodedNode::Term(2158, 87);
        let term_raw = Node {
            base: NO_CHILD,
            check: 2158,
            id: 87,
        };
        let sec_no_property_decoded = DecodedNode::Sec(52128, 59182, None);
        let sec_no_property_raw = Node {
            base: 59182,
            check: 52128,
            id: NO_ITEM,
        };
        let sec_has_property_decoded = DecodedNode::Sec(711475, 365123, Some(214));
        let sec_has_property_raw = Node {
            base: 365123,
            check: 711475,
            id: 214,
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
        let mut tree = vec![Node::default(); 256];
        tree[0] = Node::from(DecodedNode::Root(0));
        Trie {
            tree,
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
mod test_explore { 
    use super::*;
    #[test]
    fn test_explore() {
        let mut tree = Vec::new();
        tree.resize(1024, DecodedNode::default());
        /* Root(0): 0 -+- 1 -> Term(0): 1
         *             |
         *             +- 2 -> Sec(0,4): 2 -+- 2 -> Term(2): 6
         *                                  |
         *                                  +- 3 -> Sec(2, 4): 7 -+- 1 -> Term(7): 5
         */
        tree[0] = DecodedNode::Root(0);
        tree[1] = DecodedNode::Term(0, 0);
        tree[2] = DecodedNode::Sec(0, 4, Some(1));
        tree[6] = DecodedNode::Term(2, 2);
        tree[7] = DecodedNode::Sec(2, 4, Some(3));
        tree[5] = DecodedNode::Term(7, 4);
        let trie = Trie {
            tree: tree.iter().map(|elm| Node::from(*elm)).collect(),
            storage: Vec::new() as Vec<String>,
        };
        assert_eq!(trie.explore(&[1]), Ok(1));
        assert_eq!(trie.explore(&[2]), Ok(2));
        assert_eq!(trie.explore(&[2, 2]), Ok(6));
        assert_eq!(trie.explore(&[2, 3]), Ok(7));
        assert_eq!(trie.explore(&[2, 3, 1]), Ok(5));
        assert_eq!(trie.explore(&[3]), Err((0, 0)));
        assert_eq!(trie.explore(&[2, 1, 0]), Err((1, 2)));
        assert_eq!(trie.explore(&[2, 3, 0]), Err((2, 7)));
    }
}

impl<T> Trie<T> {
    fn search_new_base(&mut self, target: &[bool]) -> usize {
        for i in 0..self.tree.len() - 256 {
            let mut safe = true;
            for j in 0..256 {
                safe &= !target[j] || DecodedNode::Blank == Into::<DecodedNode>::into(self.tree[i ^ j].clone());
            }
            if safe {
                return i;
            }
        }
        let half = self.tree.len();
        self.tree.resize(half * 2, Node::default());
        for i in half-1..half + 255 {
            let mut safe = true;
            for j in 0..256 {
                safe &= !target[j] || DecodedNode::Blank == Into::<DecodedNode>::into(self.tree[i ^ j].clone());
            }
            if safe {
                return i;
            }
        }
        half + 256
    }
}

#[cfg(test)]
mod test_search_new_base {
    use super::*;
    #[test]
    fn test_search_new_base() {
        let mut mask = [false; 256];
        mask[0] = true;
        let mut tree = vec![Node::from(DecodedNode::Term(0, 0)); 512];
        tree[6] = Node::default();
        let mut trie: Trie<String> = Trie {
            tree,
            storage: Vec::new(),
        };
        assert_eq!(trie.search_new_base(&mask), 0^6);

        mask[0] = false;
        mask[47] = true;
        assert_eq!(trie.search_new_base(&mask), 6^47);

        mask[47] = true;
        mask[99] = true;
        trie.tree = vec![Node::from(DecodedNode::Blank); 512];
        trie.tree[47] = Node::from(DecodedNode::Term(0, 0));
        trie.tree[1^99] = Node::from(DecodedNode::Term(0, 0));
        assert_eq!(trie.search_new_base(&mask), 2);

        mask[47] = false;
        mask[99] = false;
        mask[0] = true;
        trie.tree = vec![Node::from(DecodedNode::Term(0, 0)); 512];
        trie.tree[511] = Node::from(DecodedNode::default());
        assert_eq!(trie.search_new_base(&mask), 511);
        assert_eq!(trie.tree.len(), 1024);

        trie.tree = vec![Node::from(DecodedNode::Term(0, 0)); 512];
        assert_eq!(trie.search_new_base(&mask), 512);
        assert_eq!(trie.tree.len(), 1024);
    }
}

