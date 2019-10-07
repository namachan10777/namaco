mod trie;
mod matrix;
pub mod parser;
use std::io::{Read, Write};
use std::io;
use serde::{Serialize};
use serde::de::DeserializeOwned;
use serde_derive::{Serialize, Deserialize};

pub use self::parser::Word as Word;

#[derive(Serialize, Deserialize)]
struct Morph<T: Serialize> {
    trie: trie::Trie<Word<T>>,
    matrix: matrix::Matrix,
}

impl<T: Serialize + DeserializeOwned> Morph<T> {
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
        let matrix_src = "0 0 100
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
            Ok(&Word {
                matrix_id: 0,
                gencost: 100,
                word: "蟹".to_string(),
                info: "カニ".to_string(),
            }));
        assert_eq!(
            restored.trie.find("土".as_bytes()),
            Ok(&Word {
                matrix_id: 1,
                gencost: 200,
                word: "土".to_string(),
                info: "ツチ".to_string(),
            }));
        assert_eq!(
            restored.trie.find("味".as_bytes()),
            Ok(&Word {
                matrix_id: 2,
                gencost: 300,
                word: "味".to_string(),
                info: "アジ".to_string(),
            }));
        assert_eq!(restored.matrix.at(0, 1), 121);
        assert_eq!(restored.matrix.at(2, 1), -54);
    }
}
