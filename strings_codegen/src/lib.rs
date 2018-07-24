pub extern crate phf;
extern crate phf_codegen;
extern crate xz2;
extern crate serde_json;

use std::env;
use std::fs::File;
use std::fs;
use std::io::{BufWriter, Write};
use std::path::Path;
use std::io::prelude::*;
use std::collections::HashMap;
use xz2::read::{XzEncoder, XzDecoder};
use std::cell::RefCell;

pub fn generate(out_name: &str) {
    let path = Path::new(&env::var("OUT_DIR").unwrap()).join(out_name);
    let mut file = BufWriter::new(File::create(&path).unwrap());

    writeln!(&mut file, "use strings_codegen::phf;").unwrap();
    write!(&mut file, "static BUNDLES: phf::Map<&'static str, &'static [u8]> = ").unwrap();
    let res_i18n_path = Path::new(&env::var("CARGO_MANIFEST_DIR").unwrap()).join("res").join("i18n");

    let mut map = phf_codegen::Map::new();
    
    for entry in fs::read_dir(res_i18n_path).unwrap() {
        let entry = entry.unwrap();
        let filename = entry.file_name().into_string().unwrap();
        
        if !filename.ends_with(".json") {
            continue;
        }

        let name = filename.split(".").next().unwrap().to_owned();
        let file = File::open(entry.path()).unwrap();

        let mut contents = vec![];
        let mut compressor = XzEncoder::new(file, 9);
        compressor.read_to_end(&mut contents).unwrap();

        map.entry(name, &format!("&{:?}", contents));
    }

    map.build(&mut file).unwrap();
    
    write!(&mut file, ";\n").unwrap();
}

#[derive(Debug)]
pub enum LoadLanguageError {
    NoLanguageFound,
    ParseError
}

pub struct StringBundle(RefCell<HashMap<String, String>>, &'static phf::Map<&'static str, &'static [u8]>);

impl StringBundle {
    pub fn new(bundles: &'static phf::Map<&'static str, &'static [u8]>) -> StringBundle {
        StringBundle(
            RefCell::new(load_language(&bundles, "en").expect("en language always exists")),
            bundles
        )
    }

    pub fn set_language(&self, lang_tag: &str) -> Result<(), LoadLanguageError>{
        *self.0.borrow_mut() = load_language(self.1, lang_tag)?;
        Ok(())
    }

    pub fn with_key(&self, key: &str) -> String {
        self.0.borrow()[key].clone()
    }
}

pub fn load_language(bundles: &phf::Map<&'static str, &[u8]>, lang_tag: &str) -> Result<HashMap<String, String>, LoadLanguageError> {
    let lang = match bundles.get(lang_tag) {
        Some(v) => v,
        None => return Err(LoadLanguageError::NoLanguageFound)
    };

    let mut decoder = XzDecoder::new(*lang);
    let mut contents = String::new();
    decoder.read_to_string(&mut contents).map_err(|_| LoadLanguageError::ParseError)?;
    let strings: HashMap<String, String> = serde_json::from_str(&contents).unwrap();
    Ok(strings)
}

#[macro_export]
macro_rules! inject_strings {
    (
        $codegenfn:tt
    ) => {
        use $crate::{load_language, StringBundle};
        include!(concat!(env!("OUT_DIR"), "/", $codegenfn));

        thread_local! {
            static STRINGS: StringBundle = StringBundle::new(&BUNDLES);
        }
    };
}
