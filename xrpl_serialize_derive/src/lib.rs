use proc_macro::TokenStream;
use proc_macro2::{Ident, Literal, Span};
use quote::{quote, quote_spanned};
use std::convert::Into;
use syn::spanned::Spanned;
use syn::{parse_macro_input, Data, DeriveInput, LitInt, parse_quote};

#[proc_macro_derive(Serialize, attributes(xrpl_binary))]
pub fn derive_serialize(input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);
    println!("SYN: {:#?}", derive_input); // todo allan remove

    let DeriveInput {
        ident,
        data,
        generics,
        ..
    } = derive_input;

    struct SerializeField {
        serialize_method: Ident,
        field_code: u8,
        field_ident: Ident,
    }

    let where_clause = &generics.where_clause;

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

    for field in &fields {
        if field.ident.is_none() {
            return quote_spanned! {
                field.span() =>
                compile_error!("Serialize can only be applied to structs with named fields");
            }
            .into();
        }
    }

    let serialize_fields: Vec<_> = fields
        .iter()
        .map(|field| {
            let field_ident = field.ident.as_ref().expect("not tuple");
            // let field_code = LitInt::new("1", field.span());
            // let field_code: LitInt = parse_quote!(1);
            let field_code = Literal::u8_unsuffixed(1);
            quote_spanned!(field.span() =>
            xrpl_types::serialize::Serializer::serialize_uint32(
                    serializer,
                    xrpl_types::serialize::FieldCode(#field_code),
                    self.#field_ident);
            )
        })
        .collect();

    println!("SER FIELDS: {:#?}", serialize_fields);

    let tokens = quote! {
        // todo allan handle Serialize fully qualified
        impl #generics xrpl_types::serialize::Serialize for #ident #generics #where_clause #where_clause {
          fn serialize<S: xrpl_types::serialize::Serializer>(&self, serializer: &mut S) -> std::result::Result<(), S::Error> {
             #(#serialize_fields)*
             Ok(())
          }
        }
    };
    tokens.into()
}
