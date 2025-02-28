#![allow(clippy::all)]

use quote::{
    format_ident,
    quote,
};
use syn::{
    AttrStyle,
    Attribute,
    Data,
    DeriveInput,
    Meta,
    MetaList,
    parse_macro_input,
};

#[proc_macro_derive(ComponentClasses, attributes(name_prefix))]
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream
{
    let DeriveInput {
        attrs,
        vis,
        ident,
        generics,
        data,
    } = parse_macro_input!(input as syn::DeriveInput);

    let data = if let Data::Enum(data) = data
    {
        data
    }
    else
    {
        unimplemented!()
    };

    let variants = data.variants;

    // TODO figure out how to get a name prefix from this.
    // let name = attrs.iter().find(|attr| {
    //     **attr
    //         == Attribute {
    //             pound_token:   Default::default(),
    //             style:         AttrStyle::Outer,
    //             bracket_token: Default::default(),
    //             meta:          Meta::List(MetaList {
    //                 path:      Path {},
    //                 delimiter: (),
    //                 tokens:    Default::default(),
    //             }),
    //         }
    // });
    let new_ident = format_ident!("Button{}", ident);
    // let output = quote! {
    //     #[derive(Clone, Copy, Debug, Eq, Default,
    // derive_more::Display, PartialEq, AsRefStr)]
    //     pub enum  #new_ident {
    //         #(#variants)*
    //     }
    // };
    let output = quote! {};
    output.into()
}
