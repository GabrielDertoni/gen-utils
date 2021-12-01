#![feature(box_patterns)]

use proc_macro::TokenStream;
use syn::{
    parse_macro_input,
    Item,
    ReturnType,
    Type,
    TypeParamBound,
    TraitBound,
    Path,
    PathSegment,
    PathArguments,
    GenericArgument,
};
use quote::quote;

macro_rules! extract {
    ($expr:expr, $pat:pat => $out:expr) => {
        match $expr {
            $pat => Some($out),
            _    => None,
        }
    };
}

#[proc_macro_attribute]
pub fn generator(_args: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as Item);

    let mut func = if let Item::Fn(func) = input {
        func
    } else {
        panic!("expected a function")
    };

    drop(std::mem::take(&mut func.attrs));

    let body = &func.block;
    func.block = syn::parse2(quote! {
        {
            genutils::GenWrapper::new(Box::pin(move || #body))
        }
    })
    .expect("Parse error");

    let expected_impl_gen: &'static str = "Function signature has by like `impl Generator<Yield = Y>` and \
                                           optinally be `impl Generator<Yield = Y, Return = R>. Y and R \
                                           are respectively the type to yield and the type to return.";

    let type_impl = extract!(func.sig.output, ReturnType::Type(_, box Type::ImplTrait(ref mut i)) => i)
        .expect(expected_impl_gen)
        .bounds
        .first_mut()
        .expect(expected_impl_gen);

    let path_segments = extract!(type_impl,
        TypeParamBound::Trait(TraitBound { path: Path { segments, ..}, .. }) => segments
    ).expect(expected_impl_gen);

    let assoc_types = &mut path_segments.iter_mut()
        .filter_map(|seg|
            extract!(seg, PathSegment { arguments: PathArguments::AngleBracketed(assoc_type), ..} => assoc_type)
        )
        .last()
        .expect(expected_impl_gen)
        .args;

    assert!(
        assoc_types.iter()
            .filter_map(|gen_arg| extract!(gen_arg, GenericArgument::Binding(binding) => binding))
            .any(|binding| binding.ident == "Yield"),
        "{}", expected_impl_gen
    );
    
    let has_return_type = assoc_types.iter()
        .filter_map(|gen_arg| extract!(gen_arg, GenericArgument::Binding(binding) => binding))
        .any(|binding| binding.ident == "Return");
    
    if !has_return_type {
        assoc_types.push(syn::parse2(quote!{ Return = () }).unwrap())
    }

    let bounds = &extract!(func.sig.output, ReturnType::Type(_, box Type::ImplTrait(ref mut i)) => i)
        .expect(expected_impl_gen)
        .bounds;

    func.sig.output = syn::parse2(quote!{
        -> genutils::GenWrapper<std::pin::Pin<Box<dyn #bounds>>>
    }).expect("Parse error");
    
    TokenStream::from(quote! { #func })
}