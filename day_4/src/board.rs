use std::{
    fmt::{self, Display},
    num::ParseIntError,
};

#[derive(Debug)]
pub struct Board(Vec<Vec<(u32, bool)>>);

impl From<Vec<String>> for Board {
    fn from(lines: Vec<String>) -> Self {
        let parsed_nums = lines.into_iter().map(|l| {
            l.split_whitespace()
                .map(|w| u32::from_str_radix(w, 10))
                .collect::<Result<Vec<u32>, ParseIntError>>()
                .unwrap()
        });

        Self(
            parsed_nums
                .map(|r| {
                    r.into_iter().fold(vec![], |mut acc, n| {
                        acc.push((n, false));
                        acc
                    })
                })
                .collect::<Vec<Vec<(u32, bool)>>>(),
        )
    }
}

impl AsRef<Vec<Vec<(u32, bool)>>> for Board {
    fn as_ref(&self) -> &Vec<Vec<(u32, bool)>> {
        &self.0
    }
}

impl AsMut<Vec<Vec<(u32, bool)>>> for Board {
    fn as_mut(&mut self) -> &mut Vec<Vec<(u32, bool)>> {
        &mut self.0
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let buf = self.as_ref().iter().fold("".to_owned(), |acc, r| {
            acc + r
                .iter()
                .fold("".to_owned(), |acc, (n, _)| format!("{acc}{n}, "))
                .as_str()
                + "\n"
        });

        write!(f, "{buf}")
    }
}

impl Board {
    // Updates the board with a drawn number, returning true if the board was updated
    pub fn update(&mut self, draw: u32) -> bool {
        self.as_mut()
            .iter_mut()
            .enumerate()
            .fold(false, |acc, (i, row)| {
                acc || row.iter_mut().enumerate().fold(false, |acc, (j, cell)| {
                    if cell.0 == draw && !cell.1 {
                        self.as_mut()[i][j] = true;
                        true
                    } else {
                        acc
                    }
                })
            })
    }

    // Checks for win, returning a sum unmarked cells if true
    pub fn win(&self) -> Option<u32> {
        match self.check_rows() {
            Some(v) => Some(v),
            None => match self.check_cols() {
                Some(v) => Some(v),
                None => self.check_diagonals(),
            },
        }
    }

    fn check_rows(&self) -> Option<u32> {
        let inner_board = self.as_ref();
        for i in 0..5 {
            let mut win = true;
            for j in 0..5 {
                win = win && inner_board[i][j].1;
            }

            if win {
                return Some(self.unmarked_sum());
            }
        }

        None
    }

    fn check_cols(&self) -> Option<u32> {
        let inner_board = self.as_ref();
        for i in 0..5 {
            let mut win = true;
            for j in 0..5 {
                win = win && inner_board[j][i].1;
            }

            if win {
                return Some(self.unmarked_sum());
            }
        }

        None
    }

    fn check_diagonals(&self) -> Option<u32> {
        let inner_board = self.as_ref();
        for diagonal in [
            [(0, 0), (1, 1), (2, 2), (3, 3), (4, 4)],
            [(0, 4), (1, 3), (2, 2), (3, 1), (4, 0)],
        ] {
            let mut win = true;
            for pair in diagonal {
                win = win && inner_board[pair.0][pair.1].1;
            }

            if win {
                return Some(self.unmarked_sum());
            }
        }

        None
    }

    fn unmarked_sum(&self) -> u32 {
        self.as_ref()
            .iter()
            .flatten()
            .filter(|cell| cell.1)
            .fold(0, |acc, (n, _)| acc + n)
    }
}
