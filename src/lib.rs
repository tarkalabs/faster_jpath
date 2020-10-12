#[macro_use]
extern crate rutie;
use rutie::{Class, Object, Hash, RString, AnyObject};

class!(JPath);
methods!(
    JPath,
    _rtself,
    fn jp_say_rust_hello() -> Hash {
        let mut hash = Hash::new();
        hash.store(RString::new_utf8("message"), RString::new_utf8("hello world"));
        hash
    }

    fn jp_extract_from_string(pattern: RString, json: RString) -> AnyObject {
        jpath::extract_string(pattern.unwrap().to_str(), json.unwrap().to_str()).unwrap()
    }
    fn jp_extract_from_file(pattern: RString, filename: RString) -> AnyObject {
        jpath::extract_file(pattern.unwrap().to_str(), filename.unwrap().to_str()).unwrap()
    }
);

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn Init_faster_jpath() {
    Class::new("JPath", None).define(|klass| {
        let se = Class::from_existing("StandardError");
        klass.define_nested_class("JPathError", Some(&se));
        klass.def_self("say_rust_hello", jp_say_rust_hello);
        klass.def_self("extract_from_string", jp_extract_from_string);
        klass.def_self("extract_from_file", jp_extract_from_file);
    });
}

mod jpath;
mod serde_anyobject;