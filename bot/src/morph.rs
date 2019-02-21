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

    // TODO 高速化
    // octetで指定されたoctedへの遷移だけを持つrowを配置する。
    fn find_placeable_pos(&mut self, nodes: &Row) -> usize {

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
            let mut placeable = true;
            for j in 0..nodes.len() {
                // 衝突があると再配置不可
                if nodes[j].check != NOWHERE && self.arr[i + j].check != NOWHERE {
                    placeable = false;
                    break
                }
            }
            if placeable {
                self.footprint[row_class] = i;
                return i
            }
        }
        for i in (self.arr.len() - nodes.len())..(self.arr.len()+nodes.len()) {
            let mut placeable = true;
            for j in 0..(self.arr.len() - i) {
                // 衝突があると再配置不可
                if nodes[j].check != NOWHERE && self.arr[i + j].check != NOWHERE {
                    placeable = false;
                    break
                }
            }
            if placeable {
                self.arr.resize(i + nodes.len(), Node::default());
                return i
            }
        }
        unreachable!()
    }

    fn place(&mut self, nodes: &Row) -> usize {
        let p = self.find_placeable_pos(&nodes);
        for i in 0..nodes.len() {
            if nodes[i].check != NOWHERE {
                self.arr[i+p] = nodes[i];
            }
        }
        p
    }

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
    
    fn push_out(&mut self, occupy_idx: usize) {
        let occupy_parent = self.arr[self.arr[occupy_idx].check];
        let occupy_row = self.extract_row(occupy_parent.base, occupy_parent.check);
        self.erase(occupy_parent.base, occupy_parent.check);
        // 再配置防止に
        self.arr[occupy_idx].check = 0;
        let occupy_base = self.place(&occupy_row);
        self.arr[occupy_parent.check].base = occupy_base;
        self.arr[occupy_idx].check = NOWHERE;
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
            Some(self.infos[self.arr[idx].ptr].clone())
        }
        else {
            None
        }
    }
}
#[cfg(test)]
mod trie_test {
    use super::*;

    const DUMMY1: Node = Node{base: 0, check: 0, ptr: 0};
    const EMP: Node = Node{base: NOWHERE, check: NOWHERE, ptr: NOWHERE};

    fn make_row(head: &[Node]) -> [Node; ROW_LEN] {
        let mut row = [Node::default(); ROW_LEN];
        for i in 0..head.len() {
            row[i] = head[i];
        }
        row
    }

    fn make_arr(len: usize, head: &[Node]) -> Vec<Node> {
        let mut arr = Vec::new();
        arr.resize(len, Node::default());
        for i in 0..head.len() {
            arr[i] = head[i]
        }
        arr
    }

    #[test]
    fn test_find_placeable_pos() {
        let mut trie = Trie::new();
        assert_eq!(trie.find_placeable_pos(&make_row(&[DUMMY1])), 1);
        trie.arr = make_arr(ROW_LEN, &[DUMMY1, EMP, DUMMY1, DUMMY1]);
        assert_eq!(trie.find_placeable_pos(&make_row(&[DUMMY1, EMP, DUMMY1])), 4);
        trie.arr = [Node { base: 0, check: 0, ptr: 0 }; ROW_LEN].to_vec();
        assert_eq!(trie.find_placeable_pos(&make_row(&[DUMMY1, EMP, DUMMY1])), ROW_LEN);
    }

    #[test]
    fn test_place() {
        let mut trie = Trie::new();
        trie.arr = make_arr(ROW_LEN, &[DUMMY1, EMP, DUMMY1, DUMMY1]);
        trie.place(&make_row(&[DUMMY1, EMP, DUMMY1]));
        let ans = make_arr(ROW_LEN+4, &[DUMMY1, EMP, DUMMY1, DUMMY1, DUMMY1, EMP, DUMMY1]);
        assert_eq!(trie.arr, ans);
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
    fn test_add() {
        let mut trie = Trie::new();
        let w1 = WordInfo { id: 0, cost: 1, class: Class::default() };
        let w2 = WordInfo { id: 0, cost: 2, class: Class::default() };
        let w3 = WordInfo { id: 0, cost: 3, class: Class::default() };
        trie.add(&vec![0], w1.clone());
        trie.add(&vec![0, 1], w2.clone());
        trie.add(&vec![1, 2, 3], w3.clone());
        assert_eq!(trie.find(&vec![0]), Some(w1));
        assert_eq!(trie.find(&vec![0, 1]), Some(w2));
        assert_eq!(trie.find(&vec![1, 2, 3]), Some(w3));
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
