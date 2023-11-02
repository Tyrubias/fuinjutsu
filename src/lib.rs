#![forbid(clippy::expect_used)]
#![forbid(clippy::unwrap_used)]
#![forbid(clippy::panic)]
#![forbid(unsafe_code)]

mod supertrait_sealed;

extern crate proc_macro;

use proc_macro::TokenStream;
use syn::{parse_macro_input, ItemImpl, ItemTrait};

use supertrait_sealed::{make_supertrait_seal, make_supertrait_seal_impl};

#[proc_macro_attribute]
pub fn supertrait_sealed(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let private_trait = parse_macro_input!(item as ItemTrait);

    make_supertrait_seal(private_trait)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

#[proc_macro_attribute]
pub fn impl_supertrait_sealed(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let impl_block = parse_macro_input!(item as ItemImpl);

    make_supertrait_seal_impl(impl_block)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}
