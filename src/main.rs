// copyright (c) 2019 Nakano Masaki <namachan10777@gmail.com>

extern crate namaco;
use namaco::parser;
use std::fs;

fn main() {
    let file = fs::File::open("./short.csv").unwrap();
    let cfg = parser::DictCfg {
        word: 0,
        matrix_id: 1,
        gencost: 2,
    };
    let _trie = parser::build_trie(&file, &cfg, |arr| arr[3].trim().to_string()).unwrap();
}
