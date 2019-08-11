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

#[derive(Clone, Debug, PartialEq, Default)]
struct Class {
    class: String,
    subclass: String,
    desc: String,
    subdesc: String,
}

#[derive(Clone, PartialEq, Debug)]
pub struct WordInfo {
    id: i16,
    cost: i64,
    class: Class,
    word: String,
}

use std::i16;

impl Default for WordInfo {
    fn default() -> WordInfo {
        WordInfo {
            id: i16::MAX,
            cost: i64::MAX,
            class: Class::default(),
            word: String::new(),
        }
    }
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
#[allow(dead_code)]
fn make_row(head: &[Node]) -> [Node; ROW_LEN] {
    let mut row = [Node::default(); ROW_LEN];
    for i in 0..head.len() {
        row[i] = head[i];
    }
    row
}

// test helper
#[allow(dead_code)]
fn make_arr(len: usize, head: &[Node]) -> Vec<Node> {
    let mut arr = Vec::new();
    arr.resize(len, Node::default());
    for i in 0..head.len() {
        arr[i] = head[i]
    }
    arr
}

fn count_children(row: &Row) -> usize {
    let mut cnt = 0;
    for i in 0..ROW_LEN {
        if row[i].check != NOWHERE {
            cnt += 1;
        }
    }
    cnt
}

// test helper
#[allow(dead_code)]
const DUMMY1: Node = Node{base: 0, check: 0, ptr: 0};
#[allow(dead_code)]
const DUMMY2: Node = Node{base: 0, check: 1, ptr: 0};
#[allow(dead_code)]
const EMP: Node = Node{base: NOWHERE, check: NOWHERE, ptr: NOWHERE};

pub struct Trie {
    // 圧縮済み遷移表
    arr: Vec<Node>,
    // 品詞辞書本体
    infos: Vec<Vec<WordInfo>>,
    // 0~1, 1~4, 4~32, 32~
    footprint: [usize; 4],
}

// low level
impl Trie {
    // TODO 高速化
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
        // 親を退避してから書かないと元の領域と移転先の領域が被っている場合に親が消える
        for i in 0..ROW_LEN {
            if self.arr[base+i].check == check {
                self.update_children_base(base+i, to+i);
                // 親の退避を同時に行う
                buf[i] = self.arr[base+i];
                self.arr[base+i] = Node::default();
            }
        }
        for i in 0..ROW_LEN {
            if buf[i].check != NOWHERE {
                // 親を書く
                self.arr[to+i] = buf[i];
            }
        }
    }
}
#[cfg(test)]
mod test_low_level_trie {
    use super::*;
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
        // 越えそうな場合は先に延長
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
    
    fn push_out(&mut self, occupy_idx: usize) -> usize {
        let parent_idx = self.arr[occupy_idx].check;
        let parent = self.arr[parent_idx];
        let brothers = self.extract_row(parent.base, parent_idx);
        // 再配置防止に
        self.arr[occupy_idx].check = UNKNOWN;
        let new_base = self.find_placeable_pos(parent_idx, &brothers);
        self.arr[occupy_idx].check = parent_idx;
        self.mov_row(parent_idx, new_base);

        self.arr[parent_idx].base = new_base;
        new_base
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

#[derive(Clone, Debug, PartialEq)]
pub enum TrieErr {
    KeyConflict,
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

    // FIXME 読み辛い
    fn add(&mut self, octets: &[u8], info: WordInfo) {
        match self.pursue(octets) {
            Err((common, mut pursued)) => {
                let mut parent = common;
                if self.arr[common].base != NOWHERE {
                    let current = self.arr[common].base + octets[pursued] as usize;
                    // 非終端ノードかつ衝突あり
                    if self.arr[current].check != NOWHERE {
                        let occupy = self.arr[self.arr[current].check];
                        // push_outとmov_brotherで動かすことになる子の数を計算
                        let push_out_cost = count_children(&self.extract_row(occupy.base, self.arr[current].check));
                        let mov_borther_cost = count_children(&self.extract_row(self.arr[common].base, common));
                        // 子が少ない方を動かす
                        if mov_borther_cost > push_out_cost {
                            // push_out対象に親を含む場合はcommonを更新
                            let common = if self.arr[common].check == self.arr[current].check {
                                // 子を既存のrowに追加
                                let old_base = self.arr[self.arr[common].check].base as i64;
                                let new_base = self.push_out(current) as i64;
                                (common as i64 - old_base + new_base) as usize
                            }
                            else {
                                self.push_out(current);
                                common
                            };
                            self.arr[current].check = common;
                            parent = current;
                        }
                        else {
                            // 兄弟を再配置
                            let mut new_row = self.extract_row(self.arr[common].base, common);
                            new_row[octets[pursued] as usize].check = common;
                            let new_base = self.find_placeable_pos(common, &new_row);
                            let new_current = new_base + octets[pursued] as usize;
                            self.mov_row(common, new_base);
                            self.arr[common].base = new_base;
                            self.arr[new_current].check = common;
                            parent = new_current;
                        }
                        pursued += 1;
                    }
                    // 非終端ノード(衝突がないので書くだけ)
                    else {
                        self.arr[current].check = common;
                        pursued += 1;
                        parent = current;
                    }
                }
                // 終端ノードの場合はただ付け加えるだけなのでpursuedを進めない
                for i in pursued..octets.len() {
                    // rowを追加しながらparentを更新していく
                    let mut row = [Node::default(); ROW_LEN];
                    row[octets[i] as usize].check = parent;
                    let base = self.place(&row);
                    self.arr[parent].base = base;
                    parent = base + octets[i] as usize;
                }

                self.infos.push(vec![info]);
                self.arr[parent].ptr = self.infos.len() - 1;
            },
            Ok(id) => {
                let ptr = self.arr[id].ptr;
                if ptr == NOWHERE {
                    self.infos.push(vec![info]);
                    self.arr[id].ptr = self.infos.len() - 1;
                }
                else {
                    self.infos[ptr].push(info);
                }
            }
        }
    }

    pub fn find(&self, octets: &[u8]) -> Vec<WordInfo> {
        if let Ok(idx) = self.pursue(octets) {
            let info_idx = self.arr[idx].ptr;
            if info_idx != NOWHERE {
                return self.infos[info_idx].clone()
            }
        }
        Vec::new()
    }
}
#[cfg(test)]
mod trie_test {
    use super::*;

    #[test]
    fn test_add_find() {
        let empty_class = Class { class: "".to_string(), subclass: "".to_string(), desc: "".to_string(), subdesc: "".to_string() };
        let w1 = WordInfo { word: String::new(), id: 0, cost: 0, class: empty_class.clone() };
        let w2 = WordInfo { word: String::new(), id: 1, cost: 0, class: empty_class.clone() };
        let w3 = WordInfo { word: String::new(), id: 2, cost: 0, class: empty_class.clone() };
        let w4 = WordInfo { word: String::new(), id: 3, cost: 0, class: empty_class.clone() };
        let w5 = WordInfo { word: String::new(), id: 4, cost: 0, class: empty_class.clone() };
        let w6 = WordInfo { word: String::new(), id: 5, cost: 0, class: empty_class.clone() };
        let mut trie = Trie::new();
        trie.add(&[0], w1.clone());
        assert_eq!(trie.find(&[0]), vec![w1.clone()]);
        assert_eq!(trie.find(&[1]), Vec::new());
        trie.add(&[0], w1.clone());
        assert_eq!(trie.find(&[0]), vec![w1.clone(), w1.clone()]);
        trie.add(&[0, 1], w2.clone());
        assert_eq!(trie.find(&[0]), vec![w1.clone(), w1.clone()]);
        assert_eq!(trie.find(&[1]), Vec::new());
        assert_eq!(trie.find(&[0, 1]), vec![w2.clone()]);
        assert_eq!(trie.find(&[0, 0]), Vec::new());
        trie.add(&[0, 0], w3.clone());
        assert_eq!(trie.find(&[0]), vec![w1.clone(), w1.clone()]);
        assert_eq!(trie.find(&[1]), Vec::new());
        assert_eq!(trie.find(&[0, 1]), vec![w2.clone()]);
        assert_eq!(trie.find(&[0, 0]), vec![w3.clone()]);
        trie.add(&[0, 1, 2], w4.clone());
        assert_eq!(trie.find(&[0]), vec![w1.clone(), w1.clone()]);
        assert_eq!(trie.find(&[1]), Vec::new());
        assert_eq!(trie.find(&[0, 1]), vec![w2.clone()]);
        assert_eq!(trie.find(&[0, 0]), vec![w3.clone()]);
        assert_eq!(trie.find(&[0, 1, 2]), vec![w4.clone()]);
        assert_eq!(trie.find(&[0, 1, 0]), Vec::new());
        trie.add(&[0, 1, 0], w5.clone());
        assert_eq!(trie.find(&[0]), vec![w1.clone(), w1.clone()]);
        assert_eq!(trie.find(&[1]), Vec::new());
        assert_eq!(trie.find(&[0, 1]), vec![w2.clone()]);
        assert_eq!(trie.find(&[0, 0]), vec![w3.clone()]);
        assert_eq!(trie.find(&[0, 1, 2]), vec![w4.clone()]);
        assert_eq!(trie.find(&[0, 1, 0]), vec![w5.clone()]);
        trie.add(&[2], w6.clone());
        assert_eq!(trie.find(&[0]), vec![w1.clone(), w1.clone()]);
        assert_eq!(trie.find(&[1]), Vec::new());
        assert_eq!(trie.find(&[0, 1]), vec![w2.clone()]);
        assert_eq!(trie.find(&[0, 0]), vec![w3.clone()]);
        assert_eq!(trie.find(&[0, 1, 2]), vec![w4.clone()]);
        assert_eq!(trie.find(&[0, 1, 0]), vec![w5.clone()]);
        assert_eq!(trie.find(&[2]), vec![w6.clone()]);
        trie.add(&[2], w1.clone());
        assert_eq!(trie.find(&[2]), vec![w6.clone(), w1.clone()]);

        let mut trie2 = Trie::new();
        trie2.add(&[0, 1, 5, 1], w1.clone());
        trie2.add(&[0, 1, 0, 2], w2.clone());
        trie2.add(&[0, 1, 4, 3], w3.clone());
        assert_eq!(trie2.find(&[0, 1, 5, 1]), vec![w1.clone()]);
        assert_eq!(trie2.find(&[0, 1, 0, 2]), vec![w2.clone()]);
        assert_eq!(trie2.find(&[0, 1, 4, 3]), vec![w3.clone()]);
    }
}

impl Trie {
    pub fn load_from_naist_jdic(f: &fs::File) -> Result<Trie, DictLoadErr> {
        let mut trie = Trie::new();
        let mut reader = io::BufReader::new(f);
        let mut buf = String::new();
        let mut line_cnt = 0;
        match io::stdout().flush() {
            Ok(_) => {},
            Err(_) => { return Err(DictLoadErr::FailedToFlushIO); },
        };
        loop {
            line_cnt += 1;
            if let Ok(len) = reader.read_line(&mut buf){
                if len == 0 {
                    break
                }
                let elms: Vec<&str> = buf.split(',').collect();
                if elms.len() < 8 {
                    return Err(DictLoadErr::ParseError(line_cnt))
                }
                let word = elms[0].to_string();
                let key = elms[0].as_bytes();
                let id: i16 = elms[2].trim().parse().unwrap_or(-1);
                let cost: i64 = match elms[3].trim().parse() {
                    Ok(cost) => cost,
                    Err(_) => { return Err(DictLoadErr::ParseError(line_cnt)) }
                };
                let class = elms[4].trim().to_string();
                let subclass = elms[5].trim().to_string();
                let desc = elms[6].trim().to_string();
                let subdesc = elms[7].trim().to_string();
                let info = WordInfo {
                    id: id,
                    cost: cost,
                    word: word,
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
        Ok(trie)
    }
}

use std::fs;
use std::io;
use std::io::{BufRead, Write};

#[derive(Debug)]
pub enum DictLoadErr {
    FailedToFlushIO,
    ParseError(usize),
}


#[cfg(test)]
mod test_trie_build {
    #[test]
    fn test_trie_build() {
    }
}

pub struct Matrix {
    arr: Vec<i64>,
    size: usize,
}

#[derive(Debug)]
pub enum MatrixLoadErr {
    ParseErr(usize),
    FileReadErr,
}

impl Matrix {
    fn new(size: usize) -> Matrix {
        let mut arr = Vec::new();
        arr.resize(size * size, 0);
        Matrix { size, arr }
    }

    fn cost(&self, lid: usize, rid: usize) -> i64 {
        self.arr[lid * self.size + rid]
    }

    fn write(&mut self, lid: usize, rid: usize, val: i64) {
        self.arr[lid * self.size + rid] = val
    }

    pub fn load_from_mecab_matrix(f: &fs::File) -> Result<Matrix, MatrixLoadErr> {
        let mut reader = io::BufReader::new(f);
        let mut buf = String::new();
        let mut line_cnt = 1;
        if let Ok(_) = reader.read_line(&mut buf) {
            let elms: Vec<&str> = buf.split(' ').collect();
            let size: usize = match elms[0].parse() {
                Ok(size) => size,
                Err(_) => { return Err(MatrixLoadErr::ParseErr(1)) },
            };
            buf.clear();
            let mut matrix = Matrix::new(size);
            loop {
                line_cnt += 1;
                if let Ok(len) = reader.read_line(&mut buf){
                    if len == 0 {
                        break
                    }
                    let elms: Vec<&str> = buf.split(' ').collect();
                    let lid: usize = match elms[0].trim().parse() {
                        Ok(num) => num,
                        Err(_) => { return Err(MatrixLoadErr::ParseErr(line_cnt)) },
                    };
                    let rid: usize = match elms[1].trim().parse() {
                        Ok(num) => num,
                        Err(_) => { return Err(MatrixLoadErr::ParseErr(line_cnt)) },
                    };
                    let cost: i64 = match elms[2].trim().parse() {
                        Ok(num) => num,
                        Err(_) => { return Err(MatrixLoadErr::ParseErr(line_cnt)) },
                    };
                    matrix.write(lid, rid, cost);
                    buf.clear();
                }
            }
            Ok(matrix)
        }
        else {
            Err(MatrixLoadErr::FileReadErr)
        }
    }
}

#[cfg(test)]
mod test_matrix {
    use super::*;
    #[test]
    fn test_new() {
        assert_eq!(Matrix::new(2).arr.len(), 2 * 2);
        assert_eq!(Matrix::new(100).arr.len(), 100 * 100);
    }

    #[test]
    fn test_cost() {
        let mut matrix = Matrix::new(3);
        matrix.arr = vec![
            1, 2, 3,
            4, 5, 6,
            7, 8, 9,
        ];
        assert_eq!(matrix.cost(1, 1), 5);
        assert_eq!(matrix.cost(0, 2), 3);
        assert_eq!(matrix.cost(2, 2), 9);

        let mut matrix = Matrix::new(4);
        matrix.arr = vec![
             1,  2,  3,  4,
             5,  6,  7,  8,
             9, 10, 11, 12,
            13, 14, 15, 16,
        ];
        assert_eq!(matrix.cost(1, 1), 6);
        assert_eq!(matrix.cost(0, 2), 3);
        assert_eq!(matrix.cost(3, 3), 16);
    }

    #[test]
    fn test_write() {
        let mut matrix = Matrix::new(3);
        matrix.write(1, 1, 5);
        matrix.write(0, 2, 3);
        matrix.write(2, 2, 9);
        assert_eq!(matrix.cost(1, 1), 5);
        assert_eq!(matrix.cost(0, 2), 3);
        assert_eq!(matrix.cost(2, 2), 9);
        let mut matrix = Matrix::new(4);
        matrix.write(1, 1, 6);
        matrix.write(0, 2, 3);
        matrix.write(3, 3, 16);
        assert_eq!(matrix.cost(1, 1), 6);
        assert_eq!(matrix.cost(0, 2), 3);
        assert_eq!(matrix.cost(3, 3), 16);
    }
}



use std::i64;

#[derive(Clone, Debug, PartialEq)]
struct Square {
    cost: i64,
    info: WordInfo,
    stack: Vec<(WordInfo, usize, usize)>,
}

impl Default for Square {
    fn default() -> Square {
        Square {
            cost: i64::MAX,
            info: WordInfo::default(),
            stack: Vec::new(),
        }
    }
}

#[allow(dead_code)]
pub fn fill_dp(input: &[u8], dict: &Trie, matrix: &Matrix) -> (i64, Vec<(WordInfo, usize, usize)>) {
    let len = input.len();
    let mut dp: Vec<Vec<Vec<Square>>> = Vec::new();
    dp.resize_with(len, || { let mut vec = Vec::new(); vec.resize_with(len, || Vec::new()); vec });
    for succ_end in 1..len+1 {
        let succ_word = input.get(0..succ_end).unwrap();
        for succ_info in dict.find(succ_word) {
            dp[succ_end-1][0].push(Square {
                cost: succ_info.clone().cost,
                info: succ_info.clone(),
                stack: vec![(succ_info, 0, succ_end)],
            });
        }
        for succ_begin in 1..succ_end {
            let succ_word = input.get(succ_begin..succ_end).unwrap();
            for succ_info in dict.find(succ_word) {
                let mut best = Square {
                    cost: i64::MAX,
                    info: WordInfo::default(),
                    stack: Vec::new(),
                };
                let prev_end = succ_begin;
                for prev_begin in 0..prev_end {
                    for prev in  &dp[prev_end-1][prev_begin] {
                        let cost = prev.cost
                            + matrix.cost(prev.info.id as usize, succ_info.clone().id as usize)
                            + succ_info.clone().cost;
                        if cost < best.cost {
                            let mut stack = prev.stack.clone();
                            stack.push((succ_info.clone(), succ_begin, succ_end));
                            best = Square {
                                cost,
                                info: succ_info.clone(),
                                stack
                            };
                        }
                    }
                }
                if best.cost < i64::MAX {
                    dp[succ_end-1][succ_begin].push(best);
                }
            }
        }
    }
    let mut min = Square::default();
    for i in 0..len {
        for square in &dp[len-1][i] {
            if square.cost < min.cost {
                min = square.clone();
            }
        }
    }
    (min.cost, min.stack)
}

#[cfg(test)]
mod test_viterbi {
    use super::*;
    #[test]
    fn test_dp_initialize () {
        let mut trie = Trie::new();
        let empty_class = Class { class: "".to_string(), subclass: "".to_string(), desc: "".to_string(), subdesc: "".to_string() };
        let a = WordInfo { word: String::new(), id: 0, cost: 10, class: empty_class.clone() };
        let b = WordInfo { word: String::new(), id: 1, cost: 20, class: empty_class.clone() };
        let ab = WordInfo { word: String::new(), id: 2, cost: 20, class: empty_class.clone() };
        let ba = WordInfo { word: String::new(), id: 3, cost: 30, class: empty_class.clone() };
        trie.add("a".as_bytes(), a.clone());
        trie.add("b".as_bytes(), b.clone());
        trie.add("ab".as_bytes(), ab.clone());
        trie.add("ba".as_bytes(), ba.clone());
        let mut matrix = Matrix::new(4);
        matrix.arr = vec![
            1, 2, 3, 4,
            5, 6, 7, 8,
            9, 10, 11, 12,
            13, 14, 15, 16
        ];
        let input = "aabb".as_bytes();
        assert_eq!(fill_dp(&input, &trie, &matrix),
            (10 + 3 + 20 + 10 + 20, vec![
                (a.clone(), 0, 1),
                (ab.clone(), 1, 3),
                (b.clone(), 3, 4),
            ])
        );

        matrix.arr = vec![
            1, 2, 30, 4,
            5, 6, 7, 8,
            9, 10, 11, 12,
            13, 14, 15, 16
        ];
        assert_eq!(fill_dp(&input, &trie, &matrix),
            (10 + 1 + 10 + 2 + 20 + 6 + 20, vec![
                (a.clone(), 0, 1),
                (a, 1, 2),
                (b.clone(), 2, 3),
                (b, 3, 4),
            ])
        );
    }
}

pub struct Splitter  {}