use proc_macro2::TokenStream;
use quote::quote;
use syn::{DataStruct, Fields, Ident, Generics};

pub fn expand(name: &Ident, generics: &Generics, data: &DataStruct) -> TokenStream {
    let create_body = struct_create_body(&data.fields);
    let builder_name = quote::format_ident!("{name}Builder");
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let builder_field_declarations = builder_field_declarations(&data.fields);
    let builder_field_inits = builder_field_inits(&data.fields);
    let with_methods = builder_with_methods(&data.fields);
    let builder_create_body = builder_create_body(name, &data.fields);

    quote! {
        impl #impl_generics rs_autofixture::fixture::auto_fixture::AutoFixture for #name
            #ty_generics
            #where_clause
        {
            type Builder<'_afb> = #builder_name<'_afb>;

            fn create(f: &mut rs_autofixture::fixture::Fixture) -> Self {
                use rs_autofixture::fixture::auto_fixture::AutoFixture;

                #create_body
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
            #(#builder_field_declarations),*
        }

        impl<'_afb> #builder_name<'_afb> {
            #(#with_methods)*
        }

        impl<'_afb> rs_autofixture::fixture::builder::FixtureBuilder<'_afb> for #builder_name<'_afb> {
            type F = #name;

            fn new(f: &'_afb mut rs_autofixture::fixture::Fixture) -> Self {
                Self {
                    fixture: f,
                    #(#builder_field_inits),*
                }
            }

            fn create(&mut self) -> Self::F {
                use rs_autofixture::fixture::auto_fixture::AutoFixture;

                #builder_create_body
            }
        }
    }
}

fn struct_create_body(fields: &Fields) -> TokenStream {
    match fields {
        Fields::Named(named) => {
            let field_inits = named.named
                .iter()
                .map(|f| {
                    let field_name = f.ident.as_ref().unwrap();
                    let ty = &f.ty;

                    quote! {
                        #field_name: <#ty as rs_autofixture::fixture::auto_fixture::AutoFixture>::create(f)
                    }
                });

            quote! {
                Self {
                    #(#field_inits),*
                }
            }
        },
        Fields::Unnamed(unnamed) => {
            let field_inits = unnamed.unnamed
                .iter()
                .map(|f| {
                    let ty = &f.ty;

                    quote! {
                        <#ty as rs_autofixture::fixture::auto_fixture::AutoFixture>::create(f)
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

fn unnamed_field_ident(i: usize) -> Ident {
    quote::format_ident!("field_{i}")
}

/// The `Option<T>` fields added to the generated builder, one per struct
/// field, used to stash a fixed value supplied via a `with_*` setter.
fn builder_field_declarations(fields: &Fields) -> Vec<TokenStream> {
    match fields {
        Fields::Named(named) => named.named
            .iter()
            .map(|f| {
                let field_name = f.ident.as_ref().unwrap();
                let ty = &f.ty;

                quote! { #field_name: Option<#ty> }
            })
            .collect(),
        Fields::Unnamed(unnamed) => unnamed.unnamed
            .iter()
            .enumerate()
            .map(|(i, f)| {
                let field_name = unnamed_field_ident(i);
                let ty = &f.ty;

                quote! { #field_name: Option<#ty> }
            })
            .collect(),
        Fields::Unit => Vec::new(),
    }
}

/// Initialises every stashed field `Option` to `None` in `Builder::new`.
fn builder_field_inits(fields: &Fields) -> Vec<TokenStream> {
    match fields {
        Fields::Named(named) => named.named
            .iter()
            .map(|f| {
                let field_name = f.ident.as_ref().unwrap();

                quote! { #field_name: None }
            })
            .collect(),
        Fields::Unnamed(unnamed) => (0..unnamed.unnamed.len())
            .map(|i| {
                let field_name = unnamed_field_ident(i);

                quote! { #field_name: None }
            })
            .collect(),
        Fields::Unit => Vec::new(),
    }
}

/// `with_<field>` setters (`with_0`, `with_1`, ... for tuple structs) that
/// fix a field to a specific value instead of a randomly generated one.
fn builder_with_methods(fields: &Fields) -> Vec<TokenStream> {
    match fields {
        Fields::Named(named) => named.named
            .iter()
            .map(|f| {
                let field_name = f.ident.as_ref().unwrap();
                let ty = &f.ty;
                let method_name = quote::format_ident!("with_{field_name}");

                quote! {
                    pub fn #method_name(&mut self, value: #ty) -> &mut Self {
                        self.#field_name = Some(value);

                        self
                    }
                }
            })
            .collect(),
        Fields::Unnamed(unnamed) => unnamed.unnamed
            .iter()
            .enumerate()
            .map(|(i, f)| {
                let field_name = unnamed_field_ident(i);
                let ty = &f.ty;
                let method_name = quote::format_ident!("with_{i}");

                quote! {
                    pub fn #method_name(&mut self, value: #ty) -> &mut Self {
                        self.#field_name = Some(value);

                        self
                    }
                }
            })
            .collect(),
        Fields::Unit => Vec::new(),
    }
}

/// Builds the final struct instance in `FixtureBuilder::create`, preferring
/// any value stashed by a `with_*` setter and falling back to
/// `AutoFixture::create` otherwise.
fn builder_create_body(name: &Ident, fields: &Fields) -> TokenStream {
    match fields {
        Fields::Named(named) => {
            let field_inits = named.named
                .iter()
                .map(|f| {
                    let field_name = f.ident.as_ref().unwrap();
                    let ty = &f.ty;

                    quote! {
                        #field_name: match self.#field_name.take() {
                            Some(value) => value,
                            None => <#ty as AutoFixture>::create(self.fixture),
                        }
                    }
                });

            quote! {
                #name {
                    #(#field_inits),*
                }
            }
        },
        Fields::Unnamed(u) => {
            let field_inits = u.unnamed
                .iter()
                .enumerate()
                .map(|(i, f)| {
                    let field_name = unnamed_field_ident(i);
                    let ty = &f.ty;

                    quote! {
                        match self.#field_name.take() {
                            Some(value) => value,
                            None => <#ty as AutoFixture>::create(self.fixture),
                        }
                    }
                });

            quote! {
                #name(#(#field_inits),*)
            }
        },
        Fields::Unit => {
            quote! { #name }
        },
    }
}

