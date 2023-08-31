#![doc = include_str!("../README.md")]

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;

mod attrs;
mod fields;
mod utils;

fn impl_new(ast: &syn::DeriveInput) -> proc_macro2::TokenStream {
    utils::derive_input_checks(ast);
    let struct_name = &ast.ident;
    let new_function_doc = format!(" Creates a new instance of [`{}`].", struct_name);
    let fields = fields::Fields::parse(ast);
    let names = fields.names;
    let types = fields.types;
    let values_names = fields.values;

    if fields.is_unnamed {
        quote! {
            impl #struct_name {
                #[doc = #new_function_doc]
                pub fn new(
                    #( #values_names: impl Into<#types> ),*
                ) -> Self {
                    Self(
                        #( #values_names.into() ),*
                    )
                }
            }
        }
    } else {
        quote! {
            impl #struct_name {
                #[doc = #new_function_doc]
                pub fn new(
                    #( #values_names: impl Into<#types> ),*
                ) -> Self {
                    Self {
                        #( #names: #values_names.into() ),*
                    }
                }
            }
        }
    }
}

/// Derive macro that implements a new function for a struct.
/// ## Attributes
/// - `#[impl_new(name = "name")]`: Use this attribute to change the name of the argument in the generated `new` function.
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
