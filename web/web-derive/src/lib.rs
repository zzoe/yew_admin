use proc_macro::TokenStream;

use quote::quote;
use syn::{parse_macro_input, AttributeArgs, Data, DeriveInput, Meta, NestedMeta};

/// Generate a CombinedApi.
///
/// # Example
///
/// ```ignore
/// #[CombinedApi(Apis)]
/// enum Api {
///     Api1,
///     Api2,
/// }
/// ```
///
///
/// 生成如下代码：
/// ```
/// pub(crate) struct Apis;
///
/// impl OpenApi for Apis {
///     fn meta() -> Vec<MetaApi> {
///         let mut metadata = Vec::new();
///         metadata.extend(Api1::meta());
///         metadata.extend(Api2::meta());
///         metadata
///     }
///
///     fn register(registry: &mut Registry) {
///         Api1::register(registry);
///         Api2::register(registry);
///     }
///
///     fn add_routes(self, route: Route) -> Route {
///         let route = Api1.add_routes(route);
///         let route = Api2.add_routes(route);
///         route
///     }
/// }
/// ```

#[proc_macro_attribute]
#[allow(non_snake_case)]
pub fn CombinedApi(args: TokenStream, input: TokenStream) -> TokenStream {
    let args: AttributeArgs = parse_macro_input!(args as AttributeArgs);

    if args.len() != 1 {
        return TokenStream::from(quote! {
            #[proc_macro_error]
            compile_error!("CombinedApi expects one argument");
        });
    }

    let struct_name = match &args[0] {
        // NestedMeta::Lit(Lit::Str(s)) => s.value(),
        NestedMeta::Meta(Meta::Path(path)) => match path.get_ident() {
            Some(ident) => ident,
            None => {
                return TokenStream::from(quote! {
                    #[proc_macro_error]
                    compile_error!("CombinedApi expects a struct name");
                });
            }
        },
        _ => {
            return TokenStream::from(quote! {
                #[proc_macro_error]
                compile_error!("CombinedApi expects a struct name");
            });
        }
    };

    let mut meta_tokens = vec![];
    let mut registry_tokens = vec![];
    let mut route_tokens = vec![];
    let parsed_input: DeriveInput = parse_macro_input!(input);
    if let Data::Enum(data_enum) = parsed_input.clone().data {
        for variant in data_enum.variants {
            let variant_name = variant.ident;
            meta_tokens.push(quote! {
                metadata.extend(#variant_name::meta());
            });
            registry_tokens.push(quote! {
                #variant_name::register(registry);
            });
            route_tokens.push(quote! {
                let route = #variant_name.add_routes(route);
            });
        }
    } else {
        return TokenStream::from(quote! {
            #[proc_macro_error]
            compile_error!("CombinedApi can only be used on enums");
        });
    }

    let token = quote! {
        #parsed_input

        pub(crate) struct #struct_name;

        impl OpenApi for #struct_name {
            fn meta() -> Vec<MetaApi> {
                let mut metadata = Vec::new();
                #(#meta_tokens)*
                metadata
            }

            fn register(registry: &mut Registry) {
                #(#registry_tokens)*
            }

            fn add_routes(self, route: Route) -> Route {
                #(#route_tokens)*
                route
            }
        }
    };

    token.into()
}
