use std::borrow::Cow;
#[allow(unused_imports)]
use std::fs;

pub fn path() -> Cow<'static, str> {
    #[cfg(not(debug_assertions))]
    let path = env!("ENTRY_FILE_PATH");
    #[cfg(debug_assertions)]
    let path = fs::read_to_string("entry").unwrap();
    path.into()
}
