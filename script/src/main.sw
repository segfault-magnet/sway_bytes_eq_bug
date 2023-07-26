script;

use std::string::String;
use std::assert::assert_eq;

fn main() {
    let string_a = String::from_ascii_str("Hello World");
    assert_eq(string_a.as_bytes().len(), 11);

    let string_b = String::from_ascii_str("NoNoNo");
    assert_eq(string_b.as_bytes().len(), 6);

    // Should not pass but it does
    assert_eq(string_a.as_bytes(), string_b.as_bytes());
}
