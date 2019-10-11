// copyright (c) 2019 Nakano Masaki <namachan10777@gmail.com>

use clap;
use std::fs;

fn main() {
    let matches = clap::App::new("namaco")
        .version("0.0.1")
        .author("Nakano Masaki<namachan10777@gmail.com>")
        .about("morphological analyzer")
        .subcommand(clap::SubCommand::with_name("compile")
                    .about("compile assets")
                    .arg(clap::Arg::with_name("DICT")
                         .short("d")
                         .long("dict")
                         .value_name("DICT")
                         .required(true)
                         .help("pass csv file for mecab"))
                    .arg(clap::Arg::with_name("MATRIX")
                         .short("m")
                         .long("matrix")
                         .value_name("MATRIX")
                         .required(true)
                         .help("pass matrix file for mecab"))
                    .arg(clap::Arg::with_name("OUTPUT")
                         .short("o")
                         .long("output")
                         .value_name("OUTPUT")
                         .required(true)
                         .help("specify output file name")))
        .arg(clap::Arg::with_name("ASSET")
            .help("pass asset file")
            .index(1))
        .get_matches();
    if let Some(matches) = matches.subcommand_matches("compile") {
        let mut dict_file = fs::File::open(matches.value_of("DICT").unwrap()).unwrap();
        let mut matrix_file = fs::File::open(matches.value_of("MATRIX").unwrap()).unwrap();
        let mut output_file = fs::File::create(matches.value_of("OUTPUT").unwrap()).unwrap();
        let cfg = namaco::parser::DictCfg {
            gencost: 3,
            matrix_id: 1,
            word: 0,
        };
        let morph = namaco::Morph::from_text(&mut matrix_file, &mut dict_file, &cfg, |arr| arr.join(",")).unwrap();
        morph.export(&mut output_file).unwrap();
    }
}
