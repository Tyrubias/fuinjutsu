mod supertrait_sealed;

extern crate proc_macro;

use supertrait_sealed::{make_supertrait_seal, make_supertrait_seal_impl};

use proc_macro::TokenStream;
use syn::{parse_macro_input, ItemImpl, ItemTrait};

#[proc_macro_attribute]
pub fn supertrait_sealed(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let private_trait = parse_macro_input!(item as ItemTrait);

    make_supertrait_seal(private_trait).into()
}

#[proc_macro_attribute]
pub fn impl_supertrait_sealed(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let impl_block = parse_macro_input!(item as ItemImpl);

    make_supertrait_seal_impl(impl_block).into()
}
