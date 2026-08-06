use proc_macro2::TokenStream;
use quote::quote;
use syn::{DataEnum, Fields, Generics, Ident};

pub fn expand(name: &Ident, generics: &Generics, data: &DataEnum, can_freeze: bool) -> TokenStream {
    let variant_count = data.variants.len();

    let builder_name = quote::format_ident!("{name}Builder");
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let variant_arms = data.variants.iter().enumerate().map(|(i, v)| {
        let variant_name = &v.ident;
        let body = variant_create_body(name, variant_name, &v.fields);

        quote! { #i => #body }
    });

    // Only emitted for `#[fixture(can_freeze)]` items, which must also
    // derive `Clone` themselves.
    //
    // Returning a frozen value means cloning it back out of the frozen pool.
    let frozen_check = can_freeze.then(|| {
        quote! {
            if let Some(frozen) = f.frozen::<Self>() {
                return frozen;
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

                #frozen_check

                let variant: usize = f.rng().random_range(0..#variant_count);

                match variant {
                    #(#variant_arms,)*
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

fn variant_create_body(enum_name: &Ident, variant_name: &Ident, fields: &Fields) -> TokenStream {
    match fields {
        Fields::Named(named) => {
            let field_inits = named.named.iter().map(|f| {
                let field_name = f.ident.as_ref().unwrap();
                let ty = &f.ty;

                quote! {
                    #field_name: <#ty as rs_autofixture::fixture::auto_fixture::AutoFixture>::create(f)
                }
            });

            quote! {
                #enum_name::#variant_name { #(#field_inits),* }
            }
        }
        Fields::Unnamed(unnamed) => {
            let field_inits = unnamed.unnamed.iter().map(|f| {
                let ty = &f.ty;

                quote! {
                    <#ty as rs_autofixture::fixture::auto_fixture::AutoFixture>::create(f)
                }
            });

            quote! {
                #enum_name::#variant_name(#(#field_inits),*)
            }
        }
        Fields::Unit => {
            quote! { #enum_name::#variant_name }
        }
    }
}
