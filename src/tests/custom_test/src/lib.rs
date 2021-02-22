use proc_macro::TokenStream;

use proc_macro2::Ident;

use syn::{parse_macro_input, ItemFn};

use quote::{format_ident, quote};

#[proc_macro_attribute]
pub fn custom_test(attr: TokenStream, item: TokenStream) -> TokenStream {
    let test_func = parse_macro_input!(item as ItemFn);
    let test_func_idnet = &test_func.sig.ident;

    let tc_provider_ident = format_ident!("{}_test_case_provider", test_func_idnet);

    let condition = parse_macro_input!(attr as Ident);

    let condition_path = quote! { crate::tests::test::TestCondition };

    let q = quote! {
        #[allow(unused)]
        #test_func

        #[test_case]
        fn #tc_provider_ident() -> (#condition_path, alloc::boxed::Box<Fn()>, &'static str) {
            fn get_name<T>(_: T) -> &'static str {
                core::any::type_name::<T>()
            }
            use crate::tests::test::TestCondition::*;
            (#condition, alloc::boxed::Box::new(#test_func_idnet), get_name(#test_func_idnet))
        }
    };
    proc_macro::TokenStream::from(q)
}
