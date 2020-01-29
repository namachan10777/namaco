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
        .subcommand(clap::SubCommand::with_name("repl")
            .arg(clap::Arg::with_name("DICT")
                .required(true))
            .help("pass compiled dictionary"))
        .get_matches();
    if let Some(matches) = matches.subcommand_matches("compile") {
        let mut dict_file = fs::File::open(matches.value_of("DICT").unwrap()).unwrap();
        let mut matrix_file = fs::File::open(matches.value_of("MATRIX").unwrap()).unwrap();
        let mut output_file = fs::File::create(matches.value_of("OUTPUT").unwrap()).unwrap();
        let morph = namaco::Morph::from_text(
            &mut matrix_file,
            &mut dict_file,
            |arr| (
                arr[0].as_bytes().to_vec(),
                namaco::parser::Word {
                    info: arr.join(","),
                    lid: arr[1].parse().unwrap(),
                    rid: arr[2].parse().unwrap(),
                    cost: arr[3].parse().unwrap(),
                }
            )
        ).unwrap();
        morph.export(&mut output_file).unwrap();
    }
    else if let Some(matches) = matches.subcommand_matches("repl") {
        let morph: namaco::Morph<String> = namaco::Morph::import(&mut fs::File::open(matches.value_of("DICT").unwrap()).unwrap()).unwrap();
        let mut buf = String::new();
        loop {
            buf.clear();
            if std::io::stdin().read_line(&mut buf).ok() == Some(0) {
                break;
            }
            match morph.parse(buf.trim().as_bytes()) {
                Some(arr) => {
                    for x in arr {
                        println!("{:?}", x);
                    }
                },
                None => {
                    println!("failed to parse");
                }
            }
        }
    }
}
