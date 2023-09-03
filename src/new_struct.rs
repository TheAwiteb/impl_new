use proc_macro2::Span;
use syn::spanned::Spanned;

use crate::{fields::ImplNewField, utils};

pub(crate) struct NewStruct {
    pub(crate) ident: syn::Ident,
    pub(crate) is_tuple_struct: bool,
    pub(crate) fields: Vec<ImplNewField>,
    pub(crate) span: Span,
}

impl NewStruct {
    pub(crate) fn new(ast: &syn::DeriveInput, fields: Vec<ImplNewField>) -> Self {
        let ident = ast.ident.clone();
        let is_tuple_struct = utils::is_tuple_struct(ast);
        let span = ast.span();
        Self {
            ident,
            is_tuple_struct,
            fields,
            span,
        }
    }
}
