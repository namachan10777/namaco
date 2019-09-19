pub struct DictCfg {
    matrix_id: usize,
    word: usize,
    gencost: usize,
}

#[allow(dead_code)]
pub struct Word<T> {
    info: T,

    word: String,

    gencost: i64,
    matrix_id: usize,
}


#[allow(dead_code)]
pub fn parse_naist_jdic_by_line<F, T>(cfg: &DictCfg, classifier: F, line: &str) -> Word<T>
    where F: Fn(&[&str]) -> T
{
    let arr: Vec<&str> = line.split(',').collect();
    let matrix_id: usize = arr[cfg.matrix_id].parse().unwrap();
    let gencost: i64 = arr[cfg.gencost].parse().unwrap();
    let word : String = arr[cfg.word].to_string();
    let info: T = classifier(&arr);
    Word {
        info,
        word,
        gencost,
        matrix_id,
    }
}

use std::fs;
use std::io;
use std::io::{BufRead};

#[allow(dead_code)]
pub fn parse_csv_dict<F, T>(file: &fs::File, cfg: &DictCfg, classifier: F) -> Result<Vec<Word<T>>, io::Error>
    where F: Fn(&[&str]) -> T
{
    let mut reader = io::BufReader::new(file);
    let mut buf = String::new();
    let mut result = Vec::new();
    while reader.read_line(&mut buf)? > 0 {
        result.push(parse_naist_jdic_by_line(&cfg, &classifier, &buf));
        buf.clear();
    }
    Ok(result)
}

#[cfg(test)]
mod test_parser {
    #[test]
    fn test_parser () {
    }
}
