#![doc = include_str!("../README.md")]
#![allow(dead_code)] // TODO: Remove this when the crate is ready.

extern crate proc_macro;

use new_struct::NewStruct;
use proc_macro::TokenStream;
use quote::quote;

mod attrs;
mod fields;
mod new_struct;
mod utils;

/// Derive macro that implements a new function for a struct.
/// ## Attributes
/// - `#[impl_new(name = "name")]`: Use this attribute to change the name of the argument in the generated `new` function.
/// - `#[impl_new(default)]`: Use this attribute to remove the field from the generated `new` function and use the default value instead.
/// - `#[impl_new(value = || <VALUE>)]`: Use this attribute to remove the field from the generated `new` function and use the given value instead.
///
//// ## Example
/// ### For Named Fields
///
/// ```rust
/// #[derive(impl_new::New)]
/// struct Foo {
///     name: String,
///     age: usize,
/// }
///
/// // The generated code will look like this:
/// // impl Foo {
/// //     pub fn new(name: impl Into<String>, age: Into<usize>) -> Self {
/// //         Self { name: name.into(), age: age.into() }
/// //     }
/// // }
///
/// fn main() {
///     let foo = Foo::new("Hello", 42usize); // Will use `Into::into` to convert the arguments to the fields types.
///     assert_eq!(foo.name, "Hello".to_string());
///     assert_eq!(foo.age, 42);
/// }
/// ```
/// #### With Attributes
///
/// ```rust
/// #[derive(impl_new::New)]
/// struct Foo {
///     #[impl_new(name = "user_name")]
///     name: String,
///     #[impl_new(name = "user_age")]
///     age: usize,
/// }
///
/// // The generated code will look like this:
/// // impl Foo {
/// //     pub fn new(user_name: impl Into<String>, user_age: Into<usize>) -> Self {
/// //         Self { name: user_name.into(), age: user_age.into() }
/// //     }
/// // }
///
/// fn main() {
///     let foo = Foo::new("Hello", 42usize); // Will use `Into::into` to convert the arguments to the fields types.
///     assert_eq!(foo.name, "Hello".to_string());
///     assert_eq!(foo.age, 42);
/// }
/// ```
/// ### For Unnamed Fields
///
/// ```rust
/// #[derive(impl_new::New)]
/// struct Foo(#[impl_new(name = "name")] String, #[impl_new(name = "age")] usize);
///
/// // The generated code will look like this:
/// // impl Foo {
/// //     pub fn new(name: impl Into<String>, age: Into<usize>) -> Self {
/// //         Self(name.into(), age.into())
/// //     }
/// // }
///
/// fn main() {
///     let foo = Foo::new("Hello", 42usize); // Will use `Into::into` to convert the arguments to the fields types.
///     assert_eq!(foo.0, "Hello".to_string());
///     assert_eq!(foo.1, 42);
/// }
/// ```
#[proc_macro_error::proc_macro_error]
#[proc_macro_derive(New, attributes(impl_new))]
pub fn new_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    let gen = impl_new(&ast);

    gen.into()
}

/// Implements the `new` function for the given struct.
fn new_function(new_struct: NewStruct) -> proc_macro2::TokenStream {
    let new_function_doc = format!(" Creates a new [`{}`] instance.", new_struct.ident);
    let arg_names: Vec<proc_macro2::Ident> = new_struct
        .fields
        .iter()
        .filter_map(|field| field.arg_name())
        .collect();
    let types: Vec<syn::Type> = new_struct
        .fields
        .iter()
        .map(|field| field.ty.clone())
        .collect();
    let values: Vec<syn::Expr> = new_struct
        .fields
        .iter()
        .map(|field| field.value())
        .collect();

    if new_struct.is_tuple_struct {
        quote! {
            #[doc = #new_function_doc]
            pub fn new(#(#arg_names: impl Into<#types>),*) -> Self {
                Self(#(#values),*)
            }
        }
    } else {
        let names = new_struct.fields.iter().map(|field| field.field_name());
        quote!(
            #[doc = #new_function_doc]
            pub fn new(#(#arg_names: impl Into<#types>),*) -> Self {
                Self { #(#names: #values),* }
            }
        )
    }
}

fn impl_new(ast: &syn::DeriveInput) -> proc_macro2::TokenStream {
    utils::derive_input_checks(ast);
    let struct_fields = match ast.data {
        syn::Data::Struct(ref data) => data
            .fields
            .clone()
            .into_iter()
            .map(fields::ImplNewField::parse)
            .collect::<syn::Result<Vec<fields::ImplNewField>>>(),
        _ => unreachable!("The `impl_new::New` macro can only be used on structs."),
    };

    match struct_fields {
        Ok(fields) => {
            utils::new_macro_checks(&fields);
            let new_struct = NewStruct::new(ast, fields);
            let new_function = new_function(new_struct);
            let struct_name = &ast.ident;
            quote!(
                #[allow(clippy::redundant_closure)]
                #[allow(clippy::redundant_closure_call)]
                impl #struct_name {
                    #new_function
                }
            )
        }
        Err(err) => err.to_compile_error(),
    }
}
