trait AppendBar {
    fn append_bar(self) -> Self;
}

// TODO: Implement the trait `AppendBar` for a vector of strings.
// `append_bar` should push the string "Bar" into the vector.
impl AppendBar for Vec<String> {
    fn append_bar(mut self) -> Self {
        // This is really intersting. The 'push' method modifies the
        // receiver in place, which means that we need to mutate the instance
        // of 'self'... which is why we need to decalre it as mutable.
        // 🤔 It's strange to use this syntax because we're mutating and
        // then having to return which is quit explicit. It would be great
        // if there was a more implicit method, similar to JS .map()
        self.push("Bar".to_string());
        self
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_vec_pop_eq_bar() {
        let mut foo = vec![String::from("Foo")].append_bar();
        assert_eq!(foo.pop().unwrap(), "Bar");
        assert_eq!(foo.pop().unwrap(), "Foo");
    }
}
