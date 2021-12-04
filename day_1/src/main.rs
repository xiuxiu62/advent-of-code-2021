#![allow(dead_code)]

use day_1::*;

use std::{error::Error, path::Path};

fn main() -> Result<(), Box<dyn Error>> {
    print!("Part 1: ");
    part_1()?;
    print!("Part 2: ");
    part_2()?;

    Ok(())
}

fn part_1() -> Result<(), Box<dyn Error>> {
    let test_data_path = parse_args();
    let test_data_path = Path::new(&test_data_path);
    let test_data = read_test_data(test_data_path)?;
    let count = count_increases(test_data);

    println!("There are {} increases.", count);
    Ok(())
}

fn part_2() -> Result<(), Box<dyn Error>> {
    let test_data_path = parse_args();
    let test_data_path = Path::new(&test_data_path);
    let test_data = read_test_data(test_data_path)?;
    let test_data = test_data
        .iter()
        .enumerate()
        .map(|(i, n)| {
            if i < test_data.len() - 2 {
                n + test_data[i + 1] + test_data[i + 2]
            } else {
                0
            }
        })
        .collect();

    let count = count_increases(test_data);

    println!("There are {} increases.", count);
    Ok(())
}
