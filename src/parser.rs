pub struct DictCfg {
    matrix_id: usize,
    word: usize,
    gencost: usize,
}

#[derive(Debug, PartialEq)]
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

use std::io;
use std::io::{BufRead, Read};

#[allow(dead_code)]
pub fn parse_csv_dict<R: Read, F, T>(readable: R, cfg: &DictCfg, classifier: F) -> Result<Vec<Word<T>>, io::Error>
    where F: Fn(&[&str]) -> T
{
    let mut reader = io::BufReader::new(readable);
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
    use super::*;
    #[test]
    fn test_parser () {
        let csv = "蟹,0,100,カニ\n土,1,200,ツチ\n味,2,300,アジ";
        let cfg = DictCfg {
            word: 0,
            matrix_id: 1,
            gencost: 2,
        };
        let result: Result<Vec<Word<String>>, _> =
            parse_csv_dict(csv.as_bytes(), &cfg, |arr| arr[3].trim().to_string());
        assert_eq!( result.unwrap(), vec![
            Word {
                matrix_id: 0,
                gencost: 100,
                word: "蟹".to_string(),
                info: "カニ".to_string(),
            },
            Word {
                matrix_id: 1,
                gencost: 200,
                word: "土".to_string(),
                info: "ツチ".to_string(),
            },
            Word {
                matrix_id: 2,
                gencost: 300,
                word: "味".to_string(),
                info: "アジ".to_string(),
            },
        ]);
    }
}
