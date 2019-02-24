// copyright (c) 2019 Nakano Masaki <namachan10777@gmail.com>
use std::usize;
use std::vec::Vec;
use std::u8;

// ↓読んで
// https://takeda25.hatenablog.jp/entry/20120219/1329634865
// 文字ではなくbyte列として扱う。冗長なバイト列なんて存在しないです
#[derive(Clone, Copy, Debug, PartialEq)]
struct Node {
    // 行のオフセット
    base: usize,
    // 親ノードのindex(子のノードが本当に自分の子なのかを確かめる)
    check: usize,
    // WordInfoへのptr
    ptr: usize,
}

impl Default for Node {
    fn default() -> Node {
        Node {
            base: NOWHERE,
            check: NOWHERE,
            ptr: NOWHERE
        }
    }
}

pub struct Trie {
    // 圧縮済み遷移表
    arr: Vec<Node>,
    // 品詞辞書本体
    infos: Vec<WordInfo>,
    // 0~1, 1~4, 4~32, 32~
    footprint: [usize; 4],
}

#[derive(Clone, Debug, PartialEq, Default)]
struct Class {
    class: String,
    subclass: String,
    desc: String,
    subdesc: String,
}

#[derive(Clone, Debug, PartialEq)]
struct WordInfo {
    id: i16,
    cost: i16,
    class: Class,
}

const NOWHERE: usize = usize::MAX;
const UNKNOWN: usize = usize::MAX-2;
const ROW_LEN: usize = 256;

type Row = [Node; ROW_LEN];

fn cnt_enable(nodes: &Row) -> usize {
    let mut c = 0;
    for node in nodes.iter() {
        if node.check != NOWHERE {
            c += 1;
        }
    }
    c
}

// test helper
fn make_row(head: &[Node]) -> [Node; ROW_LEN] {
    let mut row = [Node::default(); ROW_LEN];
    for i in 0..head.len() {
        row[i] = head[i];
    }
    row
}

// test helper
fn make_arr(len: usize, head: &[Node]) -> Vec<Node> {
    let mut arr = Vec::new();
    arr.resize(len, Node::default());
    for i in 0..head.len() {
        arr[i] = head[i]
    }
    arr
}

// test helper
const DUMMY1: Node = Node{base: 0, check: 0, ptr: 0};
const DUMMY2: Node = Node{base: 0, check: 1, ptr: 0};
const EMP: Node = Node{base: NOWHERE, check: NOWHERE, ptr: NOWHERE};

// low level
impl Trie {
    fn erase(&mut self, start: usize, parent: usize) {
        for i in start..start+ROW_LEN {
            if self.arr[i].check == parent {
                self.arr[i] = Node::default();
            }
        }
    }
    
    fn extract_row(&self, start: usize, parent: usize) -> Row {
        let mut buf = [Node::default(); ROW_LEN];
        for i in 0..ROW_LEN {
            if self.arr[start+i].check == parent {
                buf[i] = self.arr[start+i];
            }
        }
        buf
    }

    fn update_children_base(&mut self, from: usize, to: usize) {
        let base = self.arr[from].base;
        if base != NOWHERE {
            for i in base..base+ROW_LEN {
                if i < self.arr.len() && self.arr[i].check == from {
                    self.arr[i].check = to;
                }
            }
        }
    }

    fn mov_row(&mut self, check: usize, to: usize) {
        let base = self.arr[check].base;
        let mut buf = [Node::default(); ROW_LEN];
        for i in 0..ROW_LEN {
            if self.arr[base+i].check == check {
                self.update_children_base(base+i, to+i);
                buf[i] = self.arr[base+i];
                self.arr[base+i] = Node::default();
            }
        }
        for i in 0..ROW_LEN {
            if buf[i].check != NOWHERE {
                self.arr[to+i] = buf[i];
            }
        }
    }
}
#[cfg(test)]
mod test_low_level_trie {
    use super::*;
    #[test]
    fn test_erace() {
        let mut trie = Trie::new();
        trie.arr = make_arr(ROW_LEN+1, &[DUMMY1, EMP, DUMMY2, DUMMY1]);
        trie.erase(1, 0);
        assert_eq!(trie.arr, make_arr(ROW_LEN+1, &[DUMMY1, EMP, DUMMY2, EMP]));
        trie.arr = make_arr(ROW_LEN, &[DUMMY1, EMP, DUMMY2, DUMMY1]);
        trie.erase(0, 0);
        assert_eq!(trie.arr, make_arr(ROW_LEN, &[EMP, EMP, DUMMY2, EMP]));
    }

    #[test]
    fn test_extract_row() {
        let mut trie = Trie::new();
        trie.arr = make_arr(ROW_LEN+10, &[DUMMY1, EMP, DUMMY2, DUMMY1]);
        assert_eq!(trie.extract_row(0, 0).to_vec(), make_row(&[DUMMY1, EMP, EMP, DUMMY1]).to_vec());
        assert_eq!(trie.extract_row(1, 0).to_vec(), make_row(&[EMP, EMP, DUMMY1, EMP]).to_vec());
        assert_eq!(trie.extract_row(0, 1).to_vec(), make_row(&[EMP, EMP, DUMMY2, EMP]).to_vec());
    }

    #[test]
    fn test_update_children_base() {
        let mut trie = Trie::new();
        trie.arr = make_arr(ROW_LEN, &[
            Node { base: NOWHERE, check: NOWHERE, ptr: 1 },
            Node { base: 2, check: 0, ptr: 2 },
            Node { base: NOWHERE, check: 1, ptr: 3 },
            Node { base: NOWHERE, check: NOWHERE, ptr: 4 },
            Node { base: NOWHERE, check: 1, ptr: 5 },
        ]);
        trie.update_children_base(0, 5);
        assert_eq!(trie.arr, make_arr(ROW_LEN, &[
            Node { base: NOWHERE, check: NOWHERE, ptr: 1 },
            Node { base: 2, check: 0, ptr: 2 },
            Node { base: NOWHERE, check: 1, ptr: 3 },
            Node { base: NOWHERE, check: NOWHERE, ptr: 4 },
            Node { base: NOWHERE, check: 1, ptr: 5 },
        ]));
        trie.update_children_base(1, 0);
        assert_eq!(trie.arr, make_arr(ROW_LEN, &[
            Node { base: NOWHERE, check: NOWHERE, ptr: 1 },
            Node { base: 2, check: 0, ptr: 2 },
            Node { base: NOWHERE, check: 0, ptr: 3 },
            Node { base: NOWHERE, check: NOWHERE, ptr: 4 },
            Node { base: NOWHERE, check: 0, ptr: 5 },
        ]));
    }

    #[test]
    fn test_mov_row() {
        let mut trie = Trie::new();
        trie.arr = make_arr(ROW_LEN+10, &[
            Node { base: 1, check: 0, ptr: NOWHERE },
            Node { base: NOWHERE, check: 0, ptr: 1 },
            Node { base: 3, check: 0, ptr: 2 },
            Node { base: NOWHERE, check: 2, ptr: 3 },
            Node { base: NOWHERE, check: NOWHERE, ptr: NOWHERE },
            Node { base: NOWHERE, check: 2, ptr: 5 },
        ]);
        trie.mov_row(0, 6);
        assert_eq!(trie.arr,
            make_arr(ROW_LEN+10, &[
                Node { base: 1, check: 0, ptr: NOWHERE },
                Node::default(),
                Node::default(),
                Node { base: NOWHERE, check: 7, ptr: 3 },
                Node { base: NOWHERE, check: NOWHERE, ptr: NOWHERE },
                Node { base: NOWHERE, check: 7, ptr: 5 },
                Node { base: NOWHERE, check: 0, ptr: 1 },
                Node { base: 3, check: 0, ptr: 2 },
        ]));
    }
}

// middle level
impl Trie {
    // 経路を辿り、辿りきれば終点のindexを、辿りきれなければ(終点のindex, 辿れた数)を返す
    fn pursue(&self, octets: &[u8]) -> Result<usize, (usize, usize)> {
        let mut child_id: usize = 0;
        for i in 0..octets.len() {
            if self.arr[child_id].base == NOWHERE {
                return Err((child_id, i))
            }
            let new_child_id = self.arr[child_id].base + octets[i] as usize;
            if new_child_id >= self.arr.len() || self.arr[new_child_id].check != child_id {
                return Err((child_id, i))
            }
            child_id = new_child_id;
        }
        Ok(child_id)
    }

    fn placeable(&mut self, ignore: usize, offset: usize, row: &Row) -> bool {
        for j in 0..ROW_LEN {
            // 衝突があると再配置不可
            if row[j].check != NOWHERE && self.arr[offset + j].check != NOWHERE && self.arr[offset + j].check != ignore {
                return false
            }
        }
        return true
    }

    // TODO 高速化
    // octetで指定されたoctedへの遷移だけを持つrowを配置する。
    fn find_placeable_pos(&mut self, ignore: usize, nodes: &Row) -> usize {

        let enable_cnt = cnt_enable(nodes);
        let row_class = if enable_cnt < 2 {
            0
        }
        else if enable_cnt < 4 {
            1
        }
        else if enable_cnt < 32 {
            2
        }
        else {
            3
        };

        for i in self.footprint[row_class]..(self.arr.len() - ROW_LEN) {
            if self.placeable(ignore, i, &nodes) {
                self.footprint[row_class] = i;
                return i
            }
        }
        self.arr.resize(self.arr.len() + ROW_LEN, Node::default());
        for i in (self.arr.len() - ROW_LEN * 2)..(self.arr.len()+nodes.len()) {
            if self.placeable(ignore, i, &nodes) {
                self.footprint[row_class] = i;
                return i
            }
        }
        unreachable!()
    }

    fn place(&mut self, nodes: &Row) -> usize {
        let p = self.find_placeable_pos(NOWHERE, &nodes);
        for i in 0..nodes.len() {
            if nodes[i].check != NOWHERE {
                self.arr[i+p] = nodes[i];
            }
        }
        p
    }
    
    fn push_out(&mut self, occupy_idx: usize) {
        let parent_idx = self.arr[occupy_idx].check;
        let parent = self.arr[parent_idx];
        let brothers = self.extract_row(parent.base, parent_idx);
        // 再配置防止に
        self.arr[occupy_idx].check = UNKNOWN;
        let new_base = self.find_placeable_pos(parent_idx, &brothers);
        self.arr[occupy_idx].check = parent_idx;
        self.mov_row(parent_idx, new_base);

        self.arr[parent_idx].base = new_base;
    }
}
#[cfg(test)]
mod test_middle_level_trie {
    use super::*;

    #[test]
    fn test_find_placeable_pos() {
        let mut trie = Trie::new();
        assert_eq!(trie.find_placeable_pos(NOWHERE, &make_row(&[DUMMY1])), 1);
        trie.arr = make_arr(ROW_LEN, &[DUMMY1, EMP, DUMMY1, DUMMY1]);
        assert_eq!(trie.find_placeable_pos(NOWHERE, &make_row(&[DUMMY1, EMP, DUMMY1])), 4);
        trie.arr = [Node { base: 0, check: 0, ptr: 0 }; ROW_LEN].to_vec();
        assert_eq!(trie.find_placeable_pos(NOWHERE, &make_row(&[DUMMY1, EMP, DUMMY1])), ROW_LEN);
        trie.arr = make_arr(ROW_LEN, &[DUMMY1, EMP, DUMMY1, DUMMY1]);
        assert_eq!(trie.find_placeable_pos(0, &make_row(&[DUMMY1, EMP, DUMMY1])), 0);
        trie.arr = make_arr(ROW_LEN, &[DUMMY1, DUMMY2, DUMMY1, DUMMY1]);
        assert_eq!(trie.find_placeable_pos(1, &make_row(&[DUMMY1, EMP, EMP, DUMMY1])), 1);
    }

    #[test]
    fn test_pursue() {
        let mut trie = Trie::new();
        trie.arr = [
            // root
            Node { base: 1, check: NOWHERE - 1, ptr: 0 },
            // 1 ~ 3
            Node { base: 0, check: NOWHERE, ptr: 0 }, Node { base: 4, check: 0, ptr: 0 }, Node { base: 6, check: 0, ptr: 0 },
            // 4 ~ 5
            Node { base: 0, check: NOWHERE, ptr: 0 }, Node { base: 6, check: 2, ptr: 0 },
            // 6
            Node { base: 7, check: 3, ptr: 0 },
            // 7
            Node { base: 8, check: 5, ptr: 0 }
        ].to_vec();
        assert_eq!(trie.pursue(&vec![0, 1]), Err((0, 0)));
        assert_eq!(trie.pursue(&vec![1, 1]), Ok(5));
        assert_eq!(trie.pursue(&vec![1, 1, 0]), Err((5, 2)));
        assert_eq!(trie.pursue(&vec![1, 2]), Err((2, 1)));
        assert_eq!(trie.pursue(&vec![2, 0, 1]), Err((6, 2)));
        assert_eq!(trie.pursue(&vec![2, 0]), Ok(6));
        assert_eq!(trie.pursue(&vec![1, 1, 1]), Ok(7));
    }

    #[test]
    fn test_place() {
        let mut trie = Trie::new();
        trie.arr = make_arr(ROW_LEN, &[DUMMY1, EMP, DUMMY1, DUMMY1]);
        trie.place(&make_row(&[DUMMY1, EMP, DUMMY1]));
        let ans = make_arr(ROW_LEN*2, &[DUMMY1, EMP, DUMMY1, DUMMY1, DUMMY1, EMP, DUMMY1]);
        assert_eq!(trie.arr, ans);
    }

    #[test]
    fn test_push_out() {
        let mut trie = Trie::new();
        trie.arr = make_arr(ROW_LEN*2, &[
            Node { base: 1, check: NOWHERE-1, ptr: NOWHERE },
            Node { base: 2, check: 0, ptr: NOWHERE },
            Node { base: 4, check: 1, ptr: NOWHERE },
            Node { base: NOWHERE, check: 1, ptr: NOWHERE },
            Node::default(),
            Node::default(),
            Node { base: NOWHERE, check: 2, ptr: NOWHERE },
        ]);
        trie.push_out(2);
        assert_eq!(trie.arr, make_arr(ROW_LEN*2, &[
            Node { base: 1, check: NOWHERE-1, ptr: NOWHERE },
            Node { base: 3, check: 0, ptr: NOWHERE },
            Node::default(),
            Node { base: 4, check: 1, ptr: NOWHERE },
            Node { base: NOWHERE, check: 1, ptr: NOWHERE },
            Node::default(),
            Node { base: NOWHERE, check: 3, ptr: NOWHERE },
        ]));
        trie.push_out(3);
        assert_eq!(trie.arr, make_arr(ROW_LEN*2, &[
            Node { base: 1, check: NOWHERE-1, ptr: NOWHERE },
            Node { base: 4, check: 0, ptr: NOWHERE },
            Node::default(),
            Node::default(),
            Node { base: 4, check: 1, ptr: NOWHERE },
            Node { base: NOWHERE, check: 1, ptr: NOWHERE },
            Node { base: NOWHERE, check: 4, ptr: NOWHERE },
        ]));
    }
}

impl Trie {
    fn new() -> Trie {
        let mut arr = vec![Node::default(); ROW_LEN+1].to_vec();
        arr[0] = Node { base: 1, check: 0, ptr: 0 };
        Trie {
            arr: arr,
            infos: Vec::new(),
            footprint: [0, 0, 0, 0],
        }
    }


    fn add(&mut self, octets: &[u8], info: WordInfo) {
        if let Err((common, pursued)) = self.pursue(octets) {
            if self.arr[common].base != NOWHERE {
                let current = self.arr[common].base + octets[pursued] as usize;
                // 非終端ノードかつ衝突あり
                if self.arr[current].check != NOWHERE {
                    self.push_out(current);
                }
            }
            // 終端ノード
            else {
                // 子のスペースを確保し、非終端ノードに
                let mut row = [Node::default(); ROW_LEN];
                row[octets[pursued] as usize].check = common;
                let base = self.place(&row);
                self.arr[common].base = base;
            }

            let mut parent = common;

            for i in pursued..octets.len() {
                // rowを追加しながらparentを更新していく
                let mut row = [Node::default(); ROW_LEN];
                row[octets[i] as usize].check = parent;
                let base = self.place(&row);
                self.arr[parent].base = base;
                parent = base + octets[i] as usize;
            }

            self.infos.push(info);
            self.arr[parent].ptr = self.infos.len() - 1;
        }
    }

    fn find(&mut self, octets: &Vec<u8>) -> Option<WordInfo> {
        if let Ok(idx) = self.pursue(octets) {
            let info_idx = self.arr[idx].ptr;
            if info_idx == NOWHERE {
                None
            }
            else {
                Some(self.infos[info_idx].clone())
            }
        }
        else {
            None
        }
    }
}
#[cfg(test)]
mod trie_test {
    use super::*;

    #[test]
    fn test_add_find() {
        let empty_class = Class { class: "".to_string(), subclass: "".to_string(), desc: "".to_string(), subdesc: "".to_string() };
        let w1 = WordInfo { id: 0, cost: 0, class: empty_class.clone() };
        let w2 = WordInfo { id: 1, cost: 0, class: empty_class.clone() };
        let w3 = WordInfo { id: 2, cost: 0, class: empty_class.clone() };
        let mut trie = Trie::new();
        trie.add(&vec![0], w1.clone());
        assert_eq!(trie.find(&vec![0]), Some(w1.clone()));
        assert_eq!(trie.find(&vec![1]), None);
        trie.add(&vec![0, 1], w2.clone());
        assert_eq!(trie.find(&vec![0]), Some(w1.clone()));
        assert_eq!(trie.find(&vec![1]), None);
        assert_eq!(trie.find(&vec![0, 1]), Some(w2.clone()));
        assert_eq!(trie.find(&vec![0, 0]), None);
        trie.add(&vec![0, 0], w3.clone());
        assert_eq!(trie.find(&vec![0]), Some(w1.clone()));
        assert_eq!(trie.find(&vec![1]), None);
        assert_eq!(trie.find(&vec![0, 1]), Some(w2.clone()));
        assert_eq!(trie.find(&vec![0, 0]), Some(w3.clone()));
    }
}

use std::fs;
use std::io;
use std::io::BufRead;

pub fn build_trie(f: &fs::File) -> Trie {
    let mut trie = Trie::new();
    let mut reader = io::BufReader::new(f);
    let mut buf = String::new();
    loop {
        if let Ok(len) = reader.read_line(&mut buf){
            if len == 0 {
                break
            }
            let elms: Vec<&str> = buf.split(',').collect();
            println!("adding: {:?}", elms[0]);
            let key = elms[0].as_bytes();
            let id: i16 = elms[1].parse().unwrap();
            let cost: i16 = elms[3].parse().unwrap();
            let class = elms[4].to_string();
            let subclass = elms[5].to_string();
            let desc = elms[6].to_string();
            let subdesc = elms[7].to_string();
            let info = WordInfo {
                id: id,
                cost: cost,
                class: Class {
                    class: class,
                    subclass: subclass,
                    desc: desc,
                    subdesc: subdesc,
                },
            };
            trie.add(&key, info);
            buf.clear();
        }
        else {
            break
        }
    }
    trie
}

#[cfg(test)]
mod test_trie_build {
    use super::*;

    #[test]
    fn test_trie_build() {
    }
}

pub struct Splitter  {}
