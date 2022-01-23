extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(PaginationQuery)]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;
    let query_name = format!("{}Query", name);
    let query_ident = syn::Ident::new(&query_name, name.span());
    let fields = if let syn::Data::Struct(syn::DataStruct {
        fields: syn::Fields::Named(syn::FieldsNamed { ref named, .. }),
        ..
    }) = ast.data
    {
        named
    } else {
        unimplemented!();
    };
    let fields = fields
        .into_iter()
        .map(|f| {
            let name = &f.ident;
            let ty = &f.ty;
            if ty_inner_type("Option", ty).is_some() {
                quote! {#name: #ty}
            } else {
                quote! {#name: std::option::Option<#ty>}
            }
        })
        .chain(
            vec![
                quote! { limit: std::option::Option<i32> },
                quote! { offset: std::option::Option<i32> },
            ]
            .into_iter(),
        );

    let derives = vec![
        quote! {
            #[derive(Debug, Clone, Default)]
        },
        #[cfg(feature = "derive_builder")]
        quote! {
            #[derive(derive_builder::Builder)]
            #[builder(default)]
            #[builder(setter(strip_option))]
        },
        #[cfg(feature = "serde")]
        quote! {
            #[derive(serde::Serialize, serde::Deserialize)]
        },
        #[cfg(feature = "ts-rs")]
        quote! {
            #[derive(ts_rs::TS)]
            #[ts(export)]
        },
    ];
    let final_struct = quote! {
        #(#derives)*
        pub struct #query_ident {
            #(#fields,)*
        }
    };

    final_struct.into()
}
fn ty_inner_type<'a>(wrapper: &str, ty: &'a syn::Type) -> Option<&'a syn::Type> {
    if let syn::Type::Path(ref p) = ty {
        if p.path.segments.len() != 1 || p.path.segments[0].ident != wrapper {
            return None;
        }

        if let syn::PathArguments::AngleBracketed(ref inner_ty) = p.path.segments[0].arguments {
            if inner_ty.args.len() != 1 {
                return None;
            }

            let inner_ty = inner_ty.args.first().unwrap();
            if let syn::GenericArgument::Type(ref t) = inner_ty.value() {
                return Some(t);
            }
        }
    }
    None
}
