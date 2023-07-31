use quote::quote_spanned;
use syn::parse_macro_input;

extern crate proc_macro;

#[proc_macro_derive(AutoGet, attributes(no_mut, exclude))]
pub fn derive(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(item as syn::DeriveInput);
    let name = ast.ident;

    let struct_ = match is_struct(ast, name) {
        Ok(value) => value,
        Err(err) => return err,
    };

    impl_autoget(&struct_)
}

fn is_struct(
    ast: syn::DeriveInput,
    name: syn::Ident,
) -> Result<syn::DataStruct, proc_macro::TokenStream> {
    let struct_: syn::DataStruct = match ast.data {
        syn::Data::Struct(data) => data,
        _ => {
            return Err(quote_spanned! {
                name.span() => compile_error!("AutoGet only works on structs")
            }
            .into())
        }
    };
    Ok(struct_)
}

fn impl_autoget(struct_: &syn::DataStruct) -> proc_macro::TokenStream {
    let exclude = struct_
        .fields
        .iter()
        .filter(|field| field.attrs.iter().any(|attr| attr.path.is_ident("exclude")))
        .map(|field| {
            let name = &field.ident;
            quote_spanned! {field.span() => #name}
        });
}
