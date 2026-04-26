use proc_macro::TokenStream;

use proc_macro2::Span;
use quote::quote;
use syn::meta::ParseNestedMeta;
use syn::parse::Parser;
use syn::{DeriveInput, Ident, LitStr, parse_macro_input};

#[proc_macro_attribute]
pub fn cache(attr: TokenStream, item: TokenStream) -> TokenStream {
    let item2 = proc_macro2::TokenStream::from(item.clone());
    let input = parse_macro_input!(item as DeriveInput);
    let name = &input.ident;

    let mut key_prefix = String::new();
    let mut field_name = String::new();

    let attr2: proc_macro2::TokenStream = attr.into();
    syn::meta::parser(|meta: ParseNestedMeta| {
        if meta.path.is_ident("key") {
            let val: LitStr = meta.value()?.parse()?;
            key_prefix = val.value();
        } else if meta.path.is_ident("field") {
            let val: LitStr = meta.value()?.parse()?;
            field_name = val.value();
        }
        Ok(())
    })
    .parse2(attr2)
    .expect("invalid cache attribute args");

    let field_ident = Ident::new(&field_name, Span::call_site());
    let fmt = format!("{key_prefix}:{{}}");

    quote! {
        #item2

        impl crate::models::Cacheable for #name {
            fn cache_key(&self) -> String {
                format!(#fmt, self.#field_ident)
            }
        }
    }
    .into()
}