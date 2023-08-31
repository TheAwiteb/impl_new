use proc_macro2::Ident;
use proc_macro_error::abort;
use syn::Type;

use crate::attrs::Attrs;

/// A field struct, which is a wrapper around a `String` and a `FieldType`.
#[derive(Clone, Default)]
pub(crate) struct Fields {
    /// The names of the fields.
    pub names: Vec<Ident>,
    /// The types of the fields.
    pub types: Vec<Type>,
    /// The names of the values of the fields.
    pub values: Vec<Ident>,
    pub is_unnamed: bool,
}

impl Fields {
    /// Parse the given derive input into a `Fields` struct.
    pub(crate) fn parse(ast: &syn::DeriveInput) -> Self {
        match ast.data {
            syn::Data::Struct(ref data_struct) => match data_struct.fields {
                syn::Fields::Named(ref fields_named) => Fields::from(fields_named),
                syn::Fields::Unnamed(ref fields_unnamed) => Fields::from(fields_unnamed),
                syn::Fields::Unit => {
                    abort!(
                        ast,
                        "Unit structs are not supported for `impl_new::New` derive macro"
                    );
                }
            },
            _ => unreachable!("We already checked if the derive input is a struct."),
        }
    }
}

impl From<&syn::FieldsNamed> for Fields {
    fn from(fields_named: &syn::FieldsNamed) -> Self {
        let mut fields = Fields::default();
        for field in fields_named.named.iter() {
            let attrs = Attrs::parse(&field.attrs);
            let ident = field.ident.clone().unwrap();
            fields.names.push(ident.clone());
            fields
                .values
                .push(attrs.name.map(|(name, _)| name).unwrap_or(ident));
            fields.types.push(field.ty.clone());
        }
        fields
    }
}

impl From<&syn::FieldsUnnamed> for Fields {
    fn from(fields_unnamed: &syn::FieldsUnnamed) -> Self {
        let mut fields = Fields {
            is_unnamed: true,
            ..Fields::default()
        };
        for field in fields_unnamed.unnamed.iter() {
            let attrs = Attrs::parse(&field.attrs);
            if let Some(name) = attrs.name {
                fields.values.push(name.0);
                fields.types.push(field.ty.clone());
            } else {
                abort!(
                    field,
                    "Unnamed fields must have a name attribute.";
                    help = "Write this after the field: `#[impl_new(name = \"name\")]`";
                    note = "If you don't want to specify a name, use a named field instead."
                );
            }
        }
        fields
    }
}
