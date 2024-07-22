/*
 * SPDX-License-Identifier: GNU GENERAL PUBLIC LICENSE Version 3
 *
 * Copyleft (c) 2024 James Wong. This file is part of James Wong.
 * is free software: you can redistribute it and/or modify it under
 * the terms of the GNU General Public License as published by the
 * Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * James Wong is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with James Wong.  If not, see <https://www.gnu.org/licenses/>.
 *
 * IMPORTANT: Any software that fully or partially contains or uses materials
 * covered by this license must also be released under the GNU GPL license.
 * This includes modifications and derived works.
 */

/*
TODO: 待解决
在 types/users.rs 的 SaveUserRequest 中使用时报错：
proc-macro panicked: SmartCopy requires smart_copy attributerust-analyzermacro-error
proc-macro derive panicked
message: SmartCopy requires smart_copy attribute
*/

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input,
    DeriveInput,
    parse::Parse,
    parse::ParseStream,
    Ident,
    LitStr,
    Token,
    parenthesized,
};

struct SmartCopyArgs {
    target: Ident,
}

impl Parse for SmartCopyArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        parenthesized!(content in input);
        // let target_ident: Ident = content.parse()?;
        content.parse::<Token![=]>()?;
        let target_str: LitStr = content.parse()?;
        let target = Ident::new(&target_str.value(), target_str.span());
        Ok(SmartCopyArgs { target })
    }
}

#[proc_macro_derive(SmartCopy, attributes(smart_copy))]
pub fn smart_copy(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    // 添加调试信息，输出所有属性
    for attr in &input.attrs {
        eprintln!("Attribute: {:?}", attr);
    }

    let smart_copy_attr = input.attrs
        .iter()
        .find(|attr| attr.path().is_ident("smart_copy"))
        .expect("SmartCopy requires smart_copy attribute");

    // 添加调试信息，输出找到的属性
    eprintln!("Found smart_copy attribute: {:?}", smart_copy_attr);

    let args = smart_copy_attr
        .parse_args::<SmartCopyArgs>()
        .expect("Failed to parse SmartCopy arguments");

    let target = &args.target;

    let fields = if
        let syn::Data::Struct(syn::DataStruct { fields: syn::Fields::Named(fields), .. }) =
            input.data
    {
        fields
    } else {
        panic!("SmartCopy only works on structs with named fields");
    };

    let field_names: Vec<_> = fields.named
        .iter()
        .map(|f| &f.ident)
        .collect();

    let field_types: Vec<_> = fields.named
        .iter()
        .map(|f| &f.ty)
        .collect();

    let gen =
        quote! {
        impl #name {
            pub fn smart_copy(&self) -> #target {
                let mut result = #target::default();
                #(
                    if let Some(value) = smart_copy_core::SmartCopySource::get_field(self, stringify!(#field_names)) {
                        smart_copy_core::SmartCopyTarget::set_field(&mut result, stringify!(#field_names), value);
                    }
                )*
                result
            }
        }

        impl smart_copy_core::SmartCopySource for #name {
            fn get_field(&self, name: &str) -> Option<&dyn std::any::Any> {
                match name {
                    #(
                        stringify!(#field_names) => Some(&self.#field_names as &dyn std::any::Any),
                    )*
                    _ => None,
                }
            }
        }

        impl smart_copy_core::SmartCopyTarget for #target {
            fn set_field(&mut self, name: &str, value: &dyn std::any::Any) {
                match name {
                    #(
                        stringify!(#field_names) => {
                            if let Some(v) = value.downcast_ref::<#field_types>() {
                                self.#field_names = v.clone();
                            }
                        },
                    )*
                    _ => {}
                }
            }
        }
    };

    gen.into()
}
