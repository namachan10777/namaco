use serde_derive::{Serialize, Deserialize};
use serde::Serialize;

pub struct DictCfg {
    pub matrix_id: usize,
    pub word: usize,
    pub gencost: usize,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Word<T> {
    pub info: T,
    pub word: String,
    pub gencost: i64,
    pub matrix_id: usize,
}

fn split_by_comma<'a>(line: &'a str) -> Vec<&'a str> {
    let mut buf = Vec::new();
    const COMMA: u8 = 0x2c;
    const DQUOTE: u8 = 0x22;
    let bytes = line.as_bytes();

    let mut i = 0usize;
    let mut begin = 0usize;

    while i < bytes.len() {
        if bytes[i] == DQUOTE && i + 3 < bytes.len() && bytes[i+2] == DQUOTE && bytes[i+3] == COMMA {
            unsafe {
                buf.push(line.get_unchecked(i+1..i+2));
            }
            i += 4;
            begin = i;
        }
        else if bytes[i] == COMMA {
            unsafe {
                buf.push(line.get_unchecked(begin..i));
            }
            i += 1;
            begin = i;
        }
        else {
            i += 1;
        }
    }
    unsafe {
        buf.push(line.get_unchecked(begin..));
    }

    buf
}
#[cfg(test)]
mod test_split_by_comma {
    use super::*;
    #[test]
    fn test_split_by_comma() {
        assert_eq!(split_by_comma("a,b,c"), vec!["a", "b", "c"]);
        assert_eq!(split_by_comma(",a,b,,c,"), vec!["", "a", "b", "", "c", ""]);
        assert_eq!(split_by_comma("\"\"\",\",\",a"), vec!["\"", ",", "a"]);
    }
}

#[allow(dead_code)]
pub fn parse_line<F, T>(cfg: &DictCfg, classifier: F, line: &str) -> (Vec<u8>, Word<T>)
    where F: Fn(&[&str]) -> T
{
    let arr: Vec<&str> = split_by_comma(line);
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
pub fn build_trie<R: Read, F, T: Serialize>(readable: R, cfg: &DictCfg, classifier: F) -> Result<trie::Trie<Word<T>>, io::Error>
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
