use std::{env, error::Error, fs, num::ParseIntError, path::Path, process};

pub fn parse_args() -> String {
    let args: Vec<String> = env::args().skip(1).collect();

    match args.len() {
        1 => args[0].to_owned(),
        _ => {
            println!("Please provide a single test data file.");
            process::exit(1);
        }
    }
}

pub fn read_test_data(path: &Path) -> Result<Vec<u32>, Box<dyn Error>> {
    Ok(fs::read_to_string(path)?
        .lines()
        .map(|l| l.parse())
        .collect::<Result<Vec<u32>, ParseIntError>>()?)
}

pub fn count_increases(depths: Vec<u32>) -> u32 {
    depths.iter().enumerate().fold(0_u32, |acc, (i, n)| {
        if i < depths.len() - 1 {
            if depths[i + 1] > *n {
                return acc + 1;
            }
        }

        acc
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::path::Path;

    #[test]
    fn part_1() -> Result<(), Box<dyn Error>> {
        let test_data_path = parse_args();
        let test_data_path = Path::new(&test_data_path);
        let test_data = read_test_data(test_data_path)?;
        let count = count_increases(test_data);

        println!("There are {} increases.", count);
        Ok(())
    }

    #[test]
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
}
