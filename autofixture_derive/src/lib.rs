mod r#enum;
mod r#struct;
mod r#union;

use proc_macro::TokenStream;
use quote::quote;
use syn::Ident;

#[proc_macro_derive(AutoFixture, attributes(serde, sfs))]
pub fn derive_request(item: TokenStream) -> TokenStream {
    let u = MyUnion {
        //lol: 123.45,
        something_else: 123,
    };



    quote! {
    }.into()
}

union MyUnion {
    pub lol: f64,
    pub something_else: i128,
}