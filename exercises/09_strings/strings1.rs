// TODO: Fix the compiler error without changing the function signature.
fn current_favorite_color() -> String {
    // String::from("blue")
    "blue".to_string()
}

fn main() {
    let answer = current_favorite_color();
    println!("My current favorite color is {answer}");
}
