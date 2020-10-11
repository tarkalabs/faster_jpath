#[macro_use]
extern crate rutie;

use rutie::{Class, Object, Hash, RString, VM};

class!(JPath);
methods!(
    JPath,
    _rtself,
    fn jp_say_rust_hello() -> Hash {
        let mut hash = Hash::new();
        hash.store(RString::new_utf8("message"), RString::new_utf8("hello world"));
        hash
    }
);

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn Init_faster_jpath() {
    Class::new("JPath", None).define(|klass| {
        klass.def_self("say_rust_hello", jp_say_rust_hello);
    });
}