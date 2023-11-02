extern crate proc_macro;

use std::borrow::Borrow;

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{
    parse_macro_input, punctuated::Punctuated, Ident, ItemImpl, ItemTrait, Path, PathArguments,
    PathSegment, Token, TraitBound, TraitBoundModifier, TypeParamBound,
};

struct PrivateTrait {
    private_module: Ident,
    supertrait_ident: Ident,
    full_trait_path: Path,
}

#[proc_macro_attribute]
pub fn sealed(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut r#trait = parse_macro_input!(item as ItemTrait);

    let private_trait = get_sealed_trait_path(&r#trait.ident);

    r#trait.supertraits.push(TypeParamBound::Trait(TraitBound {
        paren_token: None,
        modifier: TraitBoundModifier::None,
        lifetimes: None,
        path: private_trait.full_trait_path,
    }));

    let private_module = private_trait.private_module;
    let supertrait_ident = private_trait.supertrait_ident;

    quote! {
        mod #private_module {
            pub trait #supertrait_ident {}
        }

        #r#trait
    }
    .into()
}

#[proc_macro_attribute]
pub fn impl_sealed(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let impl_block = parse_macro_input!(item as ItemImpl);
    let trait_ = impl_block.clone().trait_.unwrap();
    let private_trait = trait_.1.get_ident().unwrap();
    let self_ = impl_block.clone().self_ty;

    let sealed_trait_path = get_sealed_trait_path(private_trait).full_trait_path;

    quote! {
        impl #sealed_trait_path for #self_ {}

        #impl_block
    }
    .into()
}

fn get_sealed_trait_path(trait_ident: impl Borrow<Ident>) -> PrivateTrait {
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

    PrivateTrait {
        private_module,
        supertrait_ident,
        full_trait_path: Path {
            leading_colon: None,
            segments: private_trait_segments,
        },
    }
}
