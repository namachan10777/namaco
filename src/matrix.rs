#[allow(dead_code)]
struct Matrix {
    internal: Vec<i32>,
    id_size: usize,
}

use std::io::{Read, BufRead};
use std::io;
use std::cmp;

impl Matrix {
    #[allow(dead_code)]
    pub fn new<R: Read>(file: &mut R) -> Result<Matrix, io::Error> {
        let mut inputs = Vec::new();
        let mut reader = io::BufReader::new(file);
        let mut internal = Vec::new();
        loop {
            let mut buf = String::new();
            if reader.read_line(&mut buf)? == 0 {
                break;
            }
            let splited_line: Vec<&str> = buf.trim().split(' ').collect();
            let lid  = splited_line[0].parse().unwrap();
            let rid  = splited_line[1].parse().unwrap();
            let cost = splited_line[2].parse().unwrap();
            inputs.push((lid, rid, cost));
            internal.push(cost);
        }

        let id_size = inputs
            .iter()
            .map(|(lid, rid, _)| cmp::max(lid, rid))
            .fold(0, | max, x | cmp::max(max, *x));

        Ok(Matrix {
            internal,
            id_size: id_size+1,
        })
    }

    #[allow(dead_code)]
    pub fn at(&self, lid: usize, rid: usize) -> i32 {
        self.internal[lid * self.id_size + rid]
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
