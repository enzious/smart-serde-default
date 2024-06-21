use proc_macro::TokenStream;
use syn::{parse_macro_input, Item};

mod smart_serde_default;

#[proc_macro_attribute]
pub fn smart_serde_default(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as Item);

    match item {
        Item::Struct(s) => smart_serde_default::expand_struct(s),
        _ => panic!("Can only be used on structs."),
    }
}
