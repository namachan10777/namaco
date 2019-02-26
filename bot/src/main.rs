// copyright (c) 2019 Nakano Masaki <namachan10777@gmail.com>

extern crate bot;
use std::fs;
use std::io;

fn main() {
    let f = fs::File::open("./test.csv").unwrap();
    let trie = bot::morph::build_trie(&f);
    loop {
        let mut buf = String::new();
        io::stdin().read_line(&mut buf).ok();
        if buf.len() == 0 {
            break
        }
        println!("{:?}", trie.find(buf.trim().as_bytes()));
    }
}
