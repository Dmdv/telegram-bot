mod params;
mod enum_case_to_str;

extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields};
use params::{HasEnumParams, HasStrProperties};

fn impl_command_macro(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    match &ast.data {
        Data::Enum(ref v) => {
            let variants = &v.variants;
            let mut to_str_arms: Vec<proc_macro2::TokenStream> = Vec::new();
            let mut from_str_arms: Vec<proc_macro2::TokenStream> = Vec::new();
            let mut prefixed_commands: Vec<String> = Vec::new();

            for variant in variants {
                let ident = &variant.ident;
                let enum_params = variant.get_enum_params();
                let str_params = variant.get_str_properties();
                let cmd_str = enum_case_to_str::to_command_str(&ident.to_string());
                to_str_arms.push(quote! { #name::#ident #enum_params => ::core::fmt::Display::fmt(&format!("{}{}", #cmd_str, #str_params), f) });

                let enum_vals = variant.get_enum_vals();
                prefixed_commands.push(cmd_str);
                // let l = quote! { () => #name::#ident #enum_vals };
                let s = String::from("test");
                match variant.fields {
                    Fields::Unit => {
                        from_str_arms.push(quote! { #name::#ident });
                    },
                    Fields::Unnamed(_) => {
                        let l = quote! {
                            {
                                println!("Params: >>>>>> {:#?}", params);

                                #name::#ident(params.to_owned())
                            }
                        };
                        println!("My tokens: >>>>>> {:#?}", l.to_string());
                        from_str_arms.push(l);
                    },
                    Fields::Named(_) => {},
                }

            }

            if to_str_arms.len() < variants.len() {
                to_str_arms.push(quote! { _ => panic!("Попытка привести к строке необработанную Enum ветку") });
            }

            (quote! {
                impl #impl_generics ::core::fmt::Display for #name #ty_generics #where_clause {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::result::Result<(), ::core::fmt::Error> {
                        match *self {
                            #(#to_str_arms),*
                        }
                    }
                }

                impl From<#name #ty_generics #where_clause> for String {
                    fn from(value: #name) -> Self {
                        format!("{}", value)
                    }
                }

                impl TryFrom<&str> for #name #ty_generics #where_clause {
                    type Error = String;

                    fn try_from(cmd_str: &str) -> Result<Self, Self::Error> {
                        let mut words = cmd_str.split(" ");

                        match words.next() {
                            None => ::std::result::Result::Err(String::from("EmptyString")),
                            Some(cmd_str) => {
                                let params = words.collect::<Vec<_>>().join(" ");

                                match cmd_str.to_owned().as_str() {
                                    #(
                                        #prefixed_commands => Ok(#from_str_arms),
                                    )*
                                    _ => ::std::result::Result::Err(String::from("NotFound")),
                                }
                            },
                        }
                    }
                }
            }).into()
        },
        // Data::Struct(_ds) => {
        //     (quote! {}).into()
        // },
        _ => panic!("This macro only supports enums."),
    }
}

/// # Examples
/// ```
/// use command::BotCommand;
///
/// #[derive(BotCommand, Debug, PartialEq)]
/// enum Command {
///     Start,
/// }
///
/// assert_eq!(String::from(Command::Start), "/start");
/// ```
#[proc_macro_derive(BotCommand)]
pub fn bot_command_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).expect("Parsing TokenStream error");

    // Build the trait implementation
    impl_command_macro(&ast)
}
