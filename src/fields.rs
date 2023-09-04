use crate::{attrs::ImplNewAttr, utils};
use darling::FromMeta;
use proc_macro2::{Span, TokenStream};
use proc_macro_error::abort;
use syn::spanned::Spanned;

#[derive(Debug, Clone)]
pub(crate) struct ImplNewField {
    /// The span of the field.
    pub(crate) span: Span,
    /// The name of the field. Will be None if the field is unnamed.
    pub(crate) ident: Option<syn::Ident>,
    /// The type of the field.
    pub(crate) ty: syn::Type,
    /// `#[impl_new(...)]` attribute.
    pub(crate) impl_new_attr: Option<ImplNewAttr>,
}

impl ImplNewField {
    pub(crate) fn parse(field: syn::Field) -> syn::Result<Self> {
        let span = field.span();
        let ident = field.ident.clone();
        let ty = field.ty.clone();
        let impl_new_attrs = field
            .attrs
            .iter()
            .filter_map(|attr| {
                if attr.path().is_ident("impl_new") {
                    if let Ok(meta_list) = attr.meta.require_list() {
                        Some(meta_list.tokens.clone())
                    } else {
                        abort!(
                            attr,
                            "Invalid `impl_new` attribute, expected #[impl_new(...)]."
                        );
                    }
                } else {
                    None
                }
            })
            .map(|attr: TokenStream| {
                match ImplNewAttr::from_list(&darling::ast::NestedMeta::parse_meta_list(attr)?) {
                    Ok(opts) => Ok(opts),
                    Err(err) => {
                        utils::abort_error(err, ImplNewAttr::supported_options());
                        unreachable!()
                    }
                }
            })
            .collect::<syn::Result<Vec<ImplNewAttr>>>()?;
        let impl_new_attr = if let Some((first, rest)) = impl_new_attrs.split_first() {
            let mut opts = first.clone();
            opts.merge(rest);
            Some(opts)
        } else {
            None
        };
        utils::impl_new_checks(&ident, ty.span(), &impl_new_attr);
        Ok(Self {
            span,
            ident,
            ty,
            impl_new_attr,
        })
    }
}

impl ImplNewField {
    /// Returns the argument name of the field.
    pub fn arg_name(&self) -> Option<syn::Ident> {
        if matches!(self.impl_new_attr, Some(ImplNewAttr { default, .. }) if default.is_present()) {
            None
        } else if let Some(name) = self
            .impl_new_attr
            .as_ref()
            .and_then(|attr| attr.name.as_ref())
        {
            Some(syn::Ident::new(name, name.span()))
        } else {
            Some(
                self.ident
                    .clone()
                    .expect("This will never happen, the unnamed fields are checked."),
            )
        }
    }

    /// Returns the field name (For named fields only) if the field is unnamed it will panic.
    pub fn field_name(&self) -> syn::Ident {
        self.ident
            .clone()
            .expect("Unnamed fields cannot be accessed.")
    }

    /// Returns the field value.
    pub fn value(&self) -> syn::Expr {
        if matches!(self.impl_new_attr, Some(ImplNewAttr { default, .. }) if default.is_present()) {
            let ty = &self.ty;
            syn::parse_quote! { #ty::default() }
        } else {
            let arg_name = self.arg_name();
            syn::parse_quote! { #arg_name.into() }
        }
    }
}
