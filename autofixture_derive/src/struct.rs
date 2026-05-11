use proc_macro2::TokenStream;
use quote::quote;
use syn::{DataStruct, Fields, Ident, Generics};

pub fn expand(name: &Ident, generics: &Generics, data: &DataStruct) -> TokenStream {
    let create_body = struct_create_body(&data.fields);
    let builder_name = quote::format_ident!("{}Builder", name);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    quote! {
        impl #impl_generics autofixture::fixture::auto_fixture::AutoFixture for #name
            #ty_generics
            #where_clause
        {
            type Builder<'_afb> = #builder_name<'_afb>;

            fn create(f: &mut autofixture::fixture::Fixture) -> Self {
                use autofixture::fixture::auto_fixture::AutoFixture;

                #create_body
            }

            fn build<'_afb>(f: &'_afb mut autofixture::fixture::Fixture)
                -> Self::Builder<'_afb>
            {
                use autofixture::fixture::builder::FixtureBuilder;

                #builder_name::new(f)
            }
        }

        pub struct #builder_name<'_afb> {
            fixture: &'_afb mut autofixture::fixture::Fixture,
        }

        impl<'_afb> autofixture::fixture::builder::FixtureBuilder<'_afb> for #builder_name<'_afb> {
            type F = #name;

            fn new(f: &'_afb mut autofixture::fixture::Fixture) -> Self {
                Self { fixture: f }
            }

            fn create(&mut self) -> Self::F {
                use autofixture::fixture::auto_fixture::AutoFixture;

                <Self::F as AutoFixture>::create(self.fixture)
            }
        }
    }
}

fn struct_create_body(fields: &Fields) -> TokenStream {
    match fields {
        Fields::Named(named) => {
            let field_inits = named.named.iter().map(|f| {
                let field_name = f.ident.as_ref().unwrap();
                let ty = &f.ty;

                quote! {
                    #field_name: <#ty as autofixture::fixture::auto_fixture::AutoFixture>::create(f)
                }
            });

            quote! {
                Self {
                    #(#field_inits),*
                }
            }
        },
        Fields::Unnamed(unnamed) => {
            let field_inits = unnamed.unnamed.iter().map(|f| {
                let ty = &f.ty;

                quote! {
                    <#ty as autofixture::fixture::auto_fixture::AutoFixture>::create(f)
                }
            });

            quote! {
                Self(#(#field_inits),*)
            }
        },
        Fields::Unit => {
            quote! { Self }
        },
    }
}
