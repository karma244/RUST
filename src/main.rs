use io::Write;
use std::{io, str, io::stdin, io::stdout};

pub struct Scanner<R> {
    reader: R,
    buf_str: Vec<u8>,
    buf_iter: str::SplitAsciiWhitespace<'static>,
}

impl<R: io::BufRead> Scanner<R> {
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            buf_str: vec![],
            buf_iter: "".split_ascii_whitespace(),
        }
    }

    pub fn token<T: str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buf_iter.next() {
                return token.parse().ok().unwrap();
            }
            self.buf_str.clear();
            self.reader
                .read_until(b'\n', &mut self.buf_str)
                .unwrap();
            self.buf_iter = unsafe {
                let slice = str::from_utf8_unchecked(&self.buf_str);
                std::mem::transmute(slice.split_ascii_whitespace())
            }
        }
    }
}


//BOJ 2562
fn main() {
    let mut scan = Scanner::new(stdin().lock());
    let mut out = io::BufWriter::new(stdout().lock());

    let mut arr = vec![0; 9];

    for i in 0..9 {
        let x = scan.token::<i32>();
        arr[i] = x;
    }

    let mut max = i32::MIN;
    let mut idx = 0;
    //GG
    for i in 0..9 as usize {
        if arr[i] > max {
            max = arr[i];
            idx = i + 1;
        }
    }
    //GG
    writeln!(out, "{}\n{}", max, idx).unwrap();
}