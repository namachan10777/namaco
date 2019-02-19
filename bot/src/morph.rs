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
    // TODO 高速化
    // octetで指定されたoctedへの遷移だけを持つrowを配置する。
    fn find_placeable_pos(&mut self, nodes: &Vec<Node>) -> usize {
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

    fn place(&mut self, nodes: &Vec<Node>) {
        let p = self.find_placeable_pos(&nodes);
        for i in 0..nodes.len() {
            self.arr[i+p] = nodes[i];
        }
    }

    fn erase(&mut self, start: usize, len: usize, parent: usize) {
        for i in start..start+len {
            if self.arr[i].check == parent {
                self.arr[i] = Node::default();
            }
        }
    }
    
    fn extract_row(&self, start: usize, len: usize, parent: usize) -> Vec<Node> {
        let mut buf = Vec::new();
        buf.resize(len, Node::default());
        for i in 0..len {
            if self.arr[start+i].check == parent {
                buf[i] = self.arr[start+i];
            }
        }
        buf
    }
}
#[cfg(test)]
mod trie_test {
    use super::*;

    const dummy: Node = Node{base: 0, check: 0, ptr: 0};
    const dummy2: Node = Node{base: 0, check: 1, ptr: 0};
    const emp: Node = Node{base: 0, check: usize::MAX, ptr: 0};

    #[test]
    fn test_find_placeable_pos() {
        let mut trie = Trie::new();
        assert_eq!(trie.find_placeable_pos(&[dummy].to_vec()), 0);
        trie.arr = [dummy, emp, dummy, dummy].to_vec();
        assert_eq!(trie.find_placeable_pos(&[dummy, emp, dummy].to_vec()), 4);
        trie.arr = [dummy; 256].to_vec();
        assert_eq!(trie.find_placeable_pos(&[dummy, emp, dummy].to_vec()), 256);
    }

    #[test]
    fn test_place() {
        let mut trie = Trie::new();
        trie.arr = [dummy, emp, dummy, dummy].to_vec();
        trie.place(&[dummy, emp, dummy].to_vec());
        assert_eq!(trie.arr, [dummy, emp, dummy, dummy, dummy, emp, dummy]);
    }

    #[test]
    fn test_erase() {
        let mut trie = Trie::new();
        trie.arr = [dummy, dummy, dummy2, dummy, dummy].to_vec();
        trie.erase(1, 3, 0);
        assert_eq!(trie.arr, [dummy, emp, dummy2, emp, dummy]);
    }

    #[test]
    fn test_extract_row() {
        let mut trie = Trie::new();
        trie.arr = [dummy, dummy, dummy2, dummy, dummy].to_vec();
        assert_eq!(trie.extract_row(1, 3, 0), [dummy, emp, dummy]);
    }
}

pub struct Splitter  {}
