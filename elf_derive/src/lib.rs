use proc_macro::TokenStream;
use quote::{quote, format_ident};
use syn::{parse_macro_input, DeriveInput, Data, Fields};

#[proc_macro_derive(GettersSetters)]
pub fn derive_getters_setters(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;

    let fields = match input.data {
        Data::Struct(data) => match data.fields {
            Fields::Named(fields) => fields.named,
            _ => panic!("Only named fields supported"),
        },
        _ => panic!("Only structs supported"),
    };

    let methods = fields.iter().map(|f| {
        let field_name = f.ident.as_ref().unwrap();
        let field_type = &f.ty;

        let getter_name = format_ident!("get_{}", field_name);
        let setter_name = format_ident!("set_{}", field_name);

        quote! {
            pub fn #getter_name(&self) -> &#field_type {
                &self.#field_name
            }

            pub fn #setter_name(&mut self, value: #field_type) {
                self.#field_name = value;
            }
        }
    });

    let expanded = quote! {
        impl #name {
            #(#methods)*
        }
    };

    TokenStream::from(expanded)
}


#[proc_macro_derive(ElfWrite)]
pub fn derive_elf_write(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;

    let fields = match input.data {
        Data::Struct(data_struct) => match data_struct.fields {
            Fields::Named(fields_named) => fields_named.named,
            Fields::Unnamed(_) | Fields::Unit => {
                panic!("ElfWrite only supports named fields")
            }
        },
        _ => panic!("ElfWrite only supports structs"),
    };

    // génère: self.field.write(file)?;
    let write_fields = fields.iter().map(|f| {
        let name = &f.ident;
        quote! {
            println!("{:?}", self.#name);
            self.#name.write(file)?;
        }
    });

    let expanded = quote! {
        impl ElfWritable for #name {
            fn write(&self, file: &mut std::fs::File) -> std::io::Result<()> {
                use crate::ElfWritable;
                #(#write_fields)*
                Ok(())
            }
        }
    };

    TokenStream::from(expanded)
}
