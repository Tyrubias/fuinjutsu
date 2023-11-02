use std::borrow::Borrow;

use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{
    punctuated::Punctuated, Ident, ItemImpl, ItemTrait, Path, PathArguments, PathSegment, Token,
    TraitBound, TraitBoundModifier, TypeParamBound,
};

pub fn make_supertrait_seal(private_trait: ItemTrait) -> syn::Result<TokenStream> {
    let sealing_trait = make_sealing_trait(&private_trait.ident);

    let mut private_trait = private_trait.clone();

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

    Ok(quote! {
        mod #sealing_trait_mod_ident {
            pub trait #sealing_trait_ident {}
        }

        #private_trait
    })
}

pub fn make_supertrait_seal_impl(private_trait_impl: ItemImpl) -> syn::Result<TokenStream> {
    let private_trait_impl = private_trait_impl.clone();

    let private_trait = match private_trait_impl.trait_ {
        Some((_, ref trait_, _)) => match trait_.get_ident() {
            Some(trait_ident) => trait_ident.clone(),
            None => {
                return Err(syn::Error::new_spanned(
                    trait_,
                    "expected trait name not path to trait",
                ))
            }
        },
        None => {
            return Err(syn::Error::new_spanned(
                private_trait_impl,
                "expected trait implementation",
            ))
        }
    };

    let self_ = private_trait_impl.clone().self_ty;

    let sealing_trait_path = make_sealing_trait(private_trait).trait_path;

    Ok(quote! {
        impl #sealing_trait_path for #self_ {}

        #private_trait_impl
    })
}

struct SealingTrait {
    enclosing_module: Ident,
    trait_ident: Ident,
    trait_path: Path,
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
