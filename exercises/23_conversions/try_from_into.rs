// `TryFrom` is a simple and safe type conversion that may fail in a controlled
// way under some circumstances. Basically, this is the same as `From`. The main
// difference is that this should return a `Result` type instead of the target
// type itself. You can read more about it in the documentation:
// https://doc.rust-lang.org/std/convert/trait.TryFrom.html

#![allow(clippy::useless_vec)]
use std::convert::{TryFrom, TryInto};

#[derive(Debug, PartialEq)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

// We will use this error type for the `TryFrom` conversions.
#[derive(Debug, PartialEq)]
enum IntoColorError {
    // Incorrect length of slice
    BadLen,
    // Integer conversion error
    IntConversion,
}

// ℹ️  Looking at the solution file, it's clear that there's a lot
// that I need to learn about cleaning up and reusing the code. I'm
// still very much in the stage of getting things to work and haven't
// yet moved to making them better/good/faster.
//
// Looking at solution/23_conversions/try_from_into.rs, I can see that
// the solution re-uses the TryFrom for a tuple. I didn't even think about
// just passing a tuple to the original method.
//
impl TryFrom<(i16, i16, i16)> for Color {
    type Error = IntoColorError;

    fn try_from(tuple: (i16, i16, i16)) -> Result<Self, Self::Error> {
        // this is very nearly there. I can do better here....
        // let (red, green, blue) = tuple;
        //
        // Looking at this implementation -- there's a clear gap to me
        // which is that the function doesn't do any check on whether or
        // not the i16 is within range. Now... I'm going to have to dig into
        // what the values of u8 *can* be.
        //
        // Ok ... just did some digging and it turns out that I don't really
        // know my data types as well as I should. u8's max val is 255. Let's break
        // it down so that I remember:
        //
        // u8 is unsigned 8 bit value
        // A single bit can be 0 or 1
        // 8 bits of 0s / 1s in binary can have a max value of 255
        // 255 == 1 | 1 | 1 | 1 | 1 | 1 | 1 | 1
        // So when we try the conversion here, we save a massive amount
        // of looping and comparison, because we don't need to check that
        // the value is within range. The conversion checks that for us.
        let (Ok(red), Ok(green), Ok(blue)) = (
            u8::try_from(tuple.0),
            u8::try_from(tuple.1),
            u8::try_from(tuple.2),
        ) else {
            return Err(IntoColorError::IntConversion);
        };

        Ok(Color { red, green, blue })
    }
}

impl TryFrom<[i16; 3]> for Color {
    type Error = IntoColorError;

    fn try_from(arr: [i16; 3]) -> Result<Self, Self::Error> {
        // We can then reuse the defined function here. Now ... I'm not entirely
        // sure how the copmiler knows that we need to use the other method. The only
        // justification I have is that the data type we're passing in (aka the function signature)
        // matches the Tuple implementation we have. That tells me that the impl trait is somewhat
        // dynamic in that it takes in the function signature and knows which method to call
        // based on the arguments. Pretty neat!
        Self::try_from((arr[0], arr[1], arr[2]))
    }
}

impl TryFrom<&[i16]> for Color {
    type Error = IntoColorError;

    fn try_from(slice: &[i16]) -> Result<Self, Self::Error> {
        if slice.len() != 3 {
            return Err(IntoColorError::BadLen);
        };

        Self::try_from((slice[0], slice[1], slice[2]))
    }
}

fn main() {
    // Using the `try_from` function.
    let c1 = Color::try_from((183, 65, 14));
    println!("{c1:?}");

    // Since `TryFrom` is implemented for `Color`, we can use `TryInto`.
    let c2: Result<Color, _> = [183, 65, 14].try_into();
    println!("{c2:?}");

    let v = vec![183, 65, 14];
    // With slice we should use the `try_from` function
    let c3 = Color::try_from(&v[..]);
    println!("{c3:?}");
    // or put the slice within round brackets and use `try_into`.
    let c4: Result<Color, _> = (&v[..]).try_into();
    println!("{c4:?}");
}

#[cfg(test)]
mod tests {
    use super::*;
    use IntoColorError::*;

    #[test]
    fn test_tuple_out_of_range_positive() {
        assert_eq!(Color::try_from((256, 1000, 10000)), Err(IntConversion));
    }

    #[test]
    fn test_tuple_out_of_range_negative() {
        assert_eq!(Color::try_from((-1, -10, -256)), Err(IntConversion));
    }

    #[test]
    fn test_tuple_sum() {
        assert_eq!(Color::try_from((-1, 255, 255)), Err(IntConversion));
    }

    #[test]
    fn test_tuple_correct() {
        let c: Result<Color, _> = (183, 65, 14).try_into();
        assert!(c.is_ok());
        assert_eq!(
            c.unwrap(),
            Color {
                red: 183,
                green: 65,
                blue: 14,
            }
        );
    }

    #[test]
    fn test_array_out_of_range_positive() {
        let c: Result<Color, _> = [1000, 10000, 256].try_into();
        assert_eq!(c, Err(IntConversion));
    }

    #[test]
    fn test_array_out_of_range_negative() {
        let c: Result<Color, _> = [-10, -256, -1].try_into();
        assert_eq!(c, Err(IntConversion));
    }

    #[test]
    fn test_array_sum() {
        let c: Result<Color, _> = [-1, 255, 255].try_into();
        assert_eq!(c, Err(IntConversion));
    }

    #[test]
    fn test_array_correct() {
        let c: Result<Color, _> = [183, 65, 14].try_into();
        assert!(c.is_ok());
        assert_eq!(
            c.unwrap(),
            Color {
                red: 183,
                green: 65,
                blue: 14
            }
        );
    }

    #[test]
    fn test_slice_out_of_range_positive() {
        let arr = [10000, 256, 1000];
        assert_eq!(Color::try_from(&arr[..]), Err(IntConversion));
    }

    #[test]
    fn test_slice_out_of_range_negative() {
        let arr = [-256, -1, -10];
        assert_eq!(Color::try_from(&arr[..]), Err(IntConversion));
    }

    #[test]
    fn test_slice_sum() {
        let arr = [-1, 255, 255];
        assert_eq!(Color::try_from(&arr[..]), Err(IntConversion));
    }

    #[test]
    fn test_slice_correct() {
        let v = vec![183, 65, 14];
        let c: Result<Color, _> = Color::try_from(&v[..]);
        assert!(c.is_ok());
        assert_eq!(
            c.unwrap(),
            Color {
                red: 183,
                green: 65,
                blue: 14,
            }
        );
    }

    #[test]
    fn test_slice_excess_length() {
        let v = vec![0, 0, 0, 0];
        assert_eq!(Color::try_from(&v[..]), Err(BadLen));
    }

    #[test]
    fn test_slice_insufficient_length() {
        let v = vec![0, 0];
        assert_eq!(Color::try_from(&v[..]), Err(BadLen));
    }
}
