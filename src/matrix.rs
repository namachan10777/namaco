use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Matrix {
    internal: Vec<i32>,
    rsize: usize,
}

use std::io::{Read, BufRead};
use std::io;

impl Matrix {
    #[allow(dead_code)]
    pub fn new<R: Read>(file: &mut R) -> Result<Matrix, io::Error> {
        let mut reader = io::BufReader::new(file);
        let mut internal = Vec::new();
        let mut buf = String::new();
        reader.read_line(&mut buf)?;
        let splited_line: Vec<&str> = buf.trim().split(' ').collect();
        let lsize: usize = splited_line[0].parse().unwrap();
        let rsize: usize = splited_line[1].parse().unwrap();
        internal.resize(lsize * rsize, 0);

        loop{
            buf.clear();
            if reader.read_line(&mut buf)? == 0 {
                break;
            }
            let splited_line: Vec<&str> = buf.trim().split(' ').collect();
            let lid: usize  = splited_line[0].parse().unwrap();
            let rid: usize  = splited_line[1].parse().unwrap();
            let cost: i32 = splited_line[2].parse().unwrap();

            internal[lid * rsize + rid] = cost;
        }

        Ok(Matrix {
            internal,
            rsize,
        })
    }

    #[allow(dead_code)]
    pub fn at(&self, lid: usize, rid: usize) -> i32 {
        self.internal[lid * self.rsize + rid]
    }
}

#[cfg(test)]
mod test_matrix {
    use super::*;
    #[test]
    fn test() {
        let src =
            "0 0 100
            0 1 121
            0 2 412
            1 0 24
            1 1 -41
            1 2 412
            2 0 21
            2 1 -54
            2 2 512";
        let matrix = Matrix::new(&mut io::Cursor::new(src)).unwrap();
        assert_eq!(matrix.at(0, 1), 121);
        assert_eq!(matrix.at(2, 1), -54);
    }
}
