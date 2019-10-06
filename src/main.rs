// copyright (c) 2019 Nakano Masaki <namachan10777@gmail.com>

use clap;
use namaco::parser;
use std::fs;

fn main() {
    let matches = clap::App::new("namaco")
        .version("0.0.1")
        .author("Nakano Masaki<namachan10777@gmail.com>")
        .about("morphological analyzer")
        .arg(clap::Arg::with_name("DICT")
            .help("set naist-jdic.csv")
            .required(true)
            .index(1))
        .get_matches();
    let file = fs::File::open(matches.value_of("DICT").unwrap()).unwrap();
    let cfg = parser::DictCfg {
        word: 0,
        matrix_id: 2,
        gencost: 4,
    };
    let _trie = parser::build_trie(&file, &cfg, |arr| arr[3].trim().to_string()).unwrap();
}
