use std::{env, error::Error, fs, num::ParseIntError, path::Path, process};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().skip(1).collect();

    match args.len() {
        1 => run(&args[0]),
        _ => {
            println!("Please provide a single test data file.");
            process::exit(1);
        }
    }
}

fn run(path: &str) -> Result<(), Box<dyn Error>> {
    let test_data_path = Path::new(path);
    let test_data = read_test_data(test_data_path)?;
    let count = count_increases(test_data);

    println!("There are {} increases.", count);
    Ok(())
}

fn read_test_data(path: &Path) -> Result<Vec<u32>, Box<dyn Error>> {
    Ok(fs::read_to_string(path)?
        .lines()
        .map(|l| l.parse())
        .collect::<Result<Vec<u32>, ParseIntError>>()?)
}

fn count_increases(depths: Vec<u32>) -> u32 {
    depths.iter().enumerate().fold(0_u32, |acc, (i, n)| {
        if i < depths.len() - 1 {
            if depths[i + 1] > *n {
                return acc + 1;
            }
        }

        acc
    })
}
