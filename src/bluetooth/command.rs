// probablemente cambie esto
// use std::fmt::Display;

pub struct Command {
    actions: Vec<u8>,
}

impl Command {
    pub fn new<T: Action>() -> Self {
        Self { 
            actions: vec![T::command()]
        }
    }

    pub fn with_vec<T: Action>(actions: Vec<T>) -> Self {
        let mut vec = vec![T::command()];
        vec.append(&mut actions.into_iter().map(|a| a.as_byte()).collect());

        Self { 
            actions: vec
        }
    }

    pub fn add_action<T: Action>(&mut self, action: T) {
        self.actions.push(action.as_byte());
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.actions
    }
}

pub trait Action {
    fn command() -> u8
    where Self: Sized;

    fn action(&self) -> u8;
    fn value(&self) -> u8;

    fn as_byte(&self) -> u8 {
        let action = self.action() / 2;
        let val = ((self.value() as u16 * 15 + 50) / 180) as u8;

        println!("action: {action}\nvalue:{val}");
        
        (action << 4) | val
    }

    /*
    let extract_command = message >> 4;
    let extract_velocity = message & 0b0000_1111;


    let real_velocity = (extract_velocity as u32 * 100 + 4) / 15;
    */
}

#[derive(Debug)]
pub enum Servo {
    Base(u8),
    Elbow(u8),
    Armrest(u8), 
    Grip(u8),
}

impl Action for Servo {
    fn command() -> u8 {
        1 << 0
    }

    fn action(&self) -> u8 {
        match self {
            Self::Base(_) => 1 << 0,
            Self::Elbow(_) => 1 << 1,
            Self::Armrest(_) => 1 << 2,
            Self::Grip(_) => 1 << 3,
        }
    }

    fn value(&self) -> u8 {
        match self {
            Self::Base(degrees) => *degrees,
            Self::Elbow(degrees) => *degrees,
            Self::Armrest(degrees) => *degrees,
            Self::Grip(degrees) => *degrees,
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

impl Action for Motor {
    fn command() -> u8 {
        1 << 1
    }

    fn action(&self) -> u8 {
        match self {
            Self::Forward => 1 << 0,
            Self::Backward => 1 << 1,
            Self::Rightward => 1 << 2,
            Self::Leftward => 1 << 3,
            Self::Speed(_) => 1 << 4,
        }
    }

    fn value(&self) -> u8 {
        match self {
            Self::Forward => 0,
            Self::Backward => 0,
            Self::Rightward => 0,
            Self::Leftward => 0,
            Self::Speed(s) => *s,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn as_bytes() {
       let mut command = Command::new::<Servo>();
       command.add_action(Servo::Base(45)); 
       command.add_action(Servo::Grip(45));

        assert_eq!(command.as_bytes(), &[1 << 0, 0b0000_0111, 0b0100_0111])
    }
}
