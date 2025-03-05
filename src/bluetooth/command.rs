/// Implementation of the command system, which is what the `Xplorer` interprets
use std::ops::Add;

use super::ToBytes;

/// Represents a command for the `Xplorer`
/// 
/// The ideal way to create this type is using the modules:
/// 1. [`car`] -> commands for motor control
/// 2. [`arm`] -> commands for robotic arm control
#[derive(Debug, Clone, Copy, Default)]
pub struct Command {
    cmd: u8,
    action: u8,
    value: Option<u8>,
}

impl Add for Command {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        assert_eq!(self.cmd, rhs.cmd);

        Self {
            cmd: self.cmd,
            action: self.action | rhs.action,
            value: rhs.value.or(self.value)
        }
    }
}

impl PartialEq for Command {
    fn eq(&self, other: &Self) -> bool {
        self.cmd == other.cmd && self.action == other.action
    }
}

impl PartialEq<u8> for Command {
    fn eq(&self, other: &u8) -> bool {
        self.cmd == *other
    }
}

impl PartialEq<Command> for u8 {
    fn eq(&self, other: &Command) -> bool {
        other.eq(self)
    }
}

impl From<Vec<u8>> for Command {
    fn from(value: Vec<u8>) -> Self {
        let mut iter = value.iter();

        let byte = *iter.next().unwrap();
        let mut s = Self { 
            cmd: byte >> 6, 
            action: byte & 0b00_111111, 
            value: None,
        };

        for byte in iter {
            if *byte == 0 {
                return s
            } 

            s.value = Some(*byte)
        }

        s
    }
}

impl ToBytes for Command {
    fn to_bytes(&self) -> Vec<u8> {
        let mut vec = Vec::with_capacity(3);
        vec.push((self.cmd << 6) | self.action);

        if let Some(value) = self.value {
            vec.push(value);
        }

        vec.push(0);
        vec
    }
}

/// A macro that creates a module with functions to create commands for a specific control like `car`
macro_rules! create_command {
    ( $name:ident => $cmd:expr; $( $action:ident => $code:expr $(,$value:ident)? ;)* ) => {
        pub mod $name {
            use super::{Command};
            
            pub const CMD: u8 = $cmd;

            $(
                pub fn $action( $($value: u8)? ) -> Command {
                    $( 
                        return Command {
                            cmd: CMD,
                            action: $code,
                            value: Some($value)
                        };
                    )?

                    #[allow(unreachable_code)]
                    Command {
                        cmd: CMD,
                        action: $code,
                        value: None
                    }
                }
            )*
        }
    };
}

create_command!(
    car => 1;
    forward => 1 << 0;
    backward => 1 << 1;
    rightward => 1 << 2;
    leftward => 1 << 3;
    speed => 1 << 4, value;
);

create_command!(
    arm => 2;
    base => 1 << 0, grades;
    elbow => 1 << 1, grades;
    rest => 1 << 2, grades;
    grip => 1 << 3, grades;
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_bytes() {
        let a = car::forward();
        let b = car::speed(100);

        let c = a + b;

        assert_eq!(c.to_bytes(), vec![0b01_010001, 0b1100100, 0])
    }

    #[test]
    fn assertions() {
        let a = car::speed(100);
        let b = car::CMD;
        let c = car::speed(200);

        // transitivity 
        assert_eq!(a, b); assert_eq!(b, c); assert_eq!(a, c);

        assert_eq!(car::forward(), car::forward()); // they are the same
        assert_eq!(car::speed(100), car::speed(200)); // values ​​are not compared
        assert_ne!(car::forward(), arm::base(90)); // the commands are not the same
        assert_ne!(arm::base(80), arm::elbow(80)); // the actions are not the same
        assert_eq!(car::forward(), car::CMD); // the command matches
    }
}
