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

    pub fn parse(&self, input: &[u8]) -> Option<Vec<T>> {
        // dp[p] = (cost, lid, word)
        // p    : position of end of word
        // cost : total cost
        // word : None or Word<T>
        let mut dp: Vec<Vec<(i64, Vec<&Word<T>>)>> = Vec::new();
        dp.resize_with(input.len(), || Vec::new());
        // initialize dp
        for end in 1..input.len()+1 {
            if let Ok(words) = self.trie.find(&input[..end]) {
                for word in words {
                    dp[end-1].push((self.matrix.at(0, word.rid) as i64 + word.cost, vec![word]));
                }
            }
        }
        // fill dp
        for end in 2..input.len()+1 {
            for begin in 1..end {
                if let Ok(words) = self.trie.find(&input[begin..end]) {
                    for word in words {
                        let mut best: Option<(i64, Vec<&Word<T>>)> = None;
                        for prev in &dp[begin-1] {
                            let join_cost = self.matrix.at(prev.1.last().unwrap().lid, word.rid);
                            let total_cost = prev.0 + word.cost + join_cost as i64;
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
            let cost  = *cost + self.matrix.at(path.last().unwrap().lid, 0) as i64;
            best = match best {
                Some((best_cost, best_path)) =>
                    if cost < best_cost {
                        Some((cost, path))
                    }
                    else {
                        Some((best_cost, best_path))
                    },
                None =>
                    Some((cost, path))
            }
        }

        best.map(|x| x.1.iter().map(|word| word.info.clone()).collect())
    }
}

#[cfg(test)]
mod test_morph {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_import_export() {
        let dict_src = "蟹,0,10,100,カニ\n\
                        土,1,20,200,ツチ\n\
                        味,2,30,300,アジ";
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
            surface: 0,
            lid: 1,
            rid: 2,
            cost: 3,
        };
        let morph = Morph::from_text(&mut Cursor::new(matrix_src.as_bytes()), &mut Cursor::new(dict_src.as_bytes()), &cfg, |arr| arr[4].trim().to_string()).unwrap();
        let mut bytes = Vec::new();
        morph.export(&mut bytes).unwrap();
        let restored = Morph::import(&mut Cursor::new(bytes)).unwrap();
        assert_eq!(
            restored.trie.find("蟹".as_bytes()),
            Ok(&[Word {
                lid: 0,
                rid: 10,
                cost: 100,
                info: "カニ".to_string(),
            }][..]));
        assert_eq!(
            restored.trie.find("土".as_bytes()),
            Ok(&[Word {
                lid: 1,
                rid: 20,
                cost: 200,
                info: "ツチ".to_string(),
            }][..]));
        assert_eq!(
            restored.trie.find("味".as_bytes()),
            Ok(&[Word {
                lid: 2,
                rid: 30,
                cost: 300,
                info: "アジ".to_string(),
            }][..]));
        assert_eq!(restored.matrix.at(0, 1), 121);
        assert_eq!(restored.matrix.at(2, 1), -54);
    }

    #[test]
    fn test_dp() {
        let dict_src = "\
            東,8,7,6245,東・名詞・ヒガシ\n\
            京,1,1,10791,京・名詞・キョウ\n\
            京都,2,1,2135,京都・名詞・キョウト\n\
            東京,1,1,3003,東京・名詞・トウキョウ\n\
            都,3,2,9428,都・接尾辞・ト\n\
            都,4,3,7595,都・名詞・ト\n\
            に,5,4,11880,に・動詞・ニ\n\
            に,6,5,4304,に・助詞・ニ\n\
            住む,7,6,7048,住む・動詞・スム\n";
        let matrix_src = "\
            9 8
            0 7 -283
            0 1 -310
            8 1 -368
            1 2 -9617
            1 3 -1303
            2 4 1220
            2 5 -3838
            3 4 1387
            3 5 -3573
            4 4 -811
            4 5 -4811
            5 6 -12165
            6 6 -3547
            7 0 -409";
        let cfg = parser::DictCfg {
            surface: 0,
            lid: 1,
            rid: 2,
            cost: 3,
        };
        let morph =
            Morph::from_text(&mut Cursor::new(matrix_src.as_bytes()), &mut Cursor::new(dict_src.as_bytes()), &cfg,
                |arr| arr[4].trim().to_string()
            ).unwrap();
        assert_eq!(morph.parse("東京都に住む".as_bytes()), Some(vec![
            "東京・名詞・トウキョウ".to_string(),
            "都・接尾辞・ト".to_string(),
            "に・助詞・ニ".to_string(),
            "住む・動詞・スム".to_string(),
        ]));
    }
}
