mod board;

use board::Board;

use std::{error::Error, num::ParseIntError};

pub fn main() -> Result<(), Box<dyn Error>> {
    let mut lines = include_str!("../test.data").lines().map(|l| l.to_owned());

    let drawn_set: Vec<u32> = lines
        .next()
        .unwrap()
        .split(",")
        .map(|w| u32::from_str_radix(w, 10))
        .collect::<Result<Vec<u32>, ParseIntError>>()?;

    let mut boards: Vec<Board> = lines
        .skip(1)
        .fold(
            (vec![vec![]], 0),
            |(mut tables, i): (Vec<Vec<String>>, usize), l| match &*l {
                "" => {
                    tables.push(vec![]);
                    (tables, i + 1)
                }
                _ => {
                    tables[i].push(l);
                    (tables, i)
                }
            },
        )
        .0
        .into_iter()
        .map(|pre_board| Board::from(pre_board))
        .collect();

    for draw in drawn_set.into_iter() {
        for board in boards.iter_mut() {
            if board.update(draw) {
                match board.win() {
                    Some(result) => {
                        println!("Result: {}", draw * result);
                        return Ok(());
                    }
                    None => {}
                }
            }
        }
    }

    Ok(())
}
