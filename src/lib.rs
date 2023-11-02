extern crate proc_macro;

use std::borrow::Borrow;

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{
    parse_macro_input, punctuated::Punctuated, Ident, ItemImpl, ItemTrait, Path, PathArguments,
    PathSegment, Token, TraitBound, TraitBoundModifier, TypeParamBound,
};

struct SealingTrait {
    enclosing_module: Ident,
    trait_ident: Ident,
    trait_path: Path,
}

#[proc_macro_attribute]
pub fn sealed(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut private_trait = parse_macro_input!(item as ItemTrait);

    let sealing_trait = make_sealing_trait(&private_trait.ident);

    private_trait
        .supertraits
        .push(TypeParamBound::Trait(TraitBound {
            paren_token: None,
            modifier: TraitBoundModifier::None,
            lifetimes: None,
            path: sealing_trait.trait_path,
        }));

    let sealing_trait_mod_ident = sealing_trait.enclosing_module;
    let sealing_trait_ident = sealing_trait.trait_ident;

    quote! {
        mod #sealing_trait_mod_ident {
            pub trait #sealing_trait_ident {}
        }

        #private_trait
    }
    .into()
}

#[proc_macro_attribute]
pub fn impl_sealed(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let impl_block = parse_macro_input!(item as ItemImpl);
    let trait_ = impl_block.clone().trait_.unwrap();
    let private_trait = trait_.1.get_ident().unwrap();
    let self_ = impl_block.clone().self_ty;

    let sealing_trait_path = make_sealing_trait(private_trait).trait_path;

    quote! {
        impl #sealing_trait_path for #self_ {}

        #impl_block
    }
    .into()
}

fn make_sealing_trait(trait_ident: impl Borrow<Ident>) -> SealingTrait {
    let mut private_trait_segments = Punctuated::new();

    private_trait_segments.push(PathSegment {
        ident: Token![crate](Span::mixed_site()).into(),
        arguments: PathArguments::None,
    });

    let private_module = Ident::new(
        &format!(
            "private_trait_{}",
            trait_ident.borrow().to_string().to_lowercase()
        ),
        trait_ident.borrow().span(),
    );

    private_trait_segments.push(PathSegment {
        ident: private_module.clone(),
        arguments: PathArguments::None,
    });

    let supertrait_ident = Ident::new("Sealed", Span::mixed_site());

    private_trait_segments.push(PathSegment {
        ident: supertrait_ident.clone(),
        arguments: PathArguments::None,
    });

    SealingTrait {
        enclosing_module: private_module,
        trait_ident: supertrait_ident,
        trait_path: Path {
            leading_colon: None,
            segments: private_trait_segments,
        },
    }
}
