// probablemente cambie esto
// use std::fmt::Display;

pub trait Action {
    fn command(&self) -> u8;

    fn action(&self) -> u8;
    fn value(&self) -> Option<u8>;

    fn as_bytes(& self) -> Vec<u8> {
        let mut vec = Vec::new();
        vec.push((self.command() << 4) | self.action());

        if let Some(value) = self.value() {
            vec.push(value);
        }

        vec.push(0);
        vec
    }
}

impl Action for String {
    fn command(&self) -> u8 {
        unimplemented!()
    }

    fn action(&self) -> u8 {
        unimplemented!()
    }

    fn value(&self) -> Option<u8> {
        unimplemented!()
    }

    fn as_bytes(&self) -> Vec<u8> {
        let mut vec: Vec<_> = self.bytes().collect();
        vec.push(0);
        vec
    }
}

#[derive(Debug)]
pub enum Servo {
    Base(u8),
    Elbow(u8),
    Armrest(u8), 
    Grip(u8),
}

impl Action for Servo {
    fn command(&self) -> u8 {
        1 << 0
    }

    fn action(&self) -> u8 {
        match self {
            Self::Base(_) => 0 << 0,
            Self::Elbow(_) => 1 << 0,
            Self::Armrest(_) => 1 << 1,
            Self::Grip(_) => 1 << 2,
        }
    }

    fn value(&self) -> Option<u8> {
        Some(match self {
            Self::Base(degrees) => *degrees,
            Self::Elbow(degrees) => *degrees,
            Self::Armrest(degrees) => *degrees,
            Self::Grip(degrees) => *degrees,
        })
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
    fn command(&self) -> u8 {
        1 << 1
    }

    fn action(&self) -> u8 {
        match self {
            Self::Forward => 0 << 0,
            Self::Backward => 1 << 0,
            Self::Rightward => 1 << 1,
            Self::Leftward => 1 << 2,
            Self::Speed(_) => 1 << 3,
        }
    }

    fn value(&self) -> Option<u8> {
        match self {
            Self::Forward => None,
            Self::Backward => None,
            Self::Rightward => None,
            Self::Leftward => None,
            Self::Speed(s) => Some(*s),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn as_bytes() {
       let action = Motor::Speed(100);

       assert_eq!(action.as_bytes(), [0b0010_1000, 0b1100100, 0])
    }
}
