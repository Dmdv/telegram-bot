extern crate proc_macro;
use quote::quote;
use syn::{Field, Fields, Ident, Token, Variant};
use proc_macro2::Span;
use syn::punctuated::Punctuated;

fn get_idents(fields: &Punctuated<Field, Token![,]>) -> Vec<Ident> {
    fields.iter().enumerate().map(|(i, f)| {
        Ident::new(&match &f.ident {
            Some(ident) => format!("{}", ident),
            None => format!("field_{}", i),
        }, Span::mixed_site())
    }).collect::<Vec<Ident>>()
}

// fn get_opt_idents(fields: &Punctuated<Field, Token![,]>) -> Vec<Ident> {
//     fields.iter().enumerate().map(|(i, f)| {
//         // match &f.ty {
//         //     Type::Path(type_path) => {
//         //         if type_path.path.segments[0].ident == "Option" {
//         //             return quote! { match #ident.unwrap_or_else(|| String::) }
//         //         }
//         //     }
//         //     _ => {}
//         // }
//
//         Ident::new(&match &f.ident {
//             Some(ident) => format!("{}", ident),
//             None => format!("field_{}", i),
//         }, Span::call_site())
//     }).collect::<Vec<Ident>>()
// }

pub trait HasEnumParams {
    fn get_enum_params(&self) -> proc_macro2::TokenStream;
    fn get_enum_vals(&self) -> proc_macro2::TokenStream;
}

impl HasEnumParams for Variant {
    fn get_enum_params(&self) -> proc_macro2::TokenStream {
        match self.fields {
            Fields::Unit => quote! {},
            Fields::Unnamed(ref field_names) => {
                let idents = get_idents(&field_names.unnamed);
                quote! { ( #(ref #idents),* ) }
            },
            Fields::Named(ref field_names) => {
                let idents = get_idents(&field_names.named);
                quote! { { #(ref #idents),* } }
            }
        }
    }

    fn get_enum_vals(&self) -> proc_macro2::TokenStream {
        match self.fields {
            Fields::Unit => quote! {},
            Fields::Unnamed(ref field_names) => {
                let idents = get_idents(&field_names.unnamed);
                quote! { ( #(#idents),* ) }
            },
            Fields::Named(ref field_names) => {
                let idents = get_idents(&field_names.named);
                quote! { { #(#idents),* } }
            }
        }
    }
}

fn get_str_props(fields: &Punctuated<Field, Token![,]>) -> proc_macro2::TokenStream {
    let l = fields.len();
    let mut s = String::from(" \"[");
    let idents = get_idents(&fields);
    fields.iter()
        .enumerate()
        .for_each(|(i, _f)| {
            s.push_str(&format!("\"{{{}}}\"{}", i, if i < l-1 {","} else {"]\""} ));
        });
    quote! { format!(#s, #(&#idents),*) }
}

pub trait HasStrProperties {
    fn get_str_properties(&self) -> proc_macro2::TokenStream;
}

impl HasStrProperties for Variant {
    fn get_str_properties(&self) -> proc_macro2::TokenStream {
        match self.fields {
            Fields::Unit => quote! { "" },
            Fields::Unnamed(ref field_names) => {
                get_str_props(&field_names.unnamed)
            },
            Fields::Named(ref field_names) => {
                let mut s = String::from(" \"{{");
                let idents = get_idents(&field_names.named);
                let l = field_names.named.len();
                field_names.named.iter()
                    .enumerate()
                    .for_each(|(i, f)| {
                        s.push_str(&format!("\"{}\":{{{}}}{}", match &f.ident {
                            Some(ident) => format!("{}", ident),
                            None => String::new(),
                        }, i, if i < l-1 {","} else {"}}\""} ));
                    });
                quote! { format!(#s, #(&#idents),*) }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_quote() {
        let ident = Ident::new("param_1", Span::call_site());
        let t = quote! { match #ident {
            Some(m) => m,
            None => String::new(),
        } }.to_string();

        assert_eq!(t, "match param_1 { Some (m) => m , None => String :: new () , }");
    }

    #[test]
    fn test_get_str_props() {

    }
}

