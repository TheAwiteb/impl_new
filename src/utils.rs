use proc_macro2::Span;
use proc_macro_error::{abort, Diagnostic, Level};
use syn::spanned::Spanned;

use crate::{attrs::ImplNewAttr, fields::ImplNewField};

/// Returns the span of the invalid derive input, if its not a struct.
pub(crate) fn is_struct(ast: &syn::DeriveInput) -> bool {
    matches!(ast.data, syn::Data::Struct(_))
}

pub(crate) fn is_tuple_struct(ast: &syn::DeriveInput) -> bool {
    matches!(
        ast.data,
        syn::Data::Struct(syn::DataStruct {
            fields: syn::Fields::Unnamed(_),
            ..
        })
    )
}

/// Checks if the attribute exists, and returns the span of the attribute.
pub(crate) fn is_path_exist(path: &str, attrs: &[syn::Attribute]) -> Option<Span> {
    for attr in attrs {
        if let syn::Meta::List(ref meta_list) = attr.meta {
            if meta_list.path.is_ident(path) {
                return Some(attr.meta.span());
            }
        }
    }
    None
}

/// Run checks on the derive input. Will abort if the input is invalid.
pub(crate) fn derive_input_checks(ast: &syn::DeriveInput) {
    if !is_struct(ast) {
        abort!(
            ast,
            "The `impl_new::New` macro can only be used on structs."
        );
    } else if let Some(sp) = is_path_exist("impl_new", &ast.attrs) {
        abort!(
            sp,
            "The `impl_new` attribute is not supported on the struct itself.";
            help = "The `impl_new` attribute is only supported on the fields."
        )
    }
}

/// Abort the given error
pub(crate) fn abort_error(errors: darling::Error, supported_fields: &[&str]) {
    for err in errors.flatten() {
        let error_msg = err.to_string();
        let mut diagnostic = Diagnostic::spanned(err.span(), Level::Error, error_msg.clone());
        if error_msg.contains("Unexpected literal type") && error_msg.contains("name") {
            diagnostic =
                diagnostic.help("The `name` option only accepts string literals.".to_owned());
        } else if error_msg.contains("Unknown") || error_msg.contains("Unexpected") {
            diagnostic = diagnostic.help(format!(
                "Supported field attributes: {}",
                supported_fields.join(", ")
            ));
        }
        diagnostic.abort();
    }
}

/// Run checks on the `impl_new` attribute. Will abort if the attribute is invalid.
///
/// ## Checks
/// ### `name` option
/// - Checks if the `name` option are set for unnamed fields.
/// - Checks if the `name` option value is not empty.
/// - Checks if the `name` option value are a valid identifier.
/// ### `default` option
/// - Checks that the `default` option is not set with the `name` option.
pub(crate) fn impl_new_checks(
    ident: &Option<syn::Ident>,
    ty_span: Span,
    impl_new_attr: &Option<ImplNewAttr>,
) {
    let is_named = ident.is_some();
    if !is_named
        && (impl_new_attr.is_none()
            || matches!(impl_new_attr, Some(ImplNewAttr { name: None, default, .. }) if !default.is_present()))
    {
        abort!(
            ty_span,
            "Unnamed fields must have the `name` option set.";
            help = "Add #[impl_new(name = \"field_name\")] before the type."
        )
    }
    if let Some(ImplNewAttr {
        name: Some(name),
        default,
        ..
    }) = impl_new_attr
    {
        if default.is_present() {
            abort!(
                name.span(),
                "The `default` option cannot be used with the `name` option.";
                help = "Remove the `name` option.";
                note = "The `default` option will remove the field from the generated `new` function, \
                        so the `name` option is not needed."
            )
        }
        if name.is_empty() {
            abort!(
                name.span(),
                "The `name` option value cannot be empty.";
                help = "Add a value to the `name` option."
            )
        } else if syn::parse_str::<syn::Ident>(name.as_ref()).is_err() {
            abort!(
                name.span(),
                "The `name` option value `{}` is not a valid identifier.",
                name.as_ref();
                help = "The `name` option value must be a valid identifier.";
                note = "The `name` option is used to create the argument name of the field."
            )
        }
    }
}

/// Run checks on the `impl_new::New` macro struct fields. Will abort if the field is invalid.
///
/// ## Checks
/// - Checks if `name` option value are duplicated on the fields.
pub(crate) fn new_macro_checks(fields: &[ImplNewField]) {
    let mut names: Vec<&str> = Vec::new();
    for field in fields {
        if let Some(ImplNewAttr {
            name: Some(name), ..
        }) = &field.impl_new_attr
        {
            if names.iter().any(|n| n == name.as_ref()) {
                abort!(
                    name.span(),
                    "Duplicate `name` option value `{}`.",
                    name.as_ref();
                    help = "Remove the duplicate `name` option.";
                    note = "The `name` option value must be unique, its used to create the argument name of the field."
                )
            } else {
                names.push(name.as_str());
            }
        }
    }
}
