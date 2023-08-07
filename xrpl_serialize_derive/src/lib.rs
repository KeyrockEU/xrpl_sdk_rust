use proc_macro::TokenStream;
use proc_macro2::{Ident, Literal, Span};
use quote::{quote, quote_spanned, ToTokens};
use std::collections::HashMap;
use std::convert::Into;
use std::iter::Map;
use std::panic::panic_any;
use std::sync::{Once, OnceLock};
use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;
use syn::{
    parse_macro_input, parse_quote, Attribute, Data, DeriveInput, LitInt, LitStr, Meta, Path, Token,
};

// todo handle Option fields
// todo handle Vec fields
// todo handle BitFlags fields

#[derive(Default, Debug)]
struct StructAttrs {
    crate_path: Option<String>,
}

impl Parse for StructAttrs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        println!("{:#?}", input);
        let mut struct_args = StructAttrs::default();
        let ident: Ident = input.parse()?;
        if ident == "crate_path" {
            input.parse::<Token![=]>()?;
            let crate_path: LitStr = input.parse()?;
            struct_args.crate_path = Some(crate_path.value());
        } else {
            return Err(syn::Error::new(
                ident.span(),
                format!("Unknown xrpl_binary attribute: {}", ident),
            ));
        }
        Ok(struct_args)
    }
}

fn struct_attributes(attrs: &[Attribute]) -> syn::Result<StructAttrs> {
    for attr in attrs {
        if !attr.path().is_ident("xrpl_binary") {
            continue;
        }

        return attr.parse_args();
    }
    Ok(StructAttrs::default())
}

#[derive(Default, Debug)]
struct FieldAttrs {
    flatten: bool,
    name: Option<LitStr>,
}

impl Parse for FieldAttrs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        println!("{:#?}", input);
        let mut field_args = FieldAttrs::default();
        let ident: Ident = input.parse()?;
        if ident == "flatten" {
            field_args.flatten = true;
        } else if ident == "name" {
            input.parse::<Token![=]>()?;
            let name: LitStr = input.parse()?;
            field_args.name = Some(name);
        } else {
            return Err(syn::Error::new(
                ident.span(),
                format!("Unknown xrpl_binary attribute: {}", ident),
            ));
        }
        Ok(field_args)
    }
}

fn field_attributes(attrs: &[Attribute]) -> syn::Result<FieldAttrs> {
    for attr in attrs {
        if !attr.path().is_ident("xrpl_binary") {
            continue;
        }

        return attr.parse_args();
    }
    Ok(FieldAttrs::default())
}

#[proc_macro_derive(Serialize, attributes(xrpl_binary))]
pub fn derive_serialize(input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);

    let DeriveInput {
        ident, data, attrs, ..
    } = derive_input;

    let struct_attributes = match struct_attributes(&attrs) {
        Ok(struct_attributes) => struct_attributes,
        Err(err) => {
            let message = err.to_string();
            return quote_spanned! {
                err.span() =>
                compile_error!(#message);
            }
            .into();
        }
    };
    // println!("attrs: {:#?}", struct_attributes);

    struct SerializeField {
        serialize_method: Ident,
        field_code: u8,
        field_ident: Ident,
    }

    let fields = match data {
        Data::Struct(struct_data) => struct_data.fields,
        _ => {
            return quote_spanned! {
                Span::call_site() =>
                compile_error!("Serialize can only be derived for structs");
            }
            .into()
        }
    };

    let mut serialize_fields = Vec::new();

    let xrpl_types_path = Ident::new(
        struct_attributes
            .crate_path
            .as_ref()
            .map(|s| s.as_str())
            .unwrap_or("xrpl_types"),
        Span::call_site(),
    );

    for field in &fields {
        let Some(field_ident) = field.ident.as_ref() else {
            return quote_spanned! {
                field.span() =>
                compile_error!("Serialize can only be applied to structs with named fields");
            }
                .into();
        };

        let field_attributes = match field_attributes(&field.attrs) {
            Ok(field_attributes) => field_attributes,
            Err(err) => {
                let message = err.to_string();
                return quote_spanned! {
                    err.span() =>
                    compile_error!(#message);
                }
                .into();
            }
        };

        // println!("field type {}:\n{:#?}", field_ident, field.ty);

        let quote = if field_attributes.flatten {
            Some(quote_spanned!(field.span() =>
                #xrpl_types_path::serialize::Serialize::serialize(&self.#field_ident, serializer)?;
            ))
        } else if let Some(field_name) = field_attributes.name.as_ref() {
            let field_name_string = field_name.value();
            let Some(field_info) = field_info(&field_name_string) else {
                return quote_spanned! {
                    field_name.span() =>
                    compile_error!("Unknown field name");
                }.into()
            };

            let serialize_method =
                Ident::new(serialize_method(&field_info.field_type), field.span());
            let field_code = Literal::u8_unsuffixed(field_info.field_code);

            Some(quote_spanned!(field.span() =>
                #xrpl_types_path::serialize::Serializer::#serialize_method(
                    serializer,
                    #xrpl_types_path::serialize::FieldCode(#field_code),
                    self.#field_ident);
            ))
        } else {
            None
        };

        if let Some(quote) = quote {
            serialize_fields.push(quote);
        }
    }

    // println!("SER FIELDS: {:#?}", serialize_fields);

    let tokens = quote! {
        // use #xrpl_types_path as _xrpl_types;
        impl #xrpl_types_path::serialize::Serialize for #ident {
          fn serialize<S: #xrpl_types_path::serialize::Serializer>(&self, serializer: &mut S) -> std::result::Result<(), S::Error> {
             #(#serialize_fields)*
             Ok(())
          }
        }
    };
    tokens.into()
}

struct FieldInfo {
    field_type: String,
    field_code: u8,
}

static FIELD_INFO: OnceLock<HashMap<String, FieldInfo>> = OnceLock::new();

fn field_info(field_name: &str) -> Option<&FieldInfo> {
    FIELD_INFO
        .get_or_init(|| {
            let mut map = HashMap::new();
            map.insert(
                "LimitAmount".to_string(),
                FieldInfo {
                    field_type: "Amount".to_string(),
                    field_code: 3,
                },
            );
            map.insert(
                "QualityIn".to_string(),
                FieldInfo {
                    field_type: "UInt32".to_string(),
                    field_code: 20,
                },
            );
            map.insert(
                "TxnSignature".to_string(),
                FieldInfo {
                    field_type: "Blob".to_string(),
                    field_code: 4,
                },
            );
            map
        })
        .get(field_name)
}

fn serialize_method(field_type: &str) -> &'static str {
    match field_type {
        "UInt32" => "serialize_uint32",
        "Amount" => "serialize_amount",
        _ => panic!("Unknows field type {}", field_type),
    }
}
