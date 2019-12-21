extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Field, Ident, Lit, Meta, NestedMeta};

use proc_macro2::TokenStream as TokenStream2;

/*
 #[derive(Attributes)] will derive two things for the tagged struct:
 - the Default trait
 - a method, set_value(&mut self, name: &str, value: &str).

 Each field may be tagged with the 'attr' attribute which may contain
 the following sub-attributes:
   default = a string literal that will be parsed to set the default value of that field.
   parse = a method that will be called to parse the string into the field value.

 E.g.:

     #[derive(Attributes)]
     pub struct Attributes {

       rows: usize,  // default will be set with usize::default()

       #[attr(64)]
       cols: usize,

       #[attr('.', parse=parse_char_name)]
       knit_char: char,  // but uses special parser function

       #[attr("whitesmoke")]
       color: CssColor,  // still using s.parse()?
     }
*/

#[derive(Debug, Default)]
struct AttrMeta {
    default_string: Option<String>,
    parse_func: Option<Ident>,
}

impl AttrMeta {
    fn meta_for_field(field: &Field) -> AttrMeta {
        let my_attr = field.attrs.iter().find(|a| {
            if let Some(segment) = a.path.segments.first() {
                if segment.ident == "attr" {
                    return true;
                }
            }
            false
        });

        let mut attr_meta = AttrMeta::default();
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
    fields: &Vec<&Field>,
    field_names: &Vec<&Ident>,
) -> TokenStream2 {
    let meta_for_fields = fields
        .iter()
        .map(|f| AttrMeta::meta_for_field(f))
        .collect::<Vec<_>>();

    let default_exprs = meta_for_fields
        .iter()
        .map(|am| {
            am.default_string.as_ref().map_or_else(
                || {
                    quote! { std::default::Default::default() }
                },
                |val| {
                    if let Some(parse_func) = &am.parse_func {
                        quote! { #parse_func(#val) }
                    } else {
                        quote! { #val.parse().unwrap() }
                    }
                },
            )
        })
        .collect::<Vec<_>>();

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
    fields: &Vec<&Field>,
    field_names: &Vec<&Ident>,
) -> TokenStream2 {
    // TODO stop making this twice.
    let meta_for_fields = fields
        .iter()
        .map(|f| AttrMeta::meta_for_field(f))
        .collect::<Vec<_>>();

    let parse_exprs = meta_for_fields
        .iter()
        .map(|am| {
            if let Some(parse_func) = &am.parse_func {
                quote! { #parse_func(v) }
            } else {
                quote! { v.parse().unwrap() }
            }
        })
        .collect::<Vec<_>>();

    let if_statements = quote! {
        #(
        if std::stringify!(#field_names) == n {
            self.#field_names = #parse_exprs;
            return;
        }
        )*
    };

    let q = quote! {
    impl #struct_name {
        fn set_value(&mut self, n: &str, v: &str) {
        #if_statements
        }
    }
    };
    q.into()
}

#[proc_macro_derive(Attributes, attributes(attr))]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let fields = match input.data {
        Data::Struct(ref data_struct) => data_struct
            .fields
            .iter()
            .filter(|f| f.ident.is_some())
            .collect::<Vec<&Field>>(),
        // TODO: produce a better error message.
        _ => panic!("This is only for structs."),
    };
    let field_names = fields
        .iter()
        .map(|f| f.ident.as_ref().unwrap())
        .collect::<Vec<&Ident>>();
    let default_trait = make_default_trait(&input.ident, &fields, &field_names);

    let set_value_func = make_set_value_func(&input.ident, &fields, &field_names);

    let q = quote! {
    #default_trait

    #set_value_func
    };

    q.into()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
