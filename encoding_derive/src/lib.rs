#![recursion_limit="128"]
extern crate proc_macro2;
extern crate proc_macro;
extern crate syn;
extern crate quote;

use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{parse_macro_input, parse_quote, 
          DeriveInput, Data, GenericParam, Generics, 
          DataStruct, Fields, Index, DataEnum, Ident, Type};


#[proc_macro_derive(Encoding)]
pub fn derive_encoding(item : proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let name = input.ident;
    let generics = add_trait_bounds(input.generics);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let size_vector = match &input.data {
        Data::Enum(variants) => encoding_size_enum(variants),
        Data::Struct(data) => encoding_size_struct(data),
        Data::Union(_) => unimplemented!()

    };
    let encode = match &input.data {
        Data::Enum(variants) => encode_enum(&name, variants),
        Data::Struct(data) => encode_struct(&name, data),
        Data::Union(_) => unimplemented!()
    };
    let likelihood = match &input.data {
        Data::Enum(variants) => likelihood_enum(&name, variants),
        Data::Struct(data) => likelihood_struct(&name, data),
        Data::Union(_) => unimplemented!()
    };
    let expanded = quote! {
        impl #impl_generics ::encoding::Encoding for #name #ty_generics #where_clause {
            fn encoding_size() -> usize {
                #size_vector
            }
            fn encode_into(&self, target: &mut [f64]) {
                #encode
            }
            fn likelihood(&self, source: & [f64]) -> f64 {
                #likelihood
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

/** 
 * Take an ident as a head of a pattern and a DataStruct, adnd return a pattern together with a
 * list of variables and types.
 */

fn destruct<'a>(head : TokenStream, fields: &'a Fields) -> (TokenStream, Vec<(Ident, &'a Type)>) {
    match fields {
        Fields::Unit => (quote! { #head }, vec![]),
        Fields::Named(fields) => {
           let variables_types : Vec<(Ident, &Type)> = fields.named.iter().map(|f| {  
               match &f.ident {
                   None => unimplemented!(),
                   Some(id) => (id.clone(), &f.ty)
               }}).collect();
           let pattern = { 
                let variables = variables_types.iter().map(|(id, _)| { id });
                quote! { #head { #(#variables),* } }
           };
           (pattern, variables_types)
        }
        Fields::Unnamed(fields) => {
            let mut counter = 0;
            let variables_types : Vec<(Ident, &Type)> = fields.unnamed.iter().map(|f| {
                let id = Ident::new(&("x".to_owned() + &counter.to_string()), f.span());
                counter += 1;
                (id, &f.ty)     
            }).collect();
            let pattern = { 
                let variables = variables_types.iter().map(|(id, _)| { id });
                quote! { #head ( #(#variables),* ) }
           };
           (pattern, variables_types)
        }
    }
}

fn encode_struct(name : & Ident, data : &DataStruct) -> TokenStream { 
    let (pattern, variables) = destruct(quote! { #name }, &data.fields);
    let fields = encode_fields(variables);
    quote! {
        let #pattern = self;
        #fields
    }
}

fn encode_fields(fields : Vec<(Ident, &Type)>) -> TokenStream {
    let mut offset = quote! { 0 };
    let recurse = fields.iter().map(|(id, ty)| {
    let size = quote! {
                   <#ty as ::encoding::Encoding>::encoding_size()
             };
    let res  = quote! { 
                   #id.encode_into(&mut target[#offset..#offset + #size])
         };  
    offset = quote! { #offset + # size};
    res
    });
    quote! { #( #recurse );*  } }
    

fn encode_enum(name: &Ident, data : &DataEnum) -> TokenStream {
    let mut offset = quote! { 0 };
    let recurse = data.variants.iter().map(|v| {
        let size_fields = encoding_size_fields(&v.fields);
        let id = &v.ident;
        let (pattern, variables) = destruct(quote! { #name::#id }, &v.fields);
        let fields_code = encode_fields(variables);
        let rhs = quote! { 
            target[#offset] = 1.0;
            let target = &mut target[#offset + 1.. #offset + 1 + #size_fields];
            #fields_code
        };
        offset = quote! { #offset + 1 + #size_fields };
        quote! { #pattern => { #rhs } } 
    });
    quote! { match self { #( #recurse )*  } }
}

fn likelihood_struct(name : & Ident, data : &DataStruct) -> TokenStream { 
    let (pattern, variables) = destruct(quote! { #name }, &data.fields);
    let fields = likelihood_fields(variables);
    quote! {
        let #pattern = self;
        #fields
    }
}

fn likelihood_fields(fields : Vec<(Ident, &Type)>) -> TokenStream {
    let mut offset = quote! { 0 };
    let recurse = fields.iter().map(|(id, ty)| {
    let size = quote! {
                   <#ty as ::encoding::Encoding>::encoding_size()
             };
    let res  = quote! { 
                   #id.likelihood(&source[#offset..#offset + #size])
         };  
    offset = quote! { #offset + #size};
    res
    });
    quote! { 1.0  #( * #recurse)*  } 
    }
    

fn likelihood_enum(name: &Ident, data : &DataEnum) -> TokenStream {
    let mut offset = quote! { 0 };
    let recurse = data.variants.iter().map(|v| {
        let size_fields = encoding_size_fields(&v.fields);
        let id = &v.ident;
        let (pattern, variables) = destruct(quote! { #name::#id }, &v.fields);
        let fields_code = likelihood_fields(variables);
        let rhs = quote! { 
            let head = source[#offset]; 
            let source = &source[#offset + 1.. #offset + 1 + #size_fields];
            head * #fields_code 
        };
        offset = quote! { #offset + 1 + #size_fields };
        quote! { #pattern => { #rhs } } 
    });
    quote! { match self { #( #recurse )*  } }
}
fn add_trait_bounds(mut generics: Generics) -> Generics {
    for param in &mut generics.params {
        if let GenericParam::Type(ref mut type_param) = *param {
            type_param.bounds.push(parse_quote!(::encoding::Encoding));
        }
    }
    generics
}

#[proc_macro]
pub fn array_impl(n : proc_macro::TokenStream) -> proc_macro::TokenStream {
    let n = TokenStream::from(n);
    let res = quote!{
        impl <T: Encoding> Encoding for [T; #n] {
            fn encoding_size() -> usize {
                #n * <T as Encoding>::encoding_size()
            }
            fn encode_into(&self, target : &mut [f64]) {
                let size = <T as Encoding>::encoding_size();
                for i in 0..#n {
                    let target = &mut target[i * size .. i * size + size];
                    self[i].encode_into(target);
                }
            }

            fn likelihood(&self, source : & [f64]) -> f64 {
                let size = <T as Encoding>::encoding_size();
                let mut l = 1.0;
                for i in 0..#n {
                    let source = &source[i * size .. i * size + size];
                    l *= self[i].likelihood(source);
                };
                l
            }
        }
    };
    proc_macro::TokenStream::from(res)
}

