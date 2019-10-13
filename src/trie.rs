// copyright (c) 2019 Nakano Masaki <namachan10777@gmail>
use std::usize;
use serde_derive::{Serialize, Deserialize};
use serde::{Serialize};

#[derive(Clone, PartialEq, Debug, Copy, Serialize, Deserialize)]
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
    fn root(base: usize) -> Self {
        Node::from(DecodedNode::Root(base))
    }
    
    fn term(check: usize, id: usize) -> Self {
        Node::from(DecodedNode::Term(check, id))
    }

    // for test
    #[allow(dead_code)]
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
                DecodedNode::Sec(self.check, self.base, Some(self.id))
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

#[derive(Serialize, Deserialize)]
pub struct Trie<T: Serialize> {
    capacities: Vec<u8>,
    cache: Vec<usize>,
    // 圧縮済みの遷移表
    tree: Vec<Node>,
    // 辞書本体
    storage: Vec<Vec<T>>,
}

const ROW_LEN: usize = 256;

impl<T: Serialize> Trie<T> {
    pub fn new() -> Trie<T> {
        let mut tree = vec![Node::blank(); ROW_LEN];
        tree[0] = Node::root(0);
        Trie {
            cache: vec![0;ROW_LEN],
            capacities: vec![254],
            tree,
            storage: Vec::new(),
        }
    }
}

impl<T: Serialize> Trie<T> {
    // Ok(idx)
    // Err((passed times, last idx))
    fn explore(&self, way: &[u8]) -> Result<usize, (usize, usize)> {
        let mut here = 0usize;
        let mut octet_count = 0usize;
        for octet in way {
            let check = here;
            if self.tree[here].base == NO_CHILD {
                return Err((octet_count, check))
            }
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
            cache: vec![0;ROW_LEN],
            capacities: vec![249, 255],
            tree,
            storage: Vec::new() as Vec<Vec<String>>,
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

impl<T: Serialize> Trie<T> {
    // To reallocate base and expand tree if need to do.
    // FIXME bottleneck
    fn reallocate_base(&mut self, target: &[bool; ROW_LEN], cnt: u8) -> usize {
        for block_idx in self.cache[cnt as usize]..self.capacities.len() {
            if self.capacities[block_idx] >= cnt {
                for innser_offset in 0..ROW_LEN {
                    let mut safe = true;
                    let offset = (block_idx << 8) | innser_offset;
                    for target_idx in 0..ROW_LEN {   
                        if target[target_idx] && DecodedNode::Blank != Into::<DecodedNode>::into(self.tree[offset ^ target_idx].clone()) {
                            safe = false; 
                            break;                   
                        }
                    }
                    if safe {
                        for i in (cnt as usize)..self.cache.len() {
                            self.cache[i] = std::cmp::max(self.cache[i], block_idx);
                        }
                        return offset;
                    }
                }
            }
        }
        let half = self.tree.len();
        // expand tree
        self.tree.resize(half * 2, Node::blank());
        self.capacities.resize(self.capacities.len() * 2, 255);
        // search base straddling border of allocated area.
        half
    }
}

#[cfg(test)]
mod test_reallocate_base {
    use super::*;
    #[test]
    fn test_reallocate_base() {
        let mut mask = [false; ROW_LEN];
        mask[0] = true;
        let mut tree = vec![Node::term(0, 0); 512];
        tree[6] = Node::blank();
        let mut trie: Trie<String> = Trie {
            cache: vec![0;ROW_LEN],
            capacities: vec![1, 0],
            tree,
            storage: Vec::new(),
        };
        assert_eq!(trie.reallocate_base(&mask, 1), 0^6);

        mask[0] = false;
        mask[47] = true;
        assert_eq!(trie.reallocate_base(&mask, 1), 6^47);

        mask[47] = true;
        mask[99] = true;
        trie.tree = vec![Node::blank(); 512];
        trie.tree[47] = Node::term(0, 0);
        trie.tree[1^99] = Node::term(0, 0);
        trie.capacities = vec![253, 255];
        assert_eq!(trie.reallocate_base(&mask, 2), 2);

        mask[47] = false;
        mask[99] = false;
        mask[0] = true;
        trie.tree = vec![Node::term(0, 0); 512];
        trie.tree[511] = Node::blank();
        trie.capacities = vec![0, 1];
        assert_eq!(trie.reallocate_base(&mask, 1), 511);
        assert_eq!(trie.tree.len(), 512);

        trie.tree = vec![Node::term(0, 0); 512];
        trie.capacities = vec![0, 0];
        assert_eq!(trie.reallocate_base(&mask, 1), 512);
        assert_eq!(trie.tree.len(), 1024);
    }
}

impl<T: Serialize> Trie<T> {
    pub fn find(&self, way: &[u8]) -> Result<&[T], ()> {
        match self.explore(way) {
            Ok(idx) => {
                match Into::<DecodedNode>::into(self.tree[idx]) {
                    DecodedNode::Blank => Err(()),
                    DecodedNode::Root(_) => Err(()),
                    DecodedNode::Term(_, id) => Ok(&self.storage[id][..]),
                    DecodedNode::Sec(_, _, Some(id)) => Ok(&self.storage[id][..]),
                    DecodedNode::Sec(_, _, None) => Err(()),
                }
            },
            Err(_) => Err(())
        }
    }
}

impl<T: Serialize + Clone> Trie<T> {
    fn sort_dict(src: &mut Vec<(&[u8], T)>) {
        src.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    }

    // expect sorted
    fn get_domain<'a, 'b>(src: &'a [(&'b [u8], T)], select: usize, target: u8) -> &'a [(&'b [u8], T)] {
        let mut begin = None;
        let mut end = None;

        for i in 0..src.len() {
            if src[i].0[select] == target && src[i].0.len() > select + 1 {
                if begin == None {
                    begin = Some(i);
                }
            }
            if begin != None && end == None && src[i].0[select] != target {
                end = Some(i);
            }
        }
        match (begin, end) {
            (Some(begin), Some(end)) => &src[begin..end],
            (Some(begin), None) => &src[begin..],
            _ => unreachable!()
        }
    }

    pub fn add_static(&mut self, src: &[(&[u8], T)], select: usize, parent_idx: usize) -> usize {
        let mut row = [Node::default(); ROW_LEN];
        let mut mask = [false; ROW_LEN];
        let mut update = [false;ROW_LEN];
        let mut before = -1i16;
        let mut cnt = 0u8;

        for (way, cargo) in src {
            mask[way[select] as usize] = true;
            if way[select] as i16 > before {
                cnt += 1;
                before = way[select] as i16;
                row[way[select] as usize] = Node::term(parent_idx, NO_ITEM);
            }
            if way.len() <= select + 1 {
                if row[way[select] as usize].id != NO_ITEM {
                    self.storage[row[way[select] as usize].id as usize].push(cargo.clone());
                }
                else {
                    self.storage.push(vec![cargo.clone()]);
                    row[way[select] as usize].id = self.storage.len() - 1;
                }
            }
            else {
                update[way[select] as usize] = true;
            }
        }

        let base = self.reallocate_base(&mask, cnt);
        for i in 0..ROW_LEN {
            if mask[i] {
                self.tree[i ^ base] = row[i];
            }
        }
        for i in 0..ROW_LEN {
            if update[i] {
                let idx = i ^ base;
                self.tree[idx].base = self.add_static(Self::get_domain(src, select, i as u8), select+1, idx);
            }
        }
        base
    }

    pub fn static_construction(src: &mut Vec<(&[u8], T)>) -> Trie<T> {
        Trie::sort_dict(src);
        let mut trie = Trie::new();
        trie.tree[0] = Node::root(0);
        trie.tree[0].base = trie.add_static(src, 0, 0);
        trie
    }
}
#[cfg(test)]
mod test_static_construction {
    use super::*;
    #[test]
    fn test_sort() {
        let mut dict = vec![
            (&[1, 2, 3][..], "123"),
            (&[1, 2, 3, 5][..], "1235"),
            (&[1, 2, 3, 4][..], "1234"),
            (&[1, 2, 3][..], "123"),
        ];
        Trie::sort_dict(&mut dict);
        assert_eq!(dict, vec![
            (&[1, 2, 3][..], "123"),
            (&[1, 2, 3][..], "123"),
            (&[1, 2, 3, 4][..], "1234"),
            (&[1, 2, 3, 5][..], "1235"),
        ]);
    }

    #[test]
    fn test_get_domain() {
        let src = [
            (&[0, 0, 0][..], "000"),
            (&[0, 1, 2][..], "012"),
            (&[0, 1, 3][..], "013"),
            (&[0, 1, 3][..], "013"),
            (&[0, 1, 3, 4][..], "0134"),
            (&[1, 0][..], "10"),
            (&[1, 1][..], "11"),
            (&[2, 1, 2][..], "212"),
        ];
        assert_eq!(
            Trie::get_domain(&src[..], 0, 0),
            &[
                (&[0, 0, 0][..], "000"),
                (&[0, 1, 2][..], "012"),
                (&[0, 1, 3][..], "013"),
                (&[0, 1, 3][..], "013"),
                (&[0, 1, 3, 4][..], "0134"),
            ][..]);
        assert_eq!(
            Trie::get_domain(Trie::get_domain(&src[..], 0, 0), 1, 1),
            &[
                (&[0, 1, 2][..], "012"),
                (&[0, 1, 3][..], "013"),
                (&[0, 1, 3][..], "013"),
                (&[0, 1, 3, 4][..], "0134"),
            ][..]);
        assert_eq!(
            Trie::get_domain(&src[..], 0, 1),
            &[
                (&[1, 0][..], "10"),
                (&[1, 1][..], "11"),
            ][..]);
        assert_eq!(
            Trie::get_domain(&src[..], 0, 2),
            &[
                (&[2, 1, 2][..], "212"),
            ][..]);
    }

    #[test]
    fn test_add_find() {
        let trie = Trie::static_construction(&mut vec![
            ("咲き乱れ".as_bytes(), String::from("咲き乱れ")),
            ("張り込め".as_bytes(), String::from("張り込め")),
            ("1".as_bytes(), String::from("1")),
            ("1月".as_bytes(), String::from("1月")),
            ("幻視".as_bytes(), String::from("幻視")),
            ("アオガエル".as_bytes(), String::from("アオガエル")),
            ("扁かろ".as_bytes(), String::from("扁かろ")),
            ("証城寺".as_bytes(), String::from("証城寺")),
            ("たい".as_bytes(), String::from("たい")),
            ("ニッカーボッカー".as_bytes(), String::from("ニッカーボッカー")),
            ("抜け出せれ".as_bytes(), String::from("抜け出せれ")),
            ("長府浜浦".as_bytes(), String::from("長府浜浦")),
            ("中佃".as_bytes(), String::from("中佃")),
            ("小船木".as_bytes(), String::from("小船木")),
            ("差し昇っ".as_bytes(), String::from("差し登っ")),
            ("浅黒かれ".as_bytes(), String::from("浅黒かれ")),
        ]);

        assert_eq!(trie.find("張り込め".as_bytes()), Ok(&[String::from("張り込め")][..]));
        assert_eq!(trie.find("1".as_bytes()), Ok(&[String::from("1")][..]));
        assert_eq!(trie.find("1月".as_bytes()), Ok(&[String::from("1月")][..]));
        assert_eq!(trie.find("ニッカーボッカー".as_bytes()), Ok(&[String::from("ニッカーボッカー")][..]));
        assert_eq!(trie.find("証城寺".as_bytes()), Ok(&[String::from("証城寺")][..]));
        assert_eq!(trie.find("差し昇っ".as_bytes()), Ok(&[String::from("差し登っ")][..]));
        assert_eq!(trie.find("抜け出せれ".as_bytes()), Ok(&[String::from("抜け出せれ")][..]));
        assert_eq!(trie.find("たい".as_bytes()), Ok(&[String::from("たい")][..]));
        assert_eq!(trie.find("アオガエル".as_bytes()), Ok(&[String::from("アオガエル")][..]));
        assert_eq!(trie.find("長府浜浦".as_bytes()), Ok(&[String::from("長府浜浦")][..]));
        assert_eq!(trie.find("中佃".as_bytes()), Ok(&[String::from("中佃")][..]));
        assert_eq!(trie.find("幻視".as_bytes()), Ok(&[String::from("幻視")][..]));
        assert_eq!(trie.find("小船木".as_bytes()), Ok(&[String::from("小船木")][..]));
        assert_eq!(trie.find("浅黒かれ".as_bytes()), Ok(&[String::from("浅黒かれ")][..]));
        assert_eq!(trie.find("扁かろ".as_bytes()), Ok(&[String::from("扁かろ")][..]));
        assert_eq!(trie.find("咲き乱れ".as_bytes()), Ok(&[String::from("咲き乱れ")][..]));
    }
}
