extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{ Data, DataStruct, DeriveInput, Fields, FieldsNamed, Ident };

#[proc_macro_derive(PageableQueryRequest, attributes(ToSchema, IntoParams))]
pub fn pageable_query_request_derive(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as DeriveInput);

    let name = &input.ident;
    let fields = match &input.data {
        Data::Struct(DataStruct { fields: Fields::Named(FieldsNamed { named, .. }), .. }) => named,
        Data::Struct(DataStruct { fields: Fields::Unit, .. }) => {
            // 如果是空结构体,直接返回空 TokenStream
            return TokenStream::new();
        }
        _ => panic!("Expected a struct with named fields"),
    };

    let pageable_fields =
        quote! {
        pub offset: Option<u32>,
        pub limit: Option<u32>,
    };

    let mut other_fields = quote! {};

    for field in fields {
        let field_name = &field.ident;
        let field_ty = &field.ty;
        let field_attrs = &field.attrs;

        other_fields =
            quote! {
            #other_fields
            #(#field_attrs)*
            pub #field_name: #field_ty,
        };
    }

    let pageable_name = Ident::new(&format!("Pageable{}", name), name.span());

    let derive_traits = input.attrs.iter().filter_map(|attr| {
        if attr.path().is_ident("utoipa") { None } else { Some(quote! { #attr }) }
    });

    let expanded =
        quote! {
        pub mod #pageable_name {
            #(#derive_traits)*
            #[derive(serde::Deserialize, Clone, Debug, utoipa::ToSchema)]
            pub struct #pageable_name {
                #pageable_fields
                #other_fields
            }
        }
    };

    TokenStream::from(expanded)
}
