// probablemente cambie esto
use std::fmt::Display;
use regex::Regex;

#[derive(Debug)]
pub struct Command<T: Action> {
    actions: Vec<T>
}

impl<T: Action> Command<T> {
    pub fn new() -> Self {
        Self { actions: Vec::new() }
    }

    pub fn with_vec(actions: Vec<T>) -> Self {
        Self { actions }
    }

    pub fn from(s: String) -> Self {
        s.into()
    }

    pub fn add_action(&mut self, action: T) {
        self.actions.push(action);
    }
}

impl<T: Action> Display for Command<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", T::command_name())?;
        
        for action in &self.actions {
            write!(f, "+{}", action)?;
        }

        write!(f, ";")
    }
}

impl<T: Action> From<String> for Command<T> {
    fn from(value: String) -> Self {
        let mut actions = Vec::new();
        
        let mut iter = value.split(&['+', ';']);
        let _ = iter.next().unwrap();

        for s in iter {
            if s.is_empty() {
                continue;
            }

            actions.push(T::from_string(s.to_string()));
        }
        
        Self { actions }
    }
}

pub trait Action: Display {
    fn command_name() -> &'static str
    where Self: Sized;

    fn from_str(s: &str) -> impl Fn(u8) -> Self
    where Self: Sized;
    
    fn from_string(value: String) -> Self
    where Self: Sized
    {
        let re = Regex::new(r"(?x)
            (?P<action>[A-z])
            =?
            (?P<value>\d*)
        ").unwrap();

        let caps = re.captures(&value).unwrap();
        let value = caps.name("value")
            .map_or("", |m| m.as_str())
            .parse::<u8>().unwrap_or(0);
        
        let s = Self::from_str(&caps["action"])(value);
        s
    }
}

#[derive(Debug)]
pub enum Servo {
    Base(u8),
    Elbow(u8),
    Armrest(u8), 
    Grip(u8),
}

impl Display for Servo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Base(degrees) => write!(f, "B={degrees}"),
            Self::Elbow(degrees) => write!(f, "E={degrees}"),
            Self::Armrest(degrees) => write!(f, "A={degrees}"),
            Self::Grip(degrees) => write!(f, "G={degrees}"),
        }
    }
}

impl Action for Servo {
    fn command_name() -> &'static str {
        "S"   
    }

    fn from_str(s: &str) -> impl Fn(u8) -> Self {
        match s {
            "B" => Self::Base,
            "E" => Self::Elbow,
            "A" => Self::Armrest,
            "G" => Self::Grip,
            _ => panic!()
        }
    }
}

#[derive(Debug)]
pub enum Motor {
    Forward,
    Backward,
    Rightward,
    Leftward,
    Speed(u8),
}

impl Display for Motor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Motor::Forward => write!(f, "F"),
            Motor::Backward => write!(f, "B"),
            Motor::Rightward => write!(f, "R"), 
            Motor::Leftward => write!(f, "L"),
            Motor::Speed(speed) => write!(f, "S={}", speed),
        }
    }
}

impl Action for Motor {
    fn command_name() -> &'static str {
        "M"
    }

    fn from_str(s: &str) -> impl Fn(u8) -> Self {
        match s {
            "F" => |_| Self::Forward,
            "B" => |_| Self::Backward,
            "R" => |_| Self::Rightward,
            "L" => |_| Self::Leftward,
            "S" => Self::Speed,
            _ => panic!()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn command_display1() {
        let mut command = Command::new();
        command.add_action(Servo::Base(90));
        command.add_action(Servo::Grip(90));

        assert_eq!(command.to_string(), "S+B=90+G=90;")
    }

    #[test]
    fn command_display2() {
        let mut command = Command::new();
        command.add_action(Motor::Speed(100));
        command.add_action(Motor::Forward);

        assert_eq!(command.to_string(), "M+S=100+F;")
    }

    #[test]
    fn from_string_to_command() {
        //let command: Command<Motor> = "M+S=100+F;".to_string().into();
        let command: Command<Motor> = Command::from("M+S=100+F;".to_string());

        assert_eq!(command.to_string(), "M+S=100+F;")
    }
}
