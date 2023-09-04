use darling::{
    util::{Flag, SpannedValue},
    FromMeta,
};
use proc_macro_error::abort;

#[derive(Debug, Clone, FromMeta)]
#[non_exhaustive]
pub(crate) struct ImplNewAttr {
    pub name: Option<SpannedValue<String>>,
    pub default: Flag,
}

impl ImplNewAttr {
    /// Returns the supported options for the `impl_new` attribute.
    pub(crate) const fn supported_options() -> &'static [&'static str] {
        &["`name = \"field_name\"`", "`default`"]
    }

    /// Merges the attributes. Will abort if there a duplicates.
    pub(crate) fn merge(&mut self, others: &[Self]) {
        /// Checks if there are duplicates. Will abort if there are.
        macro_rules! check_dup {
        ($field:ident) => {
            let mut attrs = vec![self.clone()];
            attrs.extend_from_slice(others);
            let attrs = attrs.into_iter().filter_map(|attr| attr.$field.clone());
            if attrs.clone().count() > 1 {
                abort!(
                    attrs.last().unwrap().span(),
                    "Duplicate `impl_new` attribute for `{}`.",
                    stringify!($field);
                    help = "Remove the duplicate attributes."
                )
            }
        };
        ($($field:ident),*) => {
            $(check_dup!($field);)*
        };
    }
        /// Merges the attributes options.\
        macro_rules! merge_opts {
        ($field:ident) => {
            let opt = others.iter().find_map(|attr| {
                if let Some(ref opt) = attr.$field {
                    Some(opt.clone())
                } else {
                    None
                }
            });

            if self.$field.is_none() {
                if let Some(opt) = opt {
                    self.$field = Some(opt);
                }
            }
        };
        ($($field:ident),*) => {
            $(merge_opts!($field);)*
        };
    }
        check_dup!(name);
        merge_opts!(name);
    }
}
