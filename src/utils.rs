use proc_macro2::Span;
use proc_macro_error::abort;
use syn::spanned::Spanned;

/// Returns the span of the invalid derive input, if its not a struct.
pub(crate) fn is_struct(ast: &syn::DeriveInput) -> bool {
    matches!(ast.data, syn::Data::Struct(_))
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
