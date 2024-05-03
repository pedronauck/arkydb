use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{ParseStream, Parser, Result},
    parse_macro_input, Ident, ItemStruct,
};

fn parse_idents(input: ParseStream) -> Result<Ident> {
    Ok(input.parse()?)
}

fn str_to_path(input: &str) -> syn::Result<syn::Path> {
    syn::parse_str::<syn::Path>(input)
}

#[proc_macro_attribute]
pub fn schema(args: TokenStream, input: TokenStream) -> TokenStream {
    let item_struct = parse_macro_input!(input as ItemStruct);
    let trait_name = parse_idents.parse(args).unwrap();

    match &trait_name.to_string() as &str {
        "Node" => impl_schema_for_node(&item_struct),
        "EdgeData" => impl_schema_for_edge_data(&item_struct),
        _ => syn::Error::new_spanned(
            &trait_name,
            "The trait name is not supported by the `schema` macro",
        )
        .to_compile_error()
        .into(),
    }
}

fn impl_schema_for_node(item_struct: &ItemStruct) -> TokenStream {
    let entity_name = &item_struct.ident;
    let id_field_present = item_struct.fields.iter().any(|field| {
        field
            .ident
            .as_ref()
            .map(|ident| ident == "id")
            .unwrap_or(false)
    });

    if !id_field_present {
        return syn::Error::new_spanned(
            &item_struct,
            "The `id: NodeID` field is missing in the struct",
        )
        .to_compile_error()
        .into();
    }

    let types = str_to_path("arkycore::types").unwrap();
    let node_id = str_to_path("arkycore::types::NodeID").unwrap();
    let format_entity = str_to_path("arkycore::utils::format_entity").unwrap();

    return quote! {
        #[derive(Debug, Clone, PartialEq, #types::Serialize, #types::Deserialize)]
        #[allow(dead_code)]
        #item_struct

        impl Node for #entity_name {
            fn key(&self) -> #node_id {
                self.id
            }
            fn entity(&self) -> String {
                #format_entity(stringify!(#entity_name))
            }
        }
    }
    .into();
}

fn impl_schema_for_edge_data(item_struct: &ItemStruct) -> TokenStream {
    let entity_name = &item_struct.ident;
    let types = str_to_path("arkycore::types").unwrap();
    let edge_data = str_to_path("arky::edge::Data").unwrap();
    let edge_error = str_to_path("arky::edge::EdgeError").unwrap();

    return quote! {
        #[derive(Debug, Clone, PartialEq, #types::Serialize, #types::Deserialize)]
        #[allow(dead_code)]
        #item_struct

        impl #entity_name {
            fn new(data: #entity_name) -> #edge_data {
                #edge_data::new::<#entity_name>(data)
            }
            fn get(data: &#edge_data) -> Result<#entity_name, #edge_error> {
                match data.get::<#entity_name>() {
                    Ok(data) => Ok(data.clone()),
                    _ => Err(#edge_error::EdgeDataMismatch {
                        data_type: stringify!(#entity_name).to_string()
                    }),
                }
            }
        }
    }
    .into();
}
