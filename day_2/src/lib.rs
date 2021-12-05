#[derive(Debug, Clone, Copy)]
pub enum Command {
    Up(i32),
    Down(i32),
    Forward(i32),
}

impl TryFrom<String> for Command {
    type Error = String;

    fn try_from(command: String) -> Result<Self, Self::Error> {
        let (command, amount) = {
            let parts: Vec<&str> = command.split_whitespace().collect();
            (parts[0], parts[1].parse::<i32>().unwrap())
        };

        match command {
            "up" => Ok(Self::Up(amount)),
            "down" => Ok(Self::Down(amount)),
            "forward" => Ok(Self::Forward(amount)),
            _ => Err("Failed to parse command".to_string()),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Submarine {
    horizontal_position: i32,
    depth: i32,
    aim: i32,
}

impl Submarine {
    pub fn new(horizontal_position: i32, depth: i32, aim: i32) -> Self {
        Self {
            horizontal_position,
            depth,
            aim,
        }
    }

    pub fn r#move(&mut self, command: Command) {
        match command {
            Command::Up(val) => self.aim -= val,
            Command::Down(val) => self.aim += val,
            Command::Forward(val) => {
                self.horizontal_position += val;
                self.depth += self.aim * val;
            }
        }
    }
}

impl Default for Submarine {
    fn default() -> Self {
        Self::new(0, 0, 0)
    }
}
