mod r#enum;
mod r#struct;
mod r#union;

use proc_macro::TokenStream;
use syn::{Attribute, Data, DeriveInput, parse_macro_input};

#[proc_macro_derive(AutoFixture, attributes(fixture))]
pub fn derive_request(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let name = &input.ident;
    let generics = &input.generics;
    let can_freeze = can_freeze(&input.attrs);

    let expanded = match &input.data {
        Data::Struct(data) => r#struct::expand(name, generics, data, can_freeze),
        Data::Enum(data) => r#enum::expand(name, generics, data, can_freeze),
        Data::Union(data) => r#union::expand(name, generics, data, can_freeze),
    };

    expanded.into()
}

/// Whether `#[fixture(can_freeze)]` is present
fn can_freeze(attrs: &[Attribute]) -> bool {
    attrs
        .iter()
        .filter(|attr| attr.path().is_ident("fixture"))
        .any(|attr| {
            let mut found = false;

            let _ = attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("can_freeze") {
                    found = true;
                }

                Ok(())
            });

            found
        })
}
