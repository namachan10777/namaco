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
            base: 0,
            check: NOWHERE,
            ptr: 0
        }
    }
}

struct Trie {
    // 圧縮済み遷移表
    arr: Vec<Node>,
    // 品詞辞書本体
    infos: Vec<WordInfo>
}

#[derive(Clone)]
enum wclass {
}

#[derive(Clone)]
struct WordInfo {
    lid: u16,
    rid: u16,
    cost: u16,
    wclass: wclass,
}

const NOWHERE: usize = usize::MAX;
const ROW_LEN: usize = 256;

impl Trie {
    fn new() -> Trie {
        let mut fst = [Node::default(); 257].to_vec();
        fst[0] = Node { base: 1, check: 0, ptr: 0 };
        Trie {
            arr: fst,
            infos: Vec::new(),
        }
    }

    // 経路を辿り、辿りきれば終点のindexを、辿りきれなければ(終点のindex, 辿れた数)を返す
    fn pursue(&self, parent: usize, octets: &Vec<u8>) -> Result<usize, (usize, usize)> {
        let mut check: usize = self.arr[parent].check;
        let mut child_id: usize = parent;
        for i in 0..octets.len() {
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
    fn find_placeable_pos(&mut self, nodes: &Vec<Node>) -> usize {
        for i in 0..(self.arr.len() - nodes.len()) {
            let mut placeable = true;
            for j in 0..nodes.len() {
                // 衝突があると再配置不可
                if nodes[j].check != NOWHERE && self.arr[i + j].check != NOWHERE {
                    placeable = false;
                    break
                }
            }
            if placeable {
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

    fn place(&mut self, nodes: &Vec<Node>) -> usize {
        let p = self.find_placeable_pos(&nodes);
        for i in 0..nodes.len() {
            self.arr[i+p] = nodes[i];
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
    
    fn extract_row(&self, start: usize, parent: usize) -> Vec<Node> {
        let mut buf = Vec::new();
        buf.resize(ROW_LEN, Node::default());
        for i in 0..ROW_LEN {
            if self.arr[start+i].check == parent {
                buf[i] = self.arr[start+i];
            }
        }
        buf
    }
    
    fn push_out(&mut self, base: usize, octet: usize) {
        let occupy_parent = self.arr[base + octet].check;
        let occupy_row = self.extract_row(base, occupy_parent);
        self.erase(base, occupy_parent);
        self.arr[base + octet].check = 0;
        let occupy_base = self.place(&occupy_row);
        self.arr[occupy_parent].base = occupy_base;
        self.arr[base + octet].check = NOWHERE;
    }
}
#[cfg(test)]
mod trie_test {
    use super::*;

    const dummy: Node = Node{base: 0, check: 0, ptr: 0};
    const dummy2: Node = Node{base: 0, check: 1, ptr: 0};
    const emp: Node = Node{base: 0, check: NOWHERE, ptr: 0};

    #[test]
    fn test_find_placeable_pos() {
        let mut trie = Trie::new();
        assert_eq!(trie.find_placeable_pos(&[dummy].to_vec()), 1);
        trie.arr = [dummy, emp, dummy, dummy].to_vec();
        assert_eq!(trie.find_placeable_pos(&[dummy, emp, dummy].to_vec()), 4);
        trie.arr = [dummy; ROW_LEN].to_vec();
        assert_eq!(trie.find_placeable_pos(&[dummy, emp, dummy].to_vec()), ROW_LEN);
    }

    #[test]
    fn test_place() {
        let mut trie = Trie::new();
        trie.arr = [dummy, emp, dummy, dummy].to_vec();
        trie.place(&[dummy, emp, dummy].to_vec());
        assert_eq!(trie.arr, [dummy, emp, dummy, dummy, dummy, emp, dummy]);
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
        assert_eq!(trie.pursue(0, &vec![0, 1]), Err((0, 0)));
        assert_eq!(trie.pursue(0, &vec![1, 1]), Ok(5));
        assert_eq!(trie.pursue(0, &vec![1, 1, 0]), Err((5, 2)));
        assert_eq!(trie.pursue(0, &vec![1, 2]), Err((2, 1)));
        assert_eq!(trie.pursue(0, &vec![2, 0, 1]), Err((6, 2)));
        assert_eq!(trie.pursue(0, &vec![2, 0]), Ok(6));
        assert_eq!(trie.pursue(0, &vec![1, 1, 1]), Ok(7));
    }
}

pub struct Splitter  {}
