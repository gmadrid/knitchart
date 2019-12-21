extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{quote};
use syn::{parse_macro_input, Attribute, Data, DeriveInput, Field, Ident, Type, Meta, NestedMeta, Lit};

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
    default_string: Option<String>
}

impl AttrMeta {
    fn meta_for_field(field: &Field) -> AttrMeta {
	let my_attr = field.attrs.iter().find(|a| {
	    if let Some(segment) = a.path.segments.first() {
		if segment.ident.to_string() == "attr" {
		    return true;
		}
	    }
	    false
	});

	let mut attr_meta = AttrMeta::default();
	my_attr.map(|attr| {
	    if let Ok(Meta::List(metalist)) = attr.parse_meta() {
		if let Some(NestedMeta::Meta(nested)) = metalist.nested.first() {
		    if let Meta::NameValue(name_value) = nested {
			if name_value.path.segments.first().map_or(false, |s| s.ident.to_string() == "default" ) {
			    if let Lit::Str(def) = &name_value.lit {
				attr_meta.default_string = Some(def.value());
			    }
			    // TODO: add better error checking.
			}
		    }
		}
	    }
	});
	attr_meta
    }
}

fn attr_attribute(field: &Field) -> Option<&Attribute> {
    field.attrs.iter().find(|a| {
	if let Some(segment) = a.path.segments.first() {
	    if segment.ident.to_string() == "attr" {
		return true;
	    }
	}
	false
    })
}

fn default_value(field: &Field) {
    let a = attr_attribute(field);
}

fn make_default_trait(struct_name: &Ident,
		      fields: &Vec<&Field>,
		      field_names: &Vec<&Ident>,
		      types: &Vec<&Type>,
) -> TokenStream2 {
    default_value(fields[1]);
    let meta_for_fields = fields.iter().map(|f| AttrMeta::meta_for_field(f)).collect::<Vec<_>>();

    let default_exprs = meta_for_fields.iter().map(|am| {
	am.default_string.as_ref().map_or_else(|| { quote!{ std::default::Default::default() } },
				     |val| quote! { #val.parse().unwrap() })
    }).collect::<Vec<_>>();
    
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

#[proc_macro_derive(Attributes, attributes(attr))]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let fields = match input.data {
	Data::Struct(ref data_struct) => {
	    data_struct.fields.iter().filter(|f| f.ident.is_some()).collect::<Vec<&Field>>()
	},
	// TODO: produce a better error message.
	_ => panic!("This is only for structs.")
    };
    let field_names = fields.iter().map(|f| f.ident.as_ref().unwrap()).collect::<Vec<&Ident>>();
    let types = fields.iter().map(|f| &f.ty).collect::<Vec<&Type>>();

    let default_trait = make_default_trait(&input.ident, &fields, &field_names, &types);

    let q = quote!{
	#default_trait
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
