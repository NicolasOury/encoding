extern crate proc_macro2;
extern crate proc_macro;
extern crate syn;
extern crate quote;

use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{parse_macro_input, parse_quote, 
          DeriveInput, Data, GenericParam, Generics, 
          DataStruct, Fields, Index, DataEnum};


#[proc_macro_derive(Encoding)]
pub fn derive_encoding(item : proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let name = input.ident;
    let generics = add_trait_bounds(input.generics);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let size_vector = match &input.data {
        Data::Enum(ref variants) => encoding_size_enum(variants),
        Data::Struct(ref data) => encoding_size_struct(data),
        Data::Union(_) => unimplemented!()

    };
    let expanded = quote! {
        impl #impl_generics ::encoding::Encoding for #name #ty_generics #where_clause {
            fn encoding_size() -> usize {
                #size_vector
            }
        }
    };
    proc_macro::TokenStream::from(expanded)
}


fn encoding_size_struct(data : &DataStruct) -> TokenStream { 
    encoding_size_fields(&data.fields)
}

fn encoding_size_fields(fields : &Fields) -> TokenStream {
    match fields {
         Fields::Named(ref fields) => {
            let recurse = fields.named.iter().map(|f| {
                let _name = &f.ident;
                let ty = &f.ty;
                quote_spanned! { f.span()=> 
                   <#ty as ::encoding::Encoding>::encoding_size()
                }     
            });
            quote! { 0 #(+ #recurse)* }
         }
         Fields::Unnamed(ref fields) => {
            let recurse = fields.unnamed.iter().enumerate().map(|(i, f)| {
                let _index = Index::from(i);
                let ty = &f.ty;
                quote_spanned! { f.span()=> 
                   <#ty as ::encoding::Encoding>::encoding_size()
                }});
            quote! {
                0 #(+ #recurse)*
            }         
         }
         Fields::Unit => { quote!(0) }
}}

fn encoding_size_enum(data : &DataEnum) -> TokenStream {
    let recurse = data.variants.iter().map(|v| {
        let fields_code = encoding_size_fields(&v.fields);
        quote! { 
            1 + #fields_code
        }
    });
    quote! { 0 #(+ #recurse)* }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

fn add_trait_bounds(mut generics: Generics) -> Generics {
    for param in &mut generics.params {
        if let GenericParam::Type(ref mut type_param) = *param {
            type_param.bounds.push(parse_quote!(::encoding::Encoding));
        }
    }
    generics
}