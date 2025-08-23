use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, Expr, Lit, Meta, parse_macro_input};

#[proc_macro_derive(Model)]
pub fn model_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;

    // 默认表名 = struct 名字转小写
    let mut table_name = name.to_string().to_lowercase();

    // 检查 #[table_name = "xxx"]
    for attr in input.attrs {
        if attr.path().is_ident("table_name") {
            if let Meta::NameValue(nv) = &attr.meta {
                if let Expr::Lit(expr_lit) = &nv.value {
                    if let Lit::Str(litstr) = &expr_lit.lit {
                        table_name = litstr.value();
                    }
                }
            }
        }
    }

    let expanded = quote! {
        impl Model for #name {
            fn table_name() -> &'static str {
                #table_name
            }
        }
    };

    TokenStream::from(expanded)
}
