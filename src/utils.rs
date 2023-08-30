use proc_macro2::Span;
use syn::spanned::Spanned;

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
