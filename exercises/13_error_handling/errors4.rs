#![allow(clippy::comparison_chain)]

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<Self, CreationError> {
        // ℹ️  The reason I've had to use an explict return here is
        // because Rust expects me to return an express (just like a JS ternary),
        // but I've added multiple statements - solutions/13_error_handling/errors4.rs
        // uses an 'else if' so that the expression is continued
        if value < 0 {
            return Err(CreationError::Negative);
        }
        if value == 0 {
            return Err(CreationError::Zero);
        }
        Ok(Self(value as u64))
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        assert_eq!(
            PositiveNonzeroInteger::new(10),
            Ok(PositiveNonzeroInteger(10)),
        );
        assert_eq!(
            PositiveNonzeroInteger::new(-10),
            Err(CreationError::Negative),
        );
        assert_eq!(PositiveNonzeroInteger::new(0), Err(CreationError::Zero));
    }
}
