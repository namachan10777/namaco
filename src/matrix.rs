use serde_derive::{Serialize, Deserialize};

#[derive(Fail, Debug)]
pub enum MatrixLoadError {
    #[fail(display = "failed to parse line at {}", line)]
    FailedToParseLine {
        line: usize,
    },
    #[fail(display = "failed to read line at {}", line)]
    FailedToReadLine {
        line: usize,
    },
    #[fail(display = "invalid header at {} ", line)]
    InvalidHeader {
        line: usize,
    },
    #[fail(display = "invalid column at {} ", line)]
    InvalidColumn {
        line: usize,
    },

}

#[derive(Serialize, Deserialize)]
pub struct Matrix {
    internal: Vec<i32>,
    rsize: usize,
}

use std::io::{Read, BufRead};
use std::io;

impl Matrix {
    pub fn new<R: Read>(file: &mut R) -> Result<Matrix, MatrixLoadError> {
        let mut reader = io::BufReader::new(file);
        let mut internal = Vec::new();
        let mut buf = String::new();
        let mut line_cnt = 1usize;

        reader.read_line(&mut buf).map_err(|_| MatrixLoadError::FailedToReadLine { line: line_cnt })?;
        let splited_line: Vec<&str> = buf.trim().split(' ').collect();
        if splited_line.len() != 2 {
            return Err(MatrixLoadError::InvalidHeader { line: line_cnt });
        }
        let lsize: usize = splited_line[0].parse().map_err(|_| MatrixLoadError::FailedToParseLine { line: line_cnt })?;
        let rsize: usize = splited_line[1].parse().map_err(|_| MatrixLoadError::FailedToParseLine { line: line_cnt })?;

        internal.resize(lsize * rsize, std::i32::MAX);

        loop{
            line_cnt += 1;
            buf.clear();
            if reader.read_line(&mut buf).map_err(|_| MatrixLoadError::FailedToReadLine { line: line_cnt })? == 0 {
                break;
            }
            let splited_line: Vec<&str> = buf.trim().split(' ').collect();
            if splited_line.len() != 3 {
                return Err(MatrixLoadError::InvalidColumn { line: line_cnt });
            }
            let lid: usize  = splited_line[0].parse().map_err(|_| MatrixLoadError::FailedToParseLine { line: line_cnt })?;
            let rid: usize  = splited_line[1].parse().map_err(|_| MatrixLoadError::FailedToParseLine { line: line_cnt })?;
            let cost: i32 = splited_line[2].parse().map_err(|_| MatrixLoadError::FailedToParseLine { line: line_cnt })?;

            if lid >= lsize || rid >= rsize {
                return Err(MatrixLoadError::InvalidColumn { line: line_cnt });
            }

            internal[lid * rsize + rid] = cost;
        }

        Ok(Matrix {
            internal,
            rsize,
        })
    }

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
            "3 3
            0 0 100
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
