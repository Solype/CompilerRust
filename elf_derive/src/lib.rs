use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data, Fields};

#[proc_macro_derive(BinaryLogicSize)]
pub fn derive_elf_write(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;
    let generics = input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let fields = match input.data {
        Data::Struct(data_struct) => match data_struct.fields {
            Fields::Named(fields_named) => fields_named.named,
            _ => panic!("BinaryLogicSize only supports named fields"),
        },
        _ => panic!("BinaryLogicSize only supports structs"),
    };

    // mem_len() → compile-time sizes
    let len_fields = fields.iter().map(|f| {
        let ty = &f.ty;
        quote! {
            total += std::mem::size_of::<#ty>();
        }
    });

    let expanded = quote! {
        impl #impl_generics #name #ty_generics #where_clause {
            pub fn mem_len() -> usize {
                let mut total = 0;
                #(#len_fields)*
                total
            }
        }
    };

    TokenStream::from(expanded)
}