// copyright (c) 2019 Nakano Masaki <namachan10777@gmail>
use std::usize;

#[derive(Clone, PartialEq, Debug, Copy)]
struct Node {
    base: usize,
    check: usize,
    id: usize,
}
impl Default for Node {
    fn default() -> Node {
        Node::from(DecodedNode::Blank)
    }
}

impl Node {
    #[allow(dead_code)]
    fn root(base: usize) -> Self {
        Node::from(DecodedNode::Root(base))
    }
    
    fn term(check: usize, id: usize) -> Self {
        Node::from(DecodedNode::Term(check, id))
    }

    
    fn sec(check: usize, base: usize, id: Option<usize>) -> Self {
        Node::from(DecodedNode::Sec(check, base, id))
    }

    fn blank() -> Self {
        Node::default()
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
type Row = [Node; ROW_LEN];

impl<T> Trie<T> {
    #[allow(dead_code)]
    pub fn new() -> Trie<T> {
        let mut tree = vec![Node::blank(); 256];
        tree[0] = Node::root(0);
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
            here = self.tree[here].base ^ (*octet as usize);
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
        tree.resize(1024, Node::blank());
        /* Root(0): 0 -+- 1 -> Term(0): 1
         *             |
         *             +- 2 -> Sec(0,4): 2 -+- 2 -> Term(2): 6
         *                                  |
         *                                  +- 3 -> Sec(2, 4): 7 -+- 1 -> Term(7): 5
         */
        tree[0] = Node::root(0);
        tree[1] = Node::term(0, 0);
        tree[2] = Node::sec(0, 4, Some(1));
        tree[6] = Node::term(2, 2);
        tree[7] = Node::sec(2, 4, Some(3));
        tree[5] = Node::term(7, 4);
        let trie = Trie {
            tree,
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

fn row2mask(row: Row) -> [bool;256] {
    let mut mask = [false; 256];
    for i in 0..256 {
        mask[i] = match Into::<DecodedNode>::into(row[i]) {
            DecodedNode::Term(_, _) => true,
            DecodedNode::Root(_) => true,
            DecodedNode::Sec(_, _, _) => true,
            DecodedNode::Blank => false,
        }
    }
    mask
}
#[cfg(test)]
mod test_row2mask {
    use super::*;
    #[test]
    fn test_row2mask() {
        let mut row = [Node::blank(); 256];
        row[2] = Node::term(0, 0);
        row[9] = Node::sec(0, 0, None);
        row[200] = Node::root(0);
        row[222] = Node::from(DecodedNode::Blank);
        let mut mask = [false; 256];
        mask[2] = true;
        mask[9] = true;
        mask[200] = true;
        assert_eq!(row2mask(row).to_vec(), mask.to_vec());
    }
}

impl<T> Trie<T> {
    fn reallocate_base(&mut self, target: &[bool]) -> usize {
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
        self.tree.resize(half * 2, Node::blank());
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
mod test_reallocate_base {
    use super::*;
    #[test]
    fn test_reallocate_base() {
        let mut mask = [false; 256];
        mask[0] = true;
        let mut tree = vec![Node::term(0, 0); 512];
        tree[6] = Node::blank();
        let mut trie: Trie<String> = Trie {
            tree,
            storage: Vec::new(),
        };
        assert_eq!(trie.reallocate_base(&mask), 0^6);

        mask[0] = false;
        mask[47] = true;
        assert_eq!(trie.reallocate_base(&mask), 6^47);

        mask[47] = true;
        mask[99] = true;
        trie.tree = vec![Node::blank(); 512];
        trie.tree[47] = Node::term(0, 0);
        trie.tree[1^99] = Node::term(0, 0);
        assert_eq!(trie.reallocate_base(&mask), 2);

        mask[47] = false;
        mask[99] = false;
        mask[0] = true;
        trie.tree = vec![Node::term(0, 0); 512];
        trie.tree[511] = Node::blank();
        assert_eq!(trie.reallocate_base(&mask), 511);
        assert_eq!(trie.tree.len(), 1024);

        trie.tree = vec![Node::term(0, 0); 512];
        assert_eq!(trie.reallocate_base(&mask), 512);
        assert_eq!(trie.tree.len(), 1024);
    }
}

impl<T> Trie<T> {
    fn read_row(&self, parent_idx: usize) -> Row {
        let mut buf: Row = [Node::blank(); 256];
        let base = self.tree[parent_idx].base;
        for i in 0..256 {
            if self.tree[base ^ i].check == parent_idx {
                buf[i] = self.tree[base ^ i];
            }
        }
        buf
    }

    fn erase_row(&mut self, parent_idx: usize) {
        let base = self.tree[parent_idx].base;
        for i in 0..256 {
            if self.tree[base ^ i].check == parent_idx {
                self.tree[base ^ i] = Node::blank();
            }
        }
    }
}

#[cfg(test)]
mod test_read_erase_row {
    use super::*;
    #[test]
    fn test_read () {
        let mut tree = [Node::blank(); 512].to_vec();
        tree[0] = Node::root(0);
        tree[1] = Node::sec(0, 64, None);
        tree[2] = Node::term(0, 0);
        tree[64] = Node::term(1, 0);
        let trie: Trie<String> = Trie {
            tree,
            storage: Vec::new(),
        };

        let row1 = trie.read_row(0).to_vec();
        let mut row1_ans = vec![Node::blank();256];
        row1_ans[1] = Node::sec(0, 64, None);
        row1_ans[2] = Node::term(0, 0);
        assert_eq!(row1, row1_ans);

        let row2 = trie.read_row(1).to_vec();
        let mut row2_ans = vec![Node::blank();256];
        row2_ans[0] = Node::term(1, 0);
        assert_eq!(row2, row2_ans)
    }

    #[test]
    fn test_erase () {
        let mut tree = [Node::blank(); 512].to_vec();
        tree[0] = Node::root(0);
        tree[1] = Node::sec(0, 64, None);
        tree[2] = Node::term(0, 0);
        tree[64] = Node::term(1, 0);
        let mut trie: Trie<String> = Trie {
            tree,
            storage: Vec::new(),
        };

        trie.erase_row(0);
        let mut tree1 = [Node::blank(); 512].to_vec();
        tree1[0] = Node::root(0);
        tree1[64] = Node::term(1, 0);
        assert_eq!(trie.tree, tree1);
    }
}

impl<T> Trie<T> {
    // This function forcely overrides tree
    // 存在しなかったのにrowに入っているとfromを誤認する
    fn paste(&mut self, row: Row, addition: Row, from: usize) -> usize {
        let mut mask = [false; 256];
        for i in 0..256 {
            mask[i] = row[i].check != NO_PARENT
                || row[i].base != NO_CHILD
                || addition[i].check != NO_PARENT
                || addition[i].base != NO_CHILD;
        }
        let to = self.reallocate_base(&mask);
        for i in 0..256 {
            if row[i].check != NO_PARENT {
                self.tree[to ^ i] = row[i];
                for j in 0..256 {
                    if row[i].base != NO_CHILD && self.tree[row[i].base ^ j].check == from ^ i {
                        self.tree[row[i].base ^ j].check = to ^ i;
                    }
                }
            }
        }
        for i in 0..256 {
            if addition[i].check != NO_PARENT || addition[i].base != NO_CHILD {
                self.tree[to ^ i] = addition[i];
            }
        }
        to
    }
}

#[allow(dead_code)]
fn decode(x: Vec<Node>) -> Vec<DecodedNode> {
    x.iter().map(|x| Into::<DecodedNode>::into(x.clone())).collect()
}

#[cfg(test)]
mod test_paste {
    // TODO improve test case
    use super::*;
    #[test]
    fn test_paste() {
        let mut tree = vec![Node::blank(); 512];
        tree[0] = Node::root(0);
        tree[64] = Node::term(5, 0);
        
        let mut row = [Node::blank(); 256];
        row[1] = Node::sec(0, 64, None);
        row[2] = Node::term(1, 0);

        let mut trie: Trie<String> = Trie {
            tree,
            storage: Vec::new(),
        };

        assert_eq!(trie.paste(row, [Node::default();256], 4), 0);
        let mut ans = vec![Node::blank(); 512];
        ans[0] = Node::root(0);
        ans[64] = Node::term(1, 0);
        ans[1] = Node::sec(0, 64, None);
        ans[2] = Node::term(1, 0);

        assert_eq!(trie.tree, ans);

        let mut tree2 = vec![Node::blank(); 512];
        tree2[0] = Node::root(0);
        let mut trie2: Trie<String> = Trie {
            tree: tree2,
            storage: Vec::new(),
        };
        let mut row = [Node::blank(); 256];
        row[0] = Node::sec(0, 0, None);
        assert_eq!(trie2.paste([Node::blank(); 256], row, 0), 1);
        let mut ans = vec![Node::blank(); 512];
        ans[0] = Node::root(0);
        ans[1] = Node::sec(0, 0, None);
        assert_eq!(decode(trie2.tree), decode(ans));
    }
}

impl<T> Trie<T> {
    fn push_put(&mut self, target_idx: usize) -> Result<(), ()> {
        if self.tree[target_idx].check == NO_PARENT {
            if self.tree[target_idx].base == NO_CHILD {
                return Ok(())
            }
            else {
                return Err(())
            }
        }
        let parent_idx = self.tree[target_idx].check;
        let row = self.read_row(parent_idx);
        self.erase_row(parent_idx);
        self.tree[target_idx] = Node::term(0, 0);
        let base = self.reallocate_base(&row2mask(row));
        self.paste(row, [Node::blank(); 256], self.tree[parent_idx].base);
        self.tree[parent_idx].base = base;
        self.tree[target_idx] = Node::blank();
        return Ok(())
    }
}
#[cfg(test)]
mod test_push_out {
    use super::*;
    #[test]
    fn test_push_out () {
        let mut tree = vec![Node::default(); 512];
        tree[0] = Node::root(0);
        tree[1] = Node::sec(0, 8, None);
        tree[2] = Node::term(0, 0);
        tree[8] = Node::term(1, 0);
        let mut trie: Trie<String> = Trie {
            tree,
            storage: Vec::new(),
        };
        trie.push_put(1).unwrap();
        let mut ans = vec![Node::default(); 512];
        ans[0] = Node::root(4);
        ans[5] = Node::sec(0, 8, None);
        ans[6] = Node::term(0, 0);
        ans[8] = Node::term(5, 0);
        assert_eq!(decode(trie.tree), decode(ans));
    }
}

impl<T> Trie<T> {
    #[allow(dead_code)]
    pub fn add(&mut self, way: &[u8], cargo: T) -> Result<(), ()> {
        let mut parent_idx = 0;
        let mut extended = false;
        for octet in way {
            let child_idx = self.tree[parent_idx].base ^ (*octet as usize);
            match Into::<DecodedNode>::into(self.tree[child_idx]) {
                DecodedNode::Root(base) => {
                    let row = self.read_row(parent_idx);
                    let mut addition = [Node::blank(); 256];
                    addition[*octet as usize] = Node::sec(parent_idx, 0, None);
                    self.erase_row(parent_idx);
                    let new_base = self.paste(row, addition, base);
                    self.tree[parent_idx].base = new_base;
                    parent_idx = (*octet as usize) ^ new_base;
                    extended = true;
                },
                DecodedNode::Blank => {
                    self.tree[child_idx] = Node::sec(parent_idx, 0, None);
                    extended = true;
                    parent_idx = child_idx;
                },
                DecodedNode::Term(check, id) => {
                    if check == parent_idx {
                        self.tree[child_idx] = Node::sec(check, 0, Some(id));
                        extended = false;
                        parent_idx = child_idx;
                    }
                    else {
                        self.push_put(child_idx)?;
                        self.tree[child_idx] = Node::sec(check, 0, None);
                        extended = true;
                        parent_idx = child_idx;
                    }
                },
                DecodedNode::Sec(check, _, _) => {
                    if check != parent_idx {
                        self.push_put(child_idx)?;
                        self.tree[child_idx] = Node::sec(check, 0, None);
                        extended = true;
                        parent_idx = child_idx;
                    }
                    else {
                        extended = false;
                        parent_idx = child_idx;
                    }
                },
            }
        }
        let cargo_id = self.storage.len();
        self.storage.push(cargo);
        match Into::<DecodedNode>::into(self.tree[parent_idx]) {
            DecodedNode::Blank => unreachable!(),
            DecodedNode::Root(_) => {
                unreachable!();
            },
            DecodedNode::Term(_, _) => unreachable!(),
            DecodedNode::Sec(check, base, _) => {
                if extended {
                    self.tree[parent_idx] = Node::term(check, cargo_id);
                }
                else {
                    self.tree[parent_idx] = Node::sec(check, base, Some(cargo_id));
                }
            }
        }
        Ok(())
    }

    #[allow(dead_code)]
    pub fn find(&self, way: &[u8]) -> Result<&T, ()> {
        match self.explore(way) {
            Ok(idx) => {
                match Into::<DecodedNode>::into(self.tree[idx]) {
                    DecodedNode::Blank => Err(()),
                    DecodedNode::Root(_) => Err(()),
                    DecodedNode::Term(_, id) => Ok(&self.storage[id]),
                    DecodedNode::Sec(_, _, Some(id)) => Ok(&self.storage[id]),
                    DecodedNode::Sec(_, _, None) => Err(()),
                }
            },
            Err(_) => Err(())
        }
    }
}

#[cfg(test)]
mod test_add_find {
    use super::*;
    #[test]
    fn test_add_fin() {
        let mut trie: Trie<String> = Trie::new();
        trie.add(&[1, 2, 3], "123".to_string()).unwrap();
        trie.add(&[0], "0".to_string()).unwrap();
        trie.add(&[1, 2], "12".to_string()).unwrap();
        trie.add(&[1, 2, 0], "120".to_string()).unwrap();
        trie.add(&[0, 1], "01".to_string()).unwrap();
        assert_eq!(trie.find(&[0]), Ok(&"0".to_string()));
        assert_eq!(trie.find(&[1, 2, 3]), Ok(&"123".to_string()));
        assert_eq!(trie.find(&[1, 2]), Ok(&"12".to_string()));
        assert_eq!(trie.find(&[1, 2, 0]), Ok(&"120".to_string()));
        assert_eq!(trie.find(&[0, 1]), Ok(&"01".to_string()));
    }
}
