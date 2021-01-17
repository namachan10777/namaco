use serde::Serialize;
use serde_derive::{Deserialize, Serialize};

pub struct DictCfg {
    pub surface: usize,
    pub cost: usize,
    pub lid: usize,
    pub rid: usize,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Word<T> {
    pub info: T,
    pub cost: i64,
    pub lid: usize,
    pub rid: usize,
}

fn split_by_comma<'a>(line: &'a str) -> Vec<&'a str> {
    let mut buf = Vec::new();
    const COMMA: u8 = 0x2c;
    const DQUOTE: u8 = 0x22;
    let bytes = line.as_bytes();

    let mut i = 0usize;
    let mut begin = 0usize;

    while i < bytes.len() {
        if bytes[i] == DQUOTE
            && i + 3 < bytes.len()
            && bytes[i + 2] == DQUOTE
            && bytes[i + 3] == COMMA
        {
            unsafe {
                buf.push(line.get_unchecked(i + 1..i + 2));
            }
            i += 4;
            begin = i;
        } else if bytes[i] == COMMA {
            unsafe {
                buf.push(line.get_unchecked(begin..i));
            }
            i += 1;
            begin = i;
        } else {
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

use super::trie;
use core::fmt::Debug;
use std::io;
use std::io::{BufRead, Read};

pub fn build_trie<R: Read, F, T: Serialize + Clone + Debug>(
    readable: R,
    classifier: F,
) -> Result<trie::Trie<Word<T>>, io::Error>
where
    F: Fn(&[&str]) -> (Vec<u8>, Word<T>),
{
    let mut reader = io::BufReader::new(readable);
    let mut buf = String::new();
    let mut dict = Vec::new();
    while reader.read_line(&mut buf)? > 0 {
        let arr: Vec<&str> = split_by_comma(&buf);
        dict.push(classifier(&arr));
        buf.clear();
    }
    Ok(trie::Trie::static_construction(
        &mut dict.iter().map(|x| (&x.0[..], x.1.clone())).collect(),
    ))
}

#[cfg(test)]
mod test_parser {
    use super::*;
    #[test]
    fn test_parser() {
        let csv = "蟹,0,10,100,カニ\n土,1,20,200,ツチ\n味,2,30,300,アジ";
        let result: trie::Trie<Word<String>> = build_trie(csv.as_bytes(), |arr| {
            (
                arr[0].as_bytes().to_vec(),
                Word {
                    info: String::from(arr[4].trim()),
                    lid: arr[1].parse().unwrap(),
                    rid: arr[2].parse().unwrap(),
                    cost: arr[3].parse().unwrap(),
                },
            )
        })
        .unwrap();
        assert_eq!(
            result.find("蟹".as_bytes()),
            Ok(&[Word {
                lid: 0,
                rid: 10,
                cost: 100,
                info: String::from("カニ"),
            }][..])
        );
        assert_eq!(
            result.find("土".as_bytes()),
            Ok(&[Word {
                lid: 1,
                rid: 20,
                cost: 200,
                info: String::from("ツチ"),
            }][..])
        );
        assert_eq!(
            result.find("味".as_bytes()),
            Ok(&[Word {
                lid: 2,
                rid: 30,
                cost: 300,
                info: String::from("アジ"),
            }][..])
        );
    }
}
