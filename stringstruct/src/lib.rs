extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Field, Ident, Lit, Meta, NestedMeta};

use proc_macro2::TokenStream as TokenStream2;

/*
 #[derive(StringStruct)] will derive two things for the tagged struct:
 - the Default trait
 - a method, set_value(&mut self, name: &str, value: &str) -> Result<()>.

 Each field may be tagged with the 'ssfield' attribute which may contain
 the following sub-attributes:
   default = a string literal that will be parsed to set the default value of that field.
   parse = a method that will be called to parse the string into the field value.

 E.g.:
     #[derive(StringStruct)]
     pub struct Attributes {

       rows: usize,      // default will be set with usize::default()

       #[ssfield(default = "64")]
       cols: usize,      // default will be set with "64".parse()

       #[ssfield(default = ".", parse="parse_char_name")]
       knit_char: char,  // default will be set with parse_char_name(".")

       #[ssfield(default = "whitesmoke")]
       color: CssColor,  // still using "whitesmoke".parse()?
     }
*/

#[derive(Debug)]
struct FieldMeta<'a> {
    field_name: &'a Ident,
    default_string: Option<String>,
    parse_func: Option<Ident>,
}

impl<'a> FieldMeta<'a> {
    fn meta_for_field(field: &Field) -> FieldMeta {
        let my_attr = field.attrs.iter().find(|a| {
            if let Some(segment) = a.path.segments.first() {
                if segment.ident == "ssfield" {
                    return true;
                }
            }
            false
        });

        let mut attr_meta = FieldMeta {
            field_name: field.ident.as_ref().unwrap(),
            default_string: None,
            parse_func: None,
        };
        my_attr.map(|attr| {
            if let Ok(Meta::List(metalist)) = attr.parse_meta() {
                for nested_meta in &metalist.nested {
                    if let NestedMeta::Meta(meta) = nested_meta {
                        if let Meta::NameValue(name_value) = meta {
                            match name_value.path.segments.first().map(|s| &s.ident) {
                                Some(i) if i == "default" => {
                                    if let Lit::Str(def) = &name_value.lit {
                                        attr_meta.default_string = Some(def.value())
                                    } else {
                                        unreachable!();
                                    }
                                }
                                Some(i) if i == "parse" => {
                                    if let Lit::Str(parse) = &name_value.lit {
                                        let parse_ident = Ident::new(&parse.value(), parse.span());
                                        attr_meta.parse_func = Some(parse_ident);
                                    } else {
                                        unreachable!()
                                    }
                                }
                                // Do you want to report unknown attrs?
                                _ => { /* no-op */ }
                            }
                        }
                    }
                }
            }
        });
        attr_meta
    }
}

fn make_default_trait(
    struct_name: &Ident,
    field_meta: &Vec<FieldMeta>,
) -> TokenStream2 {
    let field_names = field_meta.iter().map(|fm| fm.field_name);
    let default_exprs = field_meta
        .iter()
        .map(|am| {
            am.default_string.as_ref().map_or_else(
                || {
                    quote! { std::default::Default::default() }
                },
                |val| {
                    if let Some(parse_func) = &am.parse_func {
                        quote! { #parse_func(#val).unwrap() }
                    } else {
                        quote! { #val.parse().unwrap() }
                    }
                },
            )
        });

    let q = quote! {
    impl Default for #struct_name {
        fn default() -> #struct_name {
        #struct_name {
            #(#field_names: #default_exprs),*
        }
        }
    }
    };
    q
}

fn make_set_value_func(
    struct_name: &Ident,
    field_meta: &Vec<FieldMeta>,
) -> TokenStream2 {
    let field_names = field_meta.iter().map(|fm| fm.field_name);
    let parse_exprs = field_meta
        .iter()
        .map(|am| {
            if let Some(parse_func) = &am.parse_func {
                quote! { #parse_func(v).unwrap() }
            } else {
                quote! { v.parse().unwrap() }
            }
        });

    let q = quote! {
    impl #struct_name {
        fn set_value(&mut self, n: &str, v: &str) {
        #(
            if std::stringify!(#field_names) == n {
                self.#field_names = #parse_exprs;
                return;
            }
        )*
        }
    }
    };
    q.into()
}

fn meta_for_fields(input: &DeriveInput) -> Vec<FieldMeta> {
    match input.data {
        Data::Struct(ref data_struct) => data_struct
            .fields
            .iter()
            // Filter out any fields without names.
            .filter(|f| f.ident.is_some())
            .map(|f| FieldMeta::meta_for_field(f))
            .collect::<Vec<_>>(),
        // TODO: improve this error message.
        _ => panic!("This is only for structs."),
    }
}

#[proc_macro_derive(StringStruct, attributes(ssfield))]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let field_meta = meta_for_fields(&input);

    let default_trait = make_default_trait(&input.ident, &field_meta);
    let set_value_func = make_set_value_func(&input.ident, &field_meta);

    let q = quote! {
        #default_trait
        #set_value_func
    };

    q.into()
}
