// copyright (c) 2019 Nakano Masaki <namachan10777@gmail.com>

extern crate bot;
use std::fs;

fn main() {
    let f = fs::File::open("./test.csv").unwrap();
    let trie = bot::morph::build_trie(&f);
}
