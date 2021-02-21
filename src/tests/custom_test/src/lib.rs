use proc_macro::TokenStream;

use proc_macro2::Ident;

use syn::{parse_macro_input, ItemFn};

use quote::quote;

#[proc_macro_attribute]
pub fn custom_test(attr: TokenStream, item: TokenStream) -> TokenStream {
    let func = parse_macro_input!(item as ItemFn);
    let inde = &func.sig.ident;
    let attr = parse_macro_input!(attr as Ident);

    let q = quote! {
        #func

        inventory::submit!(
            TestCase {
                cond: $crate::tests::test::TestCondition::#attr,
                test: #inde,
            }
        );
    };
    proc_macro::TokenStream::from(q)
}
