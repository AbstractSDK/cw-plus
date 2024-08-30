extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemEnum};
const INTERFACE_POSTFIX: &str = "Interface";

// FIXME: it doesn't support generics (we don't need it right now )
#[proc_macro_derive(FromInterface, attributes(renamed))]
pub fn from_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as ItemEnum);
    let name = &ast.ident;
    let counter_name = {
        let counter_name = name.to_string();
        // Yep hardcoding here
        let (counter_name, _) = counter_name
            .split_once(INTERFACE_POSTFIX)
            .expect(r#"Interface message type supposed to have "Interface" postfix"#);
        proc_macro2::Ident::new(counter_name, proc_macro2::Span::call_site())
    };
    let froms = ast.variants.into_iter().map(|variant| {
        let variant_name = variant.ident.clone();
        let counter_variant_name = variant
            .attrs
            .into_iter()
            .find_map(|attr| {
                let syn::Meta::List(meta_list) = attr.meta else {
                    return None;
                };
                if meta_list.path.is_ident("renamed") {
                    let ident = meta_list.parse_args::<proc_macro2::Ident>().unwrap();
                    Some(ident)
                } else {
                    None
                }
            })
            .unwrap_or(variant.ident);
        let fields = match variant.fields {
            syn::Fields::Unnamed(variant_fields) => {
                let variant_fields = (0..variant_fields.unnamed.len()).map(|i| {
                    proc_macro2::Ident::new(&format!("arg{i}"), proc_macro2::Span::call_site())
                });
                quote!( ( #(#variant_fields,)* ) )
            }
            syn::Fields::Named(variant_fields) => {
                let idents = variant_fields
                    .named
                    .into_iter()
                    .map(|field| field.ident.unwrap());
                quote!( { #(#idents,)* } )
            }
            syn::Fields::Unit => quote!(),
        };
        quote! ( #name::#variant_name #fields => #counter_name::#counter_variant_name #fields )
    });
    quote!(
        impl From<#name> for #counter_name {
            fn from(value: #name) -> #counter_name {
                match value {
                    #(#froms,)*
                }
            }
        }
    )
    .into()
}
