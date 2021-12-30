#[allow(unused_mut)]

fn main() {
    println!(
        "Testing for \"Hello, World\": {}",
        first_word("Hello, world!")
    );
    // since slices are essentially references,
    // the semantics for borrowing remain the same.
    let mut owned_string = String::from("This is an owned string");
    let borrowed_string_slice = first_word(&owned_string);
    // owned_string.clear();
    println!("{}", borrowed_string_slice);
}

// function that returns the first word in a string.
fn first_word(s: &str) -> &str {
    // returns sliced first word if there is an emptyspace
    // or whole slice when there's  none.
    if let Some(index) = s.find(' ') {
        &s[..index]
    } else {
        s
    }
}
