use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{
    punctuated::{Pair, Punctuated},
    Ident, ItemImpl, ItemTrait, Path, PathArguments, PathSegment, TraitBound, TraitBoundModifier,
    TypeParamBound,
};

pub fn make_supertrait_seal(mut private_trait: ItemTrait) -> syn::Result<TokenStream> {
    let SealingTrait {
        module,
        supertrait,
        trait_path,
    } = make_sealing_trait(&private_trait.ident);

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

pub fn make_supertrait_seal_impl(private_trait_impl: ItemImpl) -> syn::Result<TokenStream> {
    let mut trait_path = match private_trait_impl.trait_ {
        Some((_, ref trait_, _)) => trait_.clone(),
        None => {
            return Err(syn::Error::new_spanned(
                private_trait_impl,
                "expected trait implementation",
            ))
        }
    };

    let private_trait = match trait_path.segments.pop() {
        Some(
            Pair::Punctuated(
                PathSegment {
                    ident,
                    arguments: _,
                },
                _,
            )
            | Pair::End(PathSegment {
                ident,
                arguments: _,
            }),
        ) => ident,
        None => return Err(syn::Error::new_spanned(trait_path, "expected trait path")),
    };

    let SealingTrait {
        module,
        supertrait,
        trait_path: _,
    } = make_sealing_trait(&private_trait);

    trait_path.segments.push(PathSegment {
        ident: module,
        arguments: PathArguments::None,
    });

    trait_path.segments.push(PathSegment {
        ident: supertrait,
        arguments: PathArguments::None,
    });

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

fn make_sealing_trait(trait_ident: &Ident) -> SealingTrait {
    let module_name = format!("private_trait_{}", trait_ident.to_string().to_lowercase());
    let module = Ident::new(&module_name, trait_ident.span());

    let supertrait = Ident::new("Sealed", Span::call_site());

    let mut trait_segments = Punctuated::new();

    trait_segments.push(PathSegment {
        ident: module.clone(),
        arguments: PathArguments::None,
    });

    trait_segments.push(PathSegment {
        ident: supertrait.clone(),
        arguments: PathArguments::None,
    });

    let trait_path = Path {
        leading_colon: None,
        segments: trait_segments,
    };

    SealingTrait {
        module,
        supertrait,
        trait_path,
    }
}
