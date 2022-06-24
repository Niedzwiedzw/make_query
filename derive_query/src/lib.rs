extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input,
    DeriveInput,
};

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
                quote! {
                pub #name: #ty}
            } else {
                quote! {pub #name: Option<#ty>}
            }
        })
        .chain(
            vec![
                quote! { pub limit: Option<i32> },
                quote! { pub offset: Option<i32> },
            ]
            .into_iter(),
        );
    #[cfg(feature = "ts-rs")]
    let fields = fields.map(|field| {
        quote! {
            #[ts(optional)]
            #field
        }
    });

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
        #[cfg(feature = "async-graphql-input-type")]
        quote! {
            #[derive(async_graphql::InputObject)]
        },
        #[cfg(feature = "juniper-input-type")]
        quote! {
            #[derive(juniper::GraphQLInputObject)]
        },
    ];
    let builder_name = format!("{}Builder", query_name);
    let builder_ident = syn::Ident::new(&builder_name, name.span());
    let final_struct = quote! {
        #(#derives)*
        pub struct #query_ident {
            #(#fields,)*
        }

        impl #query_ident {
            pub fn builder() -> #builder_ident {
                #builder_ident::default()
            }
        }

        impl #builder_ident {
            pub fn construct(&self) -> std::result::Result<#query_ident, crate::error::QueryError> {
                self.build()
                    .map_err(|e| crate::error::QueryError::QueryError(format!("{:?}", e)))
            }
        }
    };
    // println!("{}", final_struct.clone().to_string());

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
