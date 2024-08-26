use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{
    punctuated::Punctuated, Ident, ItemImpl, ItemTrait, Path, PathArguments, PathSegment,
    TraitBound, TraitBoundModifier, TypeParamBound,
};

pub(crate) fn make_supertrait_seal(mut private_trait: ItemTrait) -> syn::Result<TokenStream> {
    let SealingTrait {
        module,
        supertrait,
        trait_path,
    } = SealingTrait::new(&private_trait.ident);

    let trait_bound = TypeParamBound::Trait(TraitBound {
        paren_token: None,
        modifier: TraitBoundModifier::None,
        lifetimes: None,
        path: trait_path,
    });

    private_trait.supertraits.push(trait_bound);

    Ok(quote! {
        mod #module {
            pub trait #supertrait {}
        }

        #private_trait
    })
}

pub(crate) fn make_supertrait_seal_impl(private_trait_impl: ItemImpl) -> syn::Result<TokenStream> {
    let trait_ = match private_trait_impl.trait_ {
        Some((_, ref trait_, _)) => trait_.clone(),
        None => {
            return Err(syn::Error::new_spanned(
                private_trait_impl,
                "expected trait implementation",
            ))
        }
    };

    let private_trait = match trait_.segments.iter().last() {
        Some(PathSegment {
            ident,
            arguments: _,
        }) => ident,
        None => return Err(syn::Error::new_spanned(trait_, "expected trait identifier")),
    };

    let SealingTrait {
        module: _,
        supertrait: _,
        trait_path,
    } = SealingTrait::new(private_trait);

    let self_ = private_trait_impl.clone().self_ty;

    Ok(quote! {
        impl #trait_path for #self_ {}

        #private_trait_impl
    })
}

struct SealingTrait {
    module: Ident,
    supertrait: Ident,
    trait_path: Path,
}

impl SealingTrait {
    fn new(trait_ident: &Ident) -> Self {
        let module = format_ident!("private_trait_{}", trait_ident.to_string().to_lowercase());

        let supertrait = format_ident!("Sealed");

        let trait_path = Path {
            leading_colon: None,
            segments: Punctuated::from_iter(vec![
                PathSegment {
                    ident: format_ident!("crate"),
                    arguments: PathArguments::None,
                },
                PathSegment {
                    ident: module.clone(),
                    arguments: PathArguments::None,
                },
                PathSegment {
                    ident: supertrait.clone(),
                    arguments: PathArguments::None,
                },
            ]),
        };

        Self {
            module,
            supertrait,
            trait_path,
        }
    }
}
