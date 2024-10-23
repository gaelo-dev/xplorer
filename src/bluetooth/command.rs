// probablemente cambie esto
use std::fmt::Display;

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

    pub fn add_action(&mut self, action: T) {
        self.actions.push(action);
    }
}

impl<T: Action> Display for Command<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", T::name())?;
        
        for action in &self.actions {
            write!(f, "+{}", action)?;
        }

        write!(f, ";")
    }
}

pub enum Servo {
    Base(u8),
    Elbow(u8),
    Armrest(u8), 
    Grip(u8),
}

impl Display for Servo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Base(degrees) => write!(f, "BASE={degrees}"),
            Self::Elbow(degrees) => write!(f, "ELBOW={degrees}"),
            Self::Armrest(degrees) => write!(f, "ARMREST={degrees}"),
            Self::Grip(degrees) => write!(f, "GRIP={degrees}"),
        }
    }
}

impl Action for Servo {
    fn name() -> &'static str {
        "SVR"
    }
}

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
            Motor::Forward => write!(f, "FORWARD"),
            Motor::Backward => write!(f, "BACKWARD"),
            Motor::Rightward => write!(f, "RIGHTWARD"), 
            Motor::Leftward => write!(f, "LEFTWARD"),
            Motor::Speed(speed) => write!(f, "SPEED={}", speed),
        }
    }
}

impl Action for Motor {
    fn name() -> &'static str {
        "MTR"
    }
}

pub trait Action: Display {
    fn name() -> &'static str;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn command_display1() {
        let mut command = Command::new();
        command.add_action(Servo::Base(90));
        command.add_action(Servo::Grip(90));

        assert_eq!(command.to_string(), "SVR+BASE=90+GRIP=90;")
    }

    #[test]
    fn command_display2() {
        let mut command = Command::new();
        command.add_action(Motor::Speed(100));
        command.add_action(Motor::Forward);

        assert_eq!(command.to_string(), "MTR+SPEED=100+FORWARD;")
    }
}
