// The Rust compiler needs to know how to check whether supplied references are
// valid, so that it can let the programmer know if a reference is at risk of
// going out of scope before it is used. Remember, references are borrows and do
// not own their own data. What if their owner goes out of scope?

// ℹ️  I'm still not 100% sure what the lifetime definition is doing here - but
// from what I gather, I think it's something to help the compiler understand
// which REFERENCE (aka: "&" value) the function should keep around. We need to
// annotate the function with a 'generic-like' argument that is a lifetime.
// This lifetime will help the compiler understand that we've got some parameters
// coming in that must be around as long as the function is - because if not
// the program will invalidate that memory address, meaning that the variable
// is no longer available.

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest() {
        assert_eq!(longest("abcd", "123"), "abcd");
        assert_eq!(longest("abc", "1234"), "1234");
    }
}
