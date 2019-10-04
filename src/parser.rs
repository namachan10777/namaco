pub struct DictCfg {
    pub matrix_id: usize,
    pub word: usize,
    pub gencost: usize,
}

#[derive(Debug, PartialEq)]
pub struct Word<T> {
    info: T,
    word: String,
    gencost: i64,
    matrix_id: usize,
}


#[allow(dead_code)]
pub fn parse_line<F, T>(cfg: &DictCfg, classifier: F, line: &str) -> (Vec<u8>, Word<T>)
    where F: Fn(&[&str]) -> T
{
    let arr: Vec<&str> = line.split(',').collect();
    let matrix_id: usize = arr[cfg.matrix_id].parse().unwrap();
    let gencost: i64 = arr[cfg.gencost].parse().unwrap();
    let word_str : String = arr[cfg.word].to_string();
    let info: T = classifier(&arr);
    let word = Word {
        info,
        word: word_str.clone(),
        gencost,
        matrix_id,
    };
    let key = word_str.as_bytes().to_vec();
    (key, word)
}

use std::io;
use std::io::{BufRead, Read};
use super::trie;

#[allow(dead_code)]
pub fn build_trie<R: Read, F, T>(readable: R, cfg: &DictCfg, classifier: F) -> Result<trie::Trie<Word<T>>, io::Error>
    where F: Fn(&[&str]) -> T
{
    let mut reader = io::BufReader::new(readable);
    let mut buf = String::new();
    let mut trie = trie::Trie::new();
    while reader.read_line(&mut buf)? > 0 {
        let (key, info) = parse_line(&cfg, &classifier, &buf);
        trie.add(&key, info).unwrap();
        buf.clear();
    }
    Ok(trie)
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
        let result: trie::Trie<Word<String>> =
            build_trie(csv.as_bytes(), &cfg, |arr| arr[3].trim().to_string()).unwrap();
        assert_eq!(result.find("蟹".as_bytes()),
            Ok(&Word {
                matrix_id: 0,
                gencost: 100,
                word: "蟹".to_string(),
                info: "カニ".to_string(),
            }));
        assert_eq!(result.find("土".as_bytes()),
            Ok(&Word {
                matrix_id: 1,
                gencost: 200,
                word: "土".to_string(),
                info: "ツチ".to_string(),
            }));
        assert_eq!(result.find("味".as_bytes()),
            Ok(&Word {
                matrix_id: 2,
                gencost: 300,
                word: "味".to_string(),
                info: "アジ".to_string(),
            }));
    }
}
