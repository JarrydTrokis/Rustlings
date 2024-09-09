use std::i64;

#[derive(Debug, PartialEq, Eq)]
enum DivisionError {
    // Example: 42 / 0
    DivideByZero,
    // Only case for `i64`: `i64::MIN / -1` because the result is `i64::MAX + 1`
    IntegerOverflow,
    // Example: 5 / 2 = 2.5
    NotDivisible,
}

// TODO: Calculate `a` divided by `b` if `a` is evenly divisible by `b`.
// Otherwise, return a suitable error.
fn divide(a: i64, b: i64) -> Result<i64, DivisionError> {
    if b == 0 {
        return Err(DivisionError::DivideByZero);
    }
    if a == i64::MIN && b == -1 {
        return Err(DivisionError::IntegerOverflow);
    }

    if a % b != 0 {
        return Err(DivisionError::NotDivisible);
    }

    Ok(a / b)
}

// Desired output: `Ok([1, 11, 1426, 3])`
fn result_with_list() -> Result<Vec<i64>, DivisionError> {
    let numbers = [27, 297, 38502, 81];
    let division_results = numbers.into_iter().map(|n| divide(n, 27));
    // let mut res: Vec<i64> = vec![];
    //
    // for val in division_results {
    //     if let Ok(v) = val {
    //         res.push(v)
    //     }
    // }
    // Ok(res)
    division_results.collect()
}

// Desired output: `[Ok(1), Ok(11), Ok(1426), Ok(3)]`
fn list_of_results() -> Vec<Result<i64, DivisionError>> {
    let numbers = [27, 297, 38502, 81];
    let division_results = numbers.into_iter().map(|n| divide(n, 27));
    // ℹ️  My solution worked but it was *very* basic.
    // Looking at the solution, the .collect() function appears to have
    // this sort of behaviour built-in.
    //
    // let mut res: Vec<Result<i64, ()>> = vec![];
    //
    // for val in division_results {
    //     match val {
    //         Ok(v) => res.push(Ok(v)),
    //         _ => (),
    //     }
    // }
    // res
    division_results.collect()
}

// 📝 This is important when working with Iterators!
// Transforms an iterator into a collection.
//
// `collect()` can take anything iterable, and turn it into a relevant
// collection. This is one of the more powerful methods in the standard
// library, used in a variety of contexts.
//
// The most basic pattern in which `collect()` is used is to turn one
// collection into another. You take a collection, call [`iter`] on it,
// do a bunch of transformations, and then `collect()` at the end.
//
// `collect()` can also create instances of types that are not typical
// collections. For example, a [`String`](https://doc.rust-lang.org/stable/core/iter/std/string/struct.String.html) can be built from [`char`]s,
// and an iterator of [`Result<T, E>`](https://doc.rust-lang.org/stable/core/result/enum.Result.html) items can be collected
// into `Result<Collection<T>, E>`. See the examples below for more.
//
// Because `collect()` is so general, it can cause problems with type
// inference. As such, `collect()` is one of the few times you'll see
// the syntax affectionately known as the 'turbofish': `::<>`. This
// helps the inference algorithm understand specifically which collection
// you're trying to collect into.

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(divide(81, 9), Ok(9));
    }

    #[test]
    fn test_divide_by_0() {
        assert_eq!(divide(81, 0), Err(DivisionError::DivideByZero));
    }

    #[test]
    fn test_integer_overflow() {
        assert_eq!(divide(i64::MIN, -1), Err(DivisionError::IntegerOverflow));
    }

    #[test]
    fn test_not_divisible() {
        assert_eq!(divide(81, 6), Err(DivisionError::NotDivisible));
    }

    #[test]
    fn test_divide_0_by_something() {
        assert_eq!(divide(0, 81), Ok(0));
    }

    #[test]
    fn test_result_with_list() {
        assert_eq!(result_with_list().unwrap(), [1, 11, 1426, 3]);
    }

    #[test]
    fn test_list_of_results() {
        assert_eq!(list_of_results(), [Ok(1), Ok(11), Ok(1426), Ok(3)]);
    }
}
