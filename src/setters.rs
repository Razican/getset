use quote::{Ident, Tokens};
use syn::{Field, Lit, MetaItem};

const ATTRIBUTE_NAME: &'static str = "set";
const FN_NAME_PREFIX: &'static str = "set_";
const FN_NAME_SUFFIX: &'static str = "";

pub fn implement(field: &Field) -> Tokens {
    let field_name = field
        .clone()
        .ident
        .expect("Expected the field to have a name");
    let fn_name = Ident::from(format!(
        "{}{}{}",
        FN_NAME_PREFIX, field_name, FN_NAME_SUFFIX
    ));
    let ty = field.ty.clone();
    let attr = field
        .attrs
        .iter()
        .filter(|v| v.name() == ATTRIBUTE_NAME)
        .last();

    let doc = field
        .attrs
        .iter()
        .filter(|v| v.name() == "doc")
        .collect::<Vec<_>>();

    match attr {
        Some(attr) => {
            match attr.value {
                // `#[set]`
                MetaItem::Word(_) => {
                    quote! {
                        #(#doc)*
                        #[inline(always)]
                        fn #fn_name(&mut self, val: #ty) -> &mut Self {
                            self.#field_name = val;
                            self
                        }
                    }
                }
                // `#[set = "pub"]`
                MetaItem::NameValue(_, Lit::Str(ref s, _)) => {
                    let visibility = Ident::from(s.clone());
                    quote! {
                        #(#doc)*
                        #[inline(always)]
                        #visibility fn #fn_name(&mut self, val: #ty) -> &mut Self {
                            self.#field_name = val;
                            self
                        }
                    }
                }
                // This currently doesn't work, but it might in the future.
                /// ---
                // // `#[set(pub)]`
                // MetaItem::List(_, ref vec) => {
                //     let s = vec.iter().last().expect("No item found in attribute list.");
                //     let visibility = match s {
                //         &NestedMetaItem::MetaItem(MetaItem::Word(ref i)) => Ident::new(format!("{}", i)),
                //         &NestedMetaItem::Literal(Lit::Str(ref l, _)) => Ident::from(l.clone()),
                //         _ => panic!("Unexpected attribute parameters."),
                //     };
                //     quote! {
                //         #visibility fn #fn_name(&self) -> &#ty {
                //             &self.#field_name
                //         }
                //     }
                // },
                _ => panic!("Unexpected attribute parameters."),
            }
        }
        // Don't need to do anything.
        None => quote!{},
    }
}
