use proc_macro2::TokenStream;
use quote::quote;
use syn::{DataStruct, Fields, Generics, Ident, Type};

/// Field types recognised as having a well-defined "empty" value:
/// `rs_autofixture::fixture::builder::EmptyFixture`,
/// matched by their last path segment.
///
/// Only fields of these types get a `without_<field>`
/// setter generated on the derived builder.
const EMPTY_FIXTURE_TYPES: &[&str] = &[
    "Option",
    "String",
    "Vec",
    "VecDeque",
    "LinkedList",
    "HashSet",
    "BTreeSet",
    "BinaryHeap",
    "HashMap",
    "BTreeMap",
];

fn is_empty_fixture_type(ty: &Type) -> bool {
    let Type::Path(type_path) = ty else {
        return false;
    };

    type_path
        .path
        .segments
        .last()
        .is_some_and(|segment| {
            EMPTY_FIXTURE_TYPES.contains(
                &segment
                    .ident
                    .to_string()
                    .as_str(),
            )
        })
}

pub fn expand(
    name: &Ident,
    generics: &Generics,
    data: &DataStruct,
    can_freeze: bool,
) -> TokenStream {
    let create_body = struct_create_body(&data.fields);
    let builder_name = quote::format_ident!("{name}Builder");
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let builder_field_declarations = builder_field_declarations(&data.fields);
    let builder_field_inits = builder_field_inits(&data.fields);
    let with_methods = builder_with_methods(&data.fields);
    let without_methods = builder_without_methods(&data.fields);
    let builder_create_body = builder_create_body(name, &data.fields);

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

                #frozen_check

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
            #(#without_methods)*
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
        }
        Fields::Unnamed(unnamed) => {
            let field_inits = unnamed
                .unnamed
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
        }
        Fields::Unit => {
            quote! { Self }
        }
    }
}

fn unnamed_field_ident(i: usize) -> Ident {
    quote::format_ident!("field_{i}")
}

/// The `FieldOverride<T>` fields added to the generated builder,
/// one per struct field.
/// Used to stash a fixed value or "empty" override supplied
/// via a `with_*`/`without_*` setter.
fn builder_field_declarations(fields: &Fields) -> Vec<TokenStream> {
    match fields {
        Fields::Named(named) => named
            .named
            .iter()
            .map(|f| {
                let field_name = f
                    .ident
                    .as_ref()
                    .unwrap();
                let ty = &f.ty;

                quote! { #field_name: rs_autofixture::fixture::builder::FieldOverride<#ty> }
            })
            .collect(),
        Fields::Unnamed(unnamed) => unnamed
            .unnamed
            .iter()
            .enumerate()
            .map(|(i, f)| {
                let field_name = unnamed_field_ident(i);
                let ty = &f.ty;

                quote! { #field_name: rs_autofixture::fixture::builder::FieldOverride<#ty> }
            })
            .collect(),
        Fields::Unit => Vec::new(),
    }
}

/// Initialises every stashed field override to `NotSet` in `Builder::new`.
fn builder_field_inits(fields: &Fields) -> Vec<TokenStream> {
    match fields {
        Fields::Named(named) => named
            .named
            .iter()
            .map(|f| {
                let field_name = f
                    .ident
                    .as_ref()
                    .unwrap();

                quote! { #field_name: rs_autofixture::fixture::builder::FieldOverride::NotSet }
            })
            .collect(),
        Fields::Unnamed(unnamed) => (0..unnamed
            .unnamed
            .len())
            .map(|i| {
                let field_name = unnamed_field_ident(i);

                quote! { #field_name: rs_autofixture::fixture::builder::FieldOverride::NotSet }
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
                        self.#field_name = rs_autofixture::fixture::builder::FieldOverride::SetWith(value);

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
                        self.#field_name = rs_autofixture::fixture::builder::FieldOverride::SetWith(value);

                        self
                    }
                }
            })
            .collect(),
        Fields::Unit => Vec::new(),
    }
}

/// `without_<field>` setters
/// (`without_0`, `without_1`, ... for tuple structs)
/// that force a field to its "empty" value, e.g:
/// `None`, `""`, or an empty collection.
///
/// Only generated for field types recognised by `is_empty_fixture_type`.
fn builder_without_methods(fields: &Fields) -> Vec<TokenStream> {
    match fields {
        Fields::Named(named) => named.named
            .iter()
            .filter(|f| is_empty_fixture_type(&f.ty))
            .map(|f| {
                let field_name = f.ident.as_ref().unwrap();
                let method_name = quote::format_ident!("without_{field_name}");

                quote! {
                    pub fn #method_name(&mut self) -> &mut Self {
                        self.#field_name = rs_autofixture::fixture::builder::FieldOverride::SetWithout;

                        self
                    }
                }
            })
            .collect(),
        Fields::Unnamed(unnamed) => unnamed.unnamed
            .iter()
            .enumerate()
            .filter(|(_, f)| is_empty_fixture_type(&f.ty))
            .map(|(i, _)| {
                let field_name = unnamed_field_ident(i);
                let method_name = quote::format_ident!("without_{i}");

                quote! {
                    pub fn #method_name(&mut self) -> &mut Self {
                        self.#field_name = rs_autofixture::fixture::builder::FieldOverride::SetWithout;

                        self
                    }
                }
            })
            .collect(),
        Fields::Unit => Vec::new(),
    }
}

/// Builds the final struct instance in `FixtureBuilder::create`, preferring
/// any value stashed by a `with_*`/`without_*` setter and falling back to
/// `AutoFixture::create` otherwise.
fn builder_create_body(name: &Ident, fields: &Fields) -> TokenStream {
    match fields {
        Fields::Named(named) => {
            let field_inits = named.named.iter().map(|f| {
                let field_name = f.ident.as_ref().unwrap();
                let ty = &f.ty;
                let without_arm = builder_without_arm(field_name, ty);

                quote! {
                    #field_name: match std::mem::take(&mut self.#field_name) {
                        rs_autofixture::fixture::builder::FieldOverride::SetWith(value) => value,
                        rs_autofixture::fixture::builder::FieldOverride::SetWithout => #without_arm,
                        rs_autofixture::fixture::builder::FieldOverride::NotSet
                            => <#ty as AutoFixture>::create(self.fixture),
                    }
                }
            });

            quote! {
                #name {
                    #(#field_inits),*
                }
            }
        }
        Fields::Unnamed(u) => {
            let field_inits = u.unnamed.iter().enumerate().map(|(i, f)| {
                let field_name = unnamed_field_ident(i);
                let ty = &f.ty;
                let without_arm = builder_without_arm(&field_name, ty);

                quote! {
                    match std::mem::take(&mut self.#field_name) {
                        rs_autofixture::fixture::builder::FieldOverride::SetWith(value) => value,
                        rs_autofixture::fixture::builder::FieldOverride::SetWithout => #without_arm,
                        rs_autofixture::fixture::builder::FieldOverride::NotSet
                            => <#ty as AutoFixture>::create(self.fixture),
                    }
                }
            });

            quote! {
                #name(#(#field_inits),*)
            }
        }
        Fields::Unit => {
            quote! { #name }
        }
    }
}

/// The expression used for a field's `SetWithout` match arm.
///
/// Only field types recognised by `is_empty_fixture_type` can actually reach this
/// state (that's the only way `FieldOverride::SetWithout` gets
/// constructed...),
/// so anything else is genuinely unreachable and would
/// otherwise require an `EmptyFixture` bound that most field types can't satisfy.
fn builder_without_arm(field_name: &Ident, ty: &Type) -> TokenStream {
    if is_empty_fixture_type(ty) {
        quote! {
            <#ty as rs_autofixture::fixture::builder::EmptyFixture>::empty()
        }
    } else {
        let message = format!("without_{field_name} is not supported for this field type");

        quote! { unreachable!(#message) }
    }
}
