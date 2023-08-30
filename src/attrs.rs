use proc_macro2::{Ident, Literal, Span, TokenTree};
use proc_macro_error::abort;
use syn::spanned::Spanned;

const SUPPORTED_ATTRIBUTES: &str = "The supported attributes are: `name`";

/// A struct that holds the attributes of a field.
#[derive(Clone, Debug)]
pub(crate) struct Attrs {
    /// The name attribute of the field.
    pub name: Option<(Ident, Span)>,
}

#[derive(Clone, Debug)]
pub(crate) enum AttrType {
    Empty(Span),
    MetaWord(Ident, Span),
    MetaNameValueStr(Ident, Literal, Span),
    Else(Span),
}

impl AttrType {
    pub fn parse(tokens: &[TokenTree], span: Span) -> Self {
        if tokens.is_empty() {
            return AttrType::Empty(span);
        } else if tokens.len() == 1 {
            if let TokenTree::Ident(ident) = &tokens[0] {
                return AttrType::MetaWord(ident.clone(), span);
            }
        } else if tokens.len() == 3 {
            if let TokenTree::Ident(ident) = &tokens[0] {
                if let TokenTree::Punct(punct) = &tokens[1] {
                    if punct.as_char() == '=' {
                        if let TokenTree::Literal(lit) = &tokens[2] {
                            return AttrType::MetaNameValueStr(ident.clone(), lit.clone(), span);
                        }
                    }
                }
            }
        }
        println!("{:?}", tokens);
        AttrType::Else(span)
    }
}

impl Attrs {
    pub fn parse(attributes: &[syn::Attribute]) -> Self {
        let mut attrs = Attrs { name: None };
        for attribute in attributes {
            if let syn::Meta::List(ref meta_list) = attribute.meta {
                if meta_list.path.is_ident("impl_new") {
                    let tokens = meta_list.tokens.clone().into_iter().collect::<Vec<_>>();
                    let attr_type = AttrType::parse(&tokens, attribute.span());
                    match attr_type {
                        AttrType::Empty(span) => {
                            abort!(
                                span,
                                "Empty attribute is not supported. {}",
                                SUPPORTED_ATTRIBUTES
                            );
                        }
                        AttrType::MetaWord(ident, span) => {
                            abort!(
                                span,
                                "Attribute `{}` is not supported. {}",
                                ident,
                                SUPPORTED_ATTRIBUTES
                            );
                        }
                        AttrType::MetaNameValueStr(ident, lit, span) => {
                            if ident == "name" {
                                if let syn::Lit::Str(lit_str) = syn::Lit::new(lit) {
                                    attrs.name =
                                        Some((Ident::new(lit_str.value().trim(), span), span));
                                } else {
                                    abort!(span, "Attribute `name` must be a string literal");
                                }
                            } else {
                                abort!(
                                    span,
                                    "Attribute `{}` is not supported. {}",
                                    ident,
                                    SUPPORTED_ATTRIBUTES
                                );
                            }
                        }
                        AttrType::Else(span) => {
                            abort!(span, "Attribute is not supported. {}", SUPPORTED_ATTRIBUTES);
                        }
                    }
                }
            }
        }
        attrs
    }
}
