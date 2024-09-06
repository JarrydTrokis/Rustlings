// In this exercise, you'll learn some of the unique advantages that iterators
// can offer.

// "hello" -> "Hello"
fn capitalize_first(input: &str) -> String {
    let mut chars = input.chars();
    match chars.next() {
        None => String::new(),
        // ℹ️  This is the solution and it's a really nice littel convenience
        // function, because it uses the same slice from the iterator:
        // ✅ Some(first) => first.to_uppercase().to_string() + chars.as_str(),
        Some(first) => first.to_uppercase().to_string() + &input[1..],
    }
}

fn capitalize_words_vector(words: &[&str]) -> Vec<String> {
    words.iter().map(|w| capitalize_first(w)).collect()
}

fn capitalize_words_string(words: &[&str]) -> String {
    words.iter().map(|word| capitalize_first(word)).collect()
    // ℹ️  I wrote a whole bunch of explicit code here when I didn't need to
    // let mut result: String = String::new();
    // for word in words {
    //     let w = *word;
    //     let c = capitalize_first(&w);
    //     result += &c;
    // }
    // result
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(capitalize_first("hello"), "Hello");
    }

    #[test]
    fn test_empty() {
        assert_eq!(capitalize_first(""), "");
    }

    #[test]
    fn test_iterate_string_vec() {
        let words = vec!["hello", "world"];
        assert_eq!(capitalize_words_vector(&words), ["Hello", "World"]);
    }

    #[test]
    fn test_iterate_into_string() {
        let words = vec!["hello", " ", "world"];
        assert_eq!(capitalize_words_string(&words), "Hello World");
    }
}
