use std::usize;
use std::vec::Vec;
use std::u8;

// ↓読んで
// https://takeda25.hatenablog.jp/entry/20120219/1329634865
// 文字ではなくbyte列として扱う。冗長なバイト列なんて存在しないです
#[derive(Clone, Copy)]
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
            check: usize::MAX,
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

impl Trie {
    fn new() -> Trie {
        Trie {
            arr: [Node::default(); u8::MAX as usize + 1].to_vec(),
            infos: Vec::new(),
        }
    }
    // octetで指定されたoctedへの遷移だけを持つrowを配置する。
    fn find_placeable_pos(&mut self, nodes: Vec<Node>) -> usize {
        for i in 0..(self.arr.len() - nodes.len()) {
            let mut placeable = true;
            for j in 0..nodes.len() {
                // 衝突があると再配置不可
                if nodes[j].check != usize::MAX && self.arr[i + j].check != usize::MAX {
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
                if nodes[j].check != usize::MAX && self.arr[i + j].check != usize::MAX {
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
}
#[cfg(test)]
mod trie_test {
    use super::*;
    #[test]
    fn test_place_row() {
        let mut trie = Trie::new();
        assert_eq!(trie.find_placeable_pos([Node{base: 0, check: 0, ptr: 0}].to_vec()), 0);
        let dummy = Node{base: 0, check: 0, ptr: 0};
        let emp = Node::default();
        trie.arr = [dummy, emp, dummy, dummy].to_vec();
        assert_eq!(trie.find_placeable_pos([dummy, emp, dummy].to_vec()), 4);
        trie.arr = [dummy; 256].to_vec();
        assert_eq!(trie.find_placeable_pos([dummy, emp, dummy].to_vec()), 256);
    }
}

pub struct Splitter  {}
