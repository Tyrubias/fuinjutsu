use std::vec;

use proc_macro2::{Ident, Span, TokenStream};
use quote::{format_ident, quote};
use syn::{
    punctuated::Punctuated, FnArg, ImplItem, ItemImpl, ItemTrait, Pat, PatType, PatWild, Path,
    PathArguments, PathSegment, Token, TraitItem, Type, TypePath,
};

pub(crate) fn make_method_seal(mut private_trait: ItemTrait) -> syn::Result<TokenStream> {
    let SealingStruct {
        marker,
        module,
        token,
        unit,
    } = SealingStruct::new(&private_trait.ident);

    for item in &mut private_trait.items {
        if let TraitItem::Fn(ref mut func) = item {
            let marker_idx = func
                .attrs
                .iter()
                .position(|attr| attr.path().is_ident(&marker));

            func.attrs.retain(|attr| !attr.path().is_ident(&marker));

            if marker_idx.is_some() {
                func.sig.inputs.push(unit.clone());
            }
        }
    }

    Ok(quote! {
        mod #module {
            pub struct #token;
        }

        #private_trait
    })
}

pub(crate) fn make_method_seal_impl(mut private_trait_impl: ItemImpl) -> syn::Result<TokenStream> {
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

    let SealingStruct {
        marker,
        module: _,
        token: _,
        unit,
    } = SealingStruct::new(private_trait);

    for item in &mut private_trait_impl.items {
        if let ImplItem::Fn(ref mut func) = item {
            let marker_idx = func
                .attrs
                .iter()
                .position(|attr| attr.path().is_ident(&marker));

            func.attrs.retain(|attr| !attr.path().is_ident(&marker));

            if marker_idx.is_some() {
                func.sig.inputs.push(unit.clone());
            }
        }
    }

    Ok(quote! {
        #private_trait_impl
    })
}

struct SealingStruct {
    marker: Ident,
    module: Ident,
    token: Ident,
    unit: FnArg,
}

impl SealingStruct {
    fn new(trait_ident: &Ident) -> Self {
        let module = format_ident!("private_trait_{}", trait_ident.to_string().to_lowercase());

        let token = format_ident!("Token");

        let unit = FnArg::Typed(PatType {
            attrs: vec![],
            pat: Box::new(Pat::Wild(PatWild {
                attrs: vec![],
                underscore_token: Token![_](Span::call_site()),
            })),
            colon_token: Token![:](Span::call_site()),
            ty: Box::new(Type::Path(TypePath {
                qself: None,
                path: Path {
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
                            ident: token.clone(),
                            arguments: PathArguments::None,
                        },
                    ]),
                },
            })),
        });

        let marker = Ident::new("sealed", Span::call_site());

        Self {
            marker,
            module,
            token,
            unit,
        }
    }
}
