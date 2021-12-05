use lib_day_2::{Command, Submarine};

use std::{env, error::Error, fs, path::Path, process};

fn main() -> Result<(), Box<dyn Error>> {
    let test_data_path = parse_args();
    let test_data_path = Path::new(&test_data_path);
    let test_data = read_test_data(test_data_path)?;

    let mut submarine = Submarine::default();
    test_data
        .into_iter()
        .map(|d| Command::try_from(d).unwrap())
        .for_each(|c| submarine.r#move(c));

    println!("{submarine:?}");
    Ok(())
}

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

pub fn read_test_data(path: &Path) -> Result<Vec<String>, Box<dyn Error>> {
    Ok(fs::read_to_string(path)?
        .lines()
        .map(|l| l.to_owned())
        .collect::<Vec<String>>())
}
