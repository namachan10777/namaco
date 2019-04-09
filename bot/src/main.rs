// copyright (c) 2019 Nakano Masaki <namachan10777@gmail.com>

extern crate bot;
use std::fs;
use std::io;
use std::str;

fn main() {
    let dict_file = fs::File::open("./naist-jdic.csv").unwrap();
    let matrix_file = fs::File::open("./matrix.def").unwrap();
    let trie = bot::morph::Trie::load_from_naist_jdic(&dict_file).unwrap();
    let matrix = bot::morph::build_matrix(&matrix_file).unwrap();
    loop {
        let mut buf = String::new();
        io::stdin().read_line(&mut buf).ok();
        if buf.len() == 0 {
            break
        }
        let (cost, path) = bot::morph::fill_dp(buf.as_bytes(), &trie, &matrix);
        println!("cost {}", cost);
        for (info, begin, end) in path {
            println!("  {} --- {:?}", str::from_utf8(buf.as_bytes().get(begin..end).unwrap()).unwrap(), info);
        }
    }
}
