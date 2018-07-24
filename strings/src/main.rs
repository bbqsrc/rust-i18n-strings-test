extern crate strings;

fn main() {
    println!("en: {}", strings::Strings::a_test_string());
    strings::set_language("fr");
    println!("fr: {}", strings::Strings::a_test_string());
}