//! Converts any `impl Trait` in traits, impl blocks, or bare functions into method generics
//!
//! This can be used to work around language issues with `impl Trait`, such as a lack of support in type aliases.
//! It also allows trait mocking with [mockall](https://crates.io/crates/mockall), for traits that use impl Trait in arguments.

use proc_macro::TokenStream;
use quote::quote;
use quote::quote_spanned;
use syn::GenericParam;
use syn::Ident;
use syn::ImplItem;
use syn::ItemImpl;
use syn::ItemTrait;
use syn::Signature;
use syn::TraitItem;
use syn::TypeParam;
use syn::{parse_macro_input, spanned::Spanned, FnArg, Item, ItemFn, Type};

/// Rewrites `impl Trait` into a method generic on functions, trait definitions, and trait implementations.
#[proc_macro_attribute]
pub fn into_generic(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as Item);

    match item {
        Item::Trait(item_trait) => trait_into_generic(item_trait),
        Item::Impl(item_impl) => trait_impl_into_generic(item_impl),
        Item::Fn(function) => function_into_generic(function),
        _ => {
            quote_spanned! { item.span() => compile_error!("RewriteImplTrait must be used on a Trait, Impl, or Fn definition.") #[item] }.into()
        }
    }
}

fn trait_into_generic(mut item_trait: ItemTrait) -> TokenStream {
    for item in &mut item_trait.items {
        if let TraitItem::Method(method) = item {
            sig_into_generic(&mut method.sig);
        }
    }

    quote! {
        #item_trait
    }
    .into()
}

fn trait_impl_into_generic(mut item_trait: ItemImpl) -> TokenStream {
    for item in &mut item_trait.items {
        if let ImplItem::Method(method) = item {
            sig_into_generic(&mut method.sig);
        }
    }

    quote! {
        #item_trait
    }
    .into()
}

fn function_into_generic(mut function: ItemFn) -> TokenStream {
    sig_into_generic(&mut function.sig);

    quote! {
        #function
    }
    .into()
}

fn sig_into_generic(sig: &mut Signature) {
    let input_span = sig.span().clone();
    let mut index = 0;
    let mut new_generics: Vec<GenericParam> = vec![];

    for input in &mut sig.inputs {
        if let FnArg::Typed(pat_ty) = input {
            let bounds = if let Type::ImplTrait(impl_trait) = &*pat_ty.ty {
                impl_trait.bounds.clone()
            } else {
                continue;
            };

            let param = format!("RewriteImplTrait{}", index);
            let ident = Ident::new(&param, input_span.clone());
            index += 1;

            new_generics.push(GenericParam::Type(TypeParam {
                ident: ident.clone(),
                bounds,
                attrs: vec![],
                colon_token: None,
                eq_token: None,
                default: None,
            }));

            pat_ty.ty = Box::new(Type::Verbatim(quote! { #ident }));
        }
    }

    for new_param in new_generics {
        sig.generics.params.push(new_param);
    }
}
