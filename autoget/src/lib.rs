//! # autoget

//! A simple macro for generating getters for rust struct members.

//! ## Usage

//! ```rust
//! #[derive(AutoGet)]
//! struct Something {
//!     test: String,
//!     test2: String,
//!     test3: String,
//! }
//! ```

//! this will generate code that looks something like this:

//! ```rust
//! impl Something {
//!     fn test(&self) -> &String {
//!         &self.test
//!     }
//!     fn test_mut(&mut self) -> &mut String {
//!         &mut self.test
//!     }
//!     fn test2(&self) -> &String {
//!         &self.test2
//!     }
//!     fn test2_mut(&mut self) -> &mut String {
//!         &mut self.test2
//!     }
//!     fn test3(&self) -> &String {
//!         &self.test3
//!     }
//!     fn test3_mut(&mut self) -> &mut String {
//!         &mut self.test3
//!     }
//! }
//! ```

//! to disable mutable getters you can use `#[no_mut]` macro helper attributes on selected member structs.

//! ```rust
//! #[derive(AutoGet)]
//! struct Something {
//!     test: String,
//!     #[no_mut]
//!     test2: String,
//!     test3: String,
//! }
//! ```

//! or you can disable getters altogether by using `#[exclude]`

//! ```rust
//! #[derive(AutoGet)]
//! struct Something {
//!     test: String,
//!     #[exclude]
//!     test2: String,
//!     test3: String,
//! }
//! ```

//! you can use them alongside eachother such as:

//! ```rust
//! #[derive(autoget::AutoGet)]
//! struct Something {
//!     test: String,

//!     #[exclude]
//!     test2: String,

//!     #[no_mut]
//!     test3: String,
//! }
//! ```
use quote::quote_spanned;
use syn::{ parse_macro_input, Ident, Field };
use syn::spanned::Spanned;

extern crate proc_macro;

// Helper Attribute
const NO_MUT: &'static str = "no_mut";
const EXCLUDE: &'static str = "exclude";

#[proc_macro_derive(AutoGet, attributes(no_mut, exclude))]
pub fn derive(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(item as syn::DeriveInput);
    let name = ast.ident.clone();

    let struct_ = match is_struct(ast, name.clone()) {
        Ok(value) => value,
        Err(err) => {
            return err;
        }
    };

    let impls = match impl_autoget(&struct_, name) {
        Ok(value) => value,
        Err(err) => {
            return err;
        }
    };

    impls.into()
}

fn impl_autoget(
    struct_: &syn::DataStruct,
    struct_name: Ident
) -> Result<proc_macro2::TokenStream, proc_macro::TokenStream> {
    let mut fields = get_fields_ident_by_attribute(&struct_)?;

    code_gen(&mut fields);

    let fns = fields
        .into_iter()
        .filter(|f| f.code_gen.is_some())
        .map(|f| f.code_gen.unwrap())
        .collect::<Vec<_>>();

    let impls = quote::quote! {
        impl #struct_name {
            #(#fns)*
        }
    };

    Ok(impls)
}

fn is_struct(
    ast: syn::DeriveInput,
    name: syn::Ident
) -> Result<syn::DataStruct, proc_macro::TokenStream> {
    let struct_: syn::DataStruct = match ast.data {
        syn::Data::Struct(data) => data,
        _ => {
            return Err(
                (
                    quote_spanned! {
                name.span() => compile_error!("AutoGet only works on structs")
            }
                ).into()
            );
        }
    };
    Ok(struct_)
}

struct FieldInfo {
    inner: Field,
    no_mut: bool,
    exclude: bool,
    code_gen: Option<proc_macro2::TokenStream>,
}

impl FieldInfo {
    fn new(field: Field) -> Self {
        Self {
            no_mut: Self::infer_attribuets(&field, NO_MUT),
            exclude: Self::infer_attribuets(&field, EXCLUDE),
            inner: field,
            code_gen: None,
        }
    }

    fn is_double_attribute(&self) -> bool {
        self.no_mut && self.exclude
    }

    fn infer_attribuets(field: &Field, attribute_ident: &'static str) -> bool {
        field.attrs.iter().any(|attr| attr.path().is_ident(attribute_ident))
    }
}

fn info(field: Field) -> Result<FieldInfo, proc_macro::TokenStream> {
    let mut err = Vec::new();

    let field = FieldInfo::new(field);

    if field.is_double_attribute() {
        let _err =
            quote_spanned! {
                    field.inner.span() => compile_error!("Can't use 2 attributes at the same time!")
                };

        err.push(_err);
    }

    if !err.is_empty() {
        let err = quote::quote!(#(#err)*);
        return Err(err.into());
    }

    Ok(field)
}

fn get_fields_ident_by_attribute(
    struct_: &syn::DataStruct
) -> Result<Vec<FieldInfo>, proc_macro::TokenStream> {
    let fields = struct_.fields
        .clone()
        .into_iter()
        .map(info)
        .collect::<Vec<Result<FieldInfo, proc_macro::TokenStream>>>();

    let fields = fields.into_iter().collect::<Result<Vec<FieldInfo>, proc_macro::TokenStream>>()?;

    Ok(fields)
}

fn gen(field: &mut FieldInfo) {
    // for handling newtype structs
    let default_ident = Ident::new("inner", field.inner.span());

    let ident = field.inner.ident.clone().unwrap_or(default_ident.clone());
    let code_ref = match ident.eq(&default_ident) {
        true => quote::quote! { self.0 },
        false => quote::quote! { self.#ident },
    };
    let ty = field.inner.ty.clone();

    let mut code = Vec::new();

    if field.exclude {
        return;
    }

    let no_mut =
        quote::quote! {
        pub fn #ident(&self) -> &#ty {
            &#code_ref 
        }
    };

    code.push(no_mut);

    if !field.no_mut {
        let name = format!("{}_mut", ident.clone().to_string());
        let fn_ident = Ident::new(name.as_str(), ident.span());
        let _mut =
            quote::quote! {
            pub fn #fn_ident(&mut self) -> &mut #ty {
                &mut #code_ref 
            }
        };

        code.push(_mut);
    }

    let code = quote::quote!(#(#code)*);
    field.code_gen = Some(code);
}

fn code_gen(fields: &mut Vec<FieldInfo>) {
    fields.iter_mut().for_each(gen);
}
