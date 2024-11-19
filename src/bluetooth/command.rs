// probablemente cambie esto

use std::ops::Add;
// pub use actions::*;

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

impl From<&[u8]> for Command {
    fn from(value: &[u8]) -> Self {
        let mut iter = value.iter();

        let byte = *iter.next().unwrap();
        let mut s = Self { 
            cmd: byte >> 5, 
            action: byte & 0b000_11111, 
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

pub trait ToBytes {
    fn to_bytes(&self) -> Vec<u8>;
}

impl ToBytes for Command {
    fn to_bytes(&self) -> Vec<u8> {
        let mut vec = Vec::new();
        vec.push((self.cmd << 5) | self.action);

        if let Some(value) = self.value {
            vec.push(value);
        }

        vec.push(0);
        vec
    }
}

impl ToBytes for String {
    fn to_bytes(&self) -> Vec<u8> {
        let mut vec: Vec<_> = self.bytes().collect();
        vec.push(0);
        vec
    }
}

macro_rules! create_command {
    ( $name:ident => $cmd:expr; $( $action:ident => $code:expr $(,$value:ident)? ;)* ) => {
        pub mod $name {
            use super::{Command};
            
            $(
                pub fn $action( $($value: u8)? ) -> Command {
                    #[allow(unused_mut)]
                    let mut cmd = Command {
                        cmd: $cmd,
                        action: $code,
                        value: None
                    };

                    $( 
                        cmd.value = Some($value);
                    )?

                    cmd
                }
            )*
        }
    };
}

create_command!(
    car => 1 << 0;
    forward => 1 << 0;
    backward => 1 << 1;
    rightward => 1 << 2;
    leftward => 1 << 3;
    speed => 1 << 4, value;
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_bytes() {
        let cmd1 = car::forward();
        let cmd2 = car::speed(100);

        let r_cmd = cmd1 + cmd2;

        assert_eq!(r_cmd.to_bytes(), vec![0b001_10001, 0b1100100, 0])

        /*let action1 = Motor::Speed(100);
        let action2 = Motor::Forward;
    
        let _ = action1 + action2;    

       assert_eq!(action1.as_bytes(), [0b0010_1000, 0b1100100, 0])*/
    }
}
