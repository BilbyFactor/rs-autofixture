mod r#enum;
mod r#struct;
mod r#union;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput, Data};

#[proc_macro_derive(AutoFixture, attributes(serde, sfs))]
pub fn derive_request(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let name = &input.ident;
    let generics = &input.generics;

    let expanded = match &input.data {
        Data::Struct(data) => r#struct::expand(name, generics, data),
        Data::Enum(data) => r#enum::expand(name, generics, data),
        Data::Union(data) => r#union::expand(name, generics, data),
    };

    expanded.into()
}
