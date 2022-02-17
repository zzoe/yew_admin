extern crate proc_macro;

use proc_macro::TokenStream;

use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput};

#[proc_macro_derive(FromScope)]
pub fn from_scope_derive(input: TokenStream) -> TokenStream {
    let mut impl_tokens = vec![];
    let parsed_input: DeriveInput = parse_macro_input!(input);
    let enum_name = parsed_input.ident;
    if let Data::Enum(s) = parsed_input.data {
        for variant in s.variants {
            let variant_name = variant.ident;
            impl_tokens.push(quote! {
                impl From<&Scope<#variant_name>> for #enum_name {
                    fn from(v: &Scope<#variant_name>) -> Self {
                        if let Some(s) = (v as &dyn Any).downcast_ref::<Scopen<#variant_name>>() {
                            #enum_name::#variant_name(s.clone())
                        } else {
                            panic!("{} 无法转换为AppScope", type_name::<#variant_name>())
                        }
                    }
                }
            });
        }
    } else {
        panic!("Only Enum is supported");
    }

    let tokens = quote! {
        use std::any::{type_name, Any};
        use std::convert::From;

        use yew::html::Scope;

        #(#impl_tokens)*
    };

    proc_macro::TokenStream::from(tokens)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
