use proc_macro2::TokenStream;
use quote::quote;
use syn::{DataUnion, Generics, Ident};

pub fn expand(name: &Ident, generics: &Generics, data: &DataUnion) -> TokenStream {
    let field_count = data
        .fields
        .named
        .len();
    let builder_name = quote::format_ident!("{name}Builder");
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let field_arms = data.fields.named.iter().enumerate().map(|(i, f)| {
        let field_name = f.ident.as_ref().unwrap();
        let ty = &f.ty;

        quote! {
            #i => Self {
                #field_name: <#ty as rs_autofixture::fixture::auto_fixture::AutoFixture>::create(f),
            }
        }
    });

    quote! {
        impl #impl_generics rs_autofixture::fixture::auto_fixture::AutoFixture for #name
            #ty_generics
            #where_clause
        {
            type Builder<'_afb> = #builder_name<'_afb>;

            fn create(f: &mut rs_autofixture::fixture::Fixture) -> Self {
                use rs_autofixture::fixture::auto_fixture::AutoFixture;
                use rs_autofixture::fixture::FixtureExt;
                use rs_autofixture::rand::RngExt;

                let field: usize = f.rng().random_range(0..#field_count);

                match field {
                    #(#field_arms,)*
                    _ => unreachable!(),
                }
            }

            fn build<'_afb>(f: &'_afb mut rs_autofixture::fixture::Fixture)
                -> Self::Builder<'_afb>
            {
                use rs_autofixture::fixture::builder::FixtureBuilder;

                #builder_name::new(f)
            }
        }

        pub struct #builder_name<'_afb> {
            fixture: &'_afb mut rs_autofixture::fixture::Fixture,
        }

        impl<'_afb> rs_autofixture::fixture::builder::FixtureBuilder<'_afb> for #builder_name<'_afb> {
            type F = #name;

            fn new(f: &'_afb mut rs_autofixture::fixture::Fixture) -> Self {
                Self { fixture: f }
            }

            fn create(&mut self) -> Self::F {
                use rs_autofixture::fixture::auto_fixture::AutoFixture;

                <Self::F as AutoFixture>::create(self.fixture)
            }
        }
    }
}
