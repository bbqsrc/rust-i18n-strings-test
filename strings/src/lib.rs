#![feature(alloc_system)]
extern crate alloc_system;

extern crate strfmt;
#[macro_use] extern crate maplit;
#[macro_use] extern crate strings_codegen;

use strfmt::strfmt;

inject_strings!("codegen.rs");

pub struct Strings;

impl Strings {
    pub fn a_test_string() -> String {
        STRINGS.with(|s| s.with_key("aTestString"))
    }

    pub fn format_thing(thing: &str) -> String {
        let args = convert_args!(keys=String::from, hashmap!(
            "thing" => thing
        ));

        let pattern = STRINGS.with(|s| s.with_key("formatThing"));

        strfmt(&pattern, &args).unwrap()
    }
}

pub fn set_language(lang_tag: &str) {
    STRINGS.with(|s| s.set_language(lang_tag).unwrap());
}

#[cfg(test)]
mod tests {
    use ::*;

    #[test]
    fn it_works() {
        assert_eq!(Strings::a_test_string(), "This is a test string!");
        assert_eq!(Strings::format_thing("lol"), "I formatted this lol.");

        STRINGS.with(|s| s.set_language("fr").unwrap());
        
        assert_eq!(Strings::a_test_string(), "Je suis le string.");
        assert_eq!(Strings::format_thing("hue"), "Tu es un(e) hue.");
    }
}
