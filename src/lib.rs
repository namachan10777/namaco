mod trie;
mod matrix;
pub mod parser;
use std::io::{Read, Write};
use std::io;
use std::i64;
use std::usize;
use serde::{Serialize};
use serde::de::DeserializeOwned;
use serde_derive::{Serialize, Deserialize};

pub use self::parser::Word as Word;

#[derive(Serialize, Deserialize)]
struct Morph<T: Serialize> {
    trie: trie::Trie<Word<T>>,
    matrix: matrix::Matrix,
}

use core::fmt::Debug;
impl<T: Serialize + DeserializeOwned + Clone + Debug> Morph<T> {
    #[allow(dead_code)]
    pub fn from_text<R: Read, F>(matrix_src: &mut R, dict_src: &mut R, dict_cfg: &parser::DictCfg, classifier: F) -> Result<Self, io::Error> 
        where F: Fn(&[&str]) -> T {
        let trie = parser::build_trie(dict_src, dict_cfg, classifier)?;
        let matrix = matrix::Matrix::new(matrix_src)?;
        Ok(Morph { trie, matrix })
    }

    #[allow(dead_code)]
    pub fn export<W: Write>(&self, target: &mut W) -> Result<(), io::Error> {
        let mut stream = io::BufWriter::new(target);
        stream.write(&bincode::serialize(&self).unwrap())?;
        Ok(())
    }

    #[allow(dead_code)]
    pub fn import<R: Read>(target: &mut R) -> Result<Morph<T>, io::Error> {
        let mut stream = io::BufReader::new(target);
        let mut buf = Vec::new();
        stream.read_to_end(&mut buf)?;
        let trie = bincode::deserialize(&buf);
        Ok(trie.unwrap())
    }

    pub fn parse(&self, input: &[u8]) -> Option<Vec<&Word<T>>> {
        // dp[p] = (cost, lid, word)
        // p    : position of end of word
        // cost : total cost
        // word : None or Word<T>
        let mut dp: Vec<Vec<(i64, Vec<&Word<T>>)>> = Vec::new();
        dp.resize_with(input.len(), || Vec::new());
        // initialize dp
        for end in 1..input.len() {
            if let Ok(words) = self.trie.find(&input[..end]) {
                for word in words {
                    dp[end-1].push((word.gencost, vec![word]));
                }
            }
        }
        // fill dp
        for end in 2..input.len() {
            for begin in 1..end {
                if let Ok(words) = self.trie.find(&input[begin..end]) {
                    for word in words {
                        let mut best: Option<(i64, Vec<&Word<T>>)> = None;
                        for prev in &dp[begin-1] {
                            let join_cost = self.matrix.at(prev.1.last().unwrap().matrix_id, word.matrix_id);
                            let total_cost = prev.0 + word.gencost + join_cost as i64;
                            best = match best {
                                Some(inner) => {
                                    if total_cost < inner.0 {
                                        let mut path = prev.1.clone();
                                        path.push(word);
                                        Some((total_cost, path))
                                    }
                                    else {
                                        Some(inner)
                                    }
                                },
                                None => {
                                    let mut path = prev.1.clone();
                                    path.push(word);
                                    Some((total_cost, path))
                                }
                            }
                        }
                        if let Some(inner) = best {
                            dp[end-1].push(inner);
                        }
                    }
                }
            }
        }

        // select best path
        let mut best: Option<(i64, &Vec<&Word<T>>)> = None;
        for (cost, path) in &dp[input.len()-1] {
            best = match best {
                Some((best_cost, best_path)) =>
                    if *cost < best_cost {
                        Some((*cost, path))
                    }
                    else {
                        Some((best_cost, best_path))
                    },
                None =>
                    Some((*cost, path))
            }
        }

        best.map(|x| x.1.clone())
    }
}

#[cfg(test)]
mod test_morph {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_import_export() {
        let dict_src = "蟹,0,100,カニ\n\
                        土,1,200,ツチ\n\
                        味,2,300,アジ";
        let matrix_src = "3 3
                          0 0 100
                          0 1 121
                          0 2 412
                          1 0 24
                          1 1 -41
                          1 2 412
                          2 0 21
                          2 1 -54
                          2 2 512";
        let cfg = parser::DictCfg {
            word: 0,
            matrix_id: 1,
            gencost: 2,
        };
        let morph = Morph::from_text(&mut Cursor::new(matrix_src.as_bytes()), &mut Cursor::new(dict_src.as_bytes()), &cfg, |arr| arr[3].trim().to_string()).unwrap();
        let mut bytes = Vec::new();
        morph.export(&mut bytes).unwrap();
        let restored = Morph::import(&mut Cursor::new(bytes)).unwrap();
        assert_eq!(
            restored.trie.find("蟹".as_bytes()),
            Ok(&[Word {
                matrix_id: 0,
                gencost: 100,
                word: "蟹".to_string(),
                info: "カニ".to_string(),
            }][..]));
        assert_eq!(
            restored.trie.find("土".as_bytes()),
            Ok(&[Word {
                matrix_id: 1,
                gencost: 200,
                word: "土".to_string(),
                info: "ツチ".to_string(),
            }][..]));
        assert_eq!(
            restored.trie.find("味".as_bytes()),
            Ok(&[Word {
                matrix_id: 2,
                gencost: 300,
                word: "味".to_string(),
                info: "アジ".to_string(),
            }][..]));
        assert_eq!(restored.matrix.at(0, 1), 121);
        assert_eq!(restored.matrix.at(2, 1), -54);
    }
}
