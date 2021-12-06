use std::{error::Error, num::ParseIntError};

#[derive(Debug)]
struct Table(Vec<Vec<u32>>);

impl From<Vec<String>> for Table {
    fn from(lines: Vec<String>) -> Self {
        Self(
            lines
                .into_iter()
                .map(|l| {
                    l.split_whitespace()
                        .map(|w| u32::from_str_radix(w, 10))
                        .collect::<Result<Vec<u32>, ParseIntError>>()
                        .unwrap()
                })
                .collect::<Vec<Vec<u32>>>(),
        )
    }
}

pub fn main() -> Result<(), Box<dyn Error>> {
    let mut lines = include_str!("../test.data").lines().map(|l| l.to_owned());

    let drawn_set: Vec<u32> = lines
        .next()
        .unwrap()
        .split(",")
        .map(|w| u32::from_str_radix(w, 10))
        .collect::<Result<Vec<u32>, ParseIntError>>()?;

    let tables: Vec<Table> = lines
        .skip(2)
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
        .map(|pre_table| Table::from(pre_table))
        .collect();

    Ok(())
}
