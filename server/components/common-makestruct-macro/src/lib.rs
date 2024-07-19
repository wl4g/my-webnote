use proc_macro::TokenStream;
use quote::{ quote, format_ident };
use syn::{
    parse_macro_input,
    DeriveInput,
    Data,
    Fields,
    Ident,
    parse::Parse,
    parse::ParseStream,
    Token,
    Field,
    Attribute,
};

struct IdentList(Vec<Ident>);

impl Parse for IdentList {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let parsed = input.parse_terminated(Ident::parse, Token![,])?;
        Ok(IdentList(parsed.into_iter().collect()))
    }
}

#[proc_macro_derive(MakeStructWith, attributes(excludes, includes))]
pub fn make_struct_with(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let vis = input.vis;

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let fields = match input.data {
        Data::Struct(ref data) => {
            match data.fields {
                Fields::Named(ref fields) => &fields.named,
                _ => panic!("Only named fields are supported"),
            }
        }
        _ => panic!("Only structs are supported"),
    };

    let mut exclude_fields = Vec::new();
    let mut include_fields = Vec::new();

    for attr in &input.attrs {
        if attr.path().is_ident("excludes") {
            if let Ok(IdentList(list)) = attr.parse_args() {
                exclude_fields.extend(list);
            }
        } else if attr.path().is_ident("includes") {
            if let Ok(IdentList(list)) = attr.parse_args() {
                include_fields.extend(list);
            }
        }
    }

    let filtered_fields: Vec<(&Field, Vec<&Attribute>)> = fields
        .iter()
        .filter_map(|f| {
            let field_name = f.ident.as_ref().unwrap();
            if
                (!exclude_fields.is_empty() && !exclude_fields.contains(field_name)) ||
                (!include_fields.is_empty() && include_fields.contains(field_name)) ||
                (exclude_fields.is_empty() && include_fields.is_empty())
            {
                Some((f, f.attrs.iter().collect()))
            } else {
                None
            }
        })
        .collect();

    let new_fields = filtered_fields.iter().map(|(f, _)| {
        let field_name = &f.ident;
        quote! { #field_name: original.#field_name.clone() }
    });

    let new_struct_fields = filtered_fields.iter().map(|(f, attrs)| {
        let field_name = &f.ident;
        let ty = &f.ty;
        quote! {
            #(#attrs)*
            pub #field_name: #ty
        }
    });

    let new_struct_name = format_ident!("{}With", name);

    let expanded =
        quote! {
        #[derive(Deserialize, Clone, Debug, PartialEq, Validate, utoipa::ToSchema)]
        #vis struct #new_struct_name #impl_generics #where_clause {
            #(#new_struct_fields),*
        }

        impl #impl_generics From<#name #ty_generics> for #new_struct_name #ty_generics #where_clause {
            fn from(original: #name #ty_generics) -> Self {
                Self {
                    #(#new_fields),*
                }
            }
        }
    };

    TokenStream::from(expanded)
}
