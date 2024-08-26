fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    #[test]
    fn simple_option() {
        let target = "rustlings";
        // convert this to an Optional Type
        let optional_target = Some(target);

        // TODO: Make this an if-let statement whose value is `Some`.
        if let Some(word) = optional_target {
            // assert that the thing we converted is equal to the
            // source we converted it from
            assert_eq!(word, target);
        }
    }

    #[test]
    fn layered_option() {
        let range = 10;
        let mut optional_integers: Vec<Option<i8>> = vec![None];

        for i in 1..=range {
            optional_integers.push(Some(i));
        }

        let mut cursor = range;

        // TODO: Make this a while-let statement. Remember that `Vec::pop()`
        // adds another layer of `Option`. You can do nested pattern matching
        // in if-let and while-let statements.

        while let Some(integer) = optional_integers.pop() {
            // ℹ️  Interesting to note that we can reuse the variable name
            // to do nested destructuring in the while-let & if-let statements
            if let Some(integer) = integer {
                assert_eq!(integer, cursor);
                cursor -= 1;
            }
        }

        assert_eq!(cursor, 0);
    }
}
