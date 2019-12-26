extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Field, Fields};

fn get_named_fields_with_attr(input: &DeriveInput) -> std::result::Result<Vec<&Field>, String> {
    if let Data::Struct(data_struct) = &input.data {
        if let Fields::Named(fields_named) = &data_struct.fields {
            Ok(fields_named
                .named
                .iter()
                .filter(|f| {
                    f.attrs
                        .iter()
                        .find(|a| {
                            if let Some(segment) = a.path.segments.first() {
                                if segment.ident == "passthru" {
                                    return true;
                                }
                            }
                            return false;
                        })
                        .is_some()
                })
                .collect::<Vec<_>>())
        } else {
            Err("Must be a named struct".into())
        }
    } else {
        Err("Must be a struct".into())
    }
}

#[proc_macro_derive(PassThru, attributes(passthru))]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let type_name = &input.ident;
    let fields = get_named_fields_with_attr(&input).unwrap();

    /*
        Look at the type of the field. Then, for each named field in that type,
        create a func:

        impl TheOuterType {
            fn inner_field1(&self) -> field1_type { self.outer.inner; }
        }
    */
    let field_names = fields.iter().map(|f| {
        let name = f.ident.as_ref().unwrap();
        quote! { #name }
    });

    let q = quote! {
        impl #type_name {
            #(
                fn #field_names(&self) -> () {}
            )*
        }
    };

    q.into()
}
