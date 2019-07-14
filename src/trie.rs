// copyright (c) 2019 Nakano Masaki <namachan10777@gmail>
use std::usize;

struct Node {
    // rowのオフセット
    base: usize,
    // 親ノードのindex
    check: usize,
    // 格納している要素のid
    ptr: Option<usize>,
}

struct Trie<T> {
    // 圧縮済みの遷移表
    tree: Vec<Node>,
    // 辞書本体
    storage: Vec<T>,
}

impl<T> Trie<T> {
}
