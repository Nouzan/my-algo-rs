use anyhow::{bail, Result};
use std::io::{self, Write};
use structopt::StructOpt;

/// 求解八皇后问题.
#[derive(StructOpt, Debug)]
struct Opt {
    /// 输入棋盘边长.
    input: usize,
}

#[derive(Debug, Clone, Copy)]
struct Queen(usize, usize);

impl Queen {
    fn conflict(&self, other: &Self) -> bool {
        self.0 == other.0
            || self.1 == other.1
            || (self.0 as isize - other.0 as isize).abs()
                == (self.1 as isize - other.1 as isize).abs()
    }

    fn display_board(queens: &[Queen]) -> Result<String> {
        let mut board = vec![vec!['O'; queens.len()]; queens.len()];
        for queen in queens {
            if let Some(row) = board.get_mut(queen.0) {
                if let Some(cell) = row.get_mut(queen.1) {
                    if *cell != 'O' {
                        bail!("invalid queens");
                    } else {
                        *cell = 'X';
                    }
                } else {
                    bail!("invalid queens");
                }
            } else {
                bail!("invalid_queens");
            }
        }
        let mut result = String::new();
        for row in board {
            for cell in row {
                result.push(cell);
            }
            result.push('\n');
        }
        Ok(result)
    }
}

/// 八皇后问题求解迭代器, 每次迭代输出目前进度, 找到一个解以后迭代终止, `n < 4`或无解时提前终止迭代.
/// 八皇后问题: 输入棋盘边长`n >= 4`, 输出`n`个`Queen`, 它们所在位置不会相互冲突.
#[derive(Debug)]
struct EightQueens {
    len: usize,
    queens: Vec<Queen>,
    next: Queen,
    failed: bool,
}

impl EightQueens {
    fn new(len: usize) -> Self {
        Self {
            len,
            queens: vec![],
            next: Queen(0, 0),
            failed: false,
        }
    }

    fn into_ans(self) -> Result<Vec<Queen>> {
        if self.failed {
            bail!("Failed to find valid queens.")
        }
        Ok(self.queens)
    }
}

impl Iterator for EightQueens {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.failed {
            None
        } else if self.len < 4 {
            self.failed = true;
            None
        } else if self.queens.len() == self.len {
            None
        } else {
            if self.queens.iter().any(|queen| self.next.conflict(queen)) {
                self.next.1 += 1;
                while self.next.1 as usize == self.len {
                    if let Some(queen) = self.queens.pop() {
                        self.next = queen;
                        self.next.1 += 1;
                    } else {
                        self.failed = true;
                        return None;
                    }
                }
            } else {
                self.queens.push(self.next);
                self.next = Queen(self.next.0 + 1, 0);
            }
            Some(self.queens.len())
        }
    }
}

fn main() {
    let opt = Opt::from_args();
    let num = opt.input;
    let mut question = EightQueens::new(num);
    let mut max_progress = 0.0;
    const STAGE: f64 = 0.05;
    let mut count: u64 = 0;
    while let Some(progress) = question.next() {
        let progress = progress as f64 / num as f64;
        if progress > max_progress + STAGE {
            while progress - max_progress > STAGE {
                print!("#");
                io::stdout().flush().unwrap();
                max_progress += STAGE;
            }
        }
        count += 1;
    }
    println!();
    println!("runs: {}", count);
    println!(
        "{}",
        Queen::display_board(&question.into_ans().unwrap()).unwrap()
    );
}
