use proc_macro::TokenStream;
use quote::*;
use syn::*;

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;
    let bident = format_ident!("{}Builder", ast.ident);
    let fields = if let syn::Data::Struct(syn::DataStruct {
        fields: syn::Fields::Named(syn::FieldsNamed { ref named, .. }),
        ..
    }) = ast.data
    {
        named
    } else {
        unimplemented!()
    };

    let optionized = fields.iter().map(|f| {
        let ident = &f.ident;
        let ty = &f.ty;
        quote! { #ident: std::option::Option<#ty> }
    });

    let methods = fields.iter().map(|f| {
        let ident = &f.ident;
        let ty = &f.ty;
        quote! {
            pub fn #ident(&mut self, #ident: #ty) -> &mut Self {
                self.#ident = Some(#ident);
                self
            }
        }
    });

    let build_fields = fields.iter().map(|f| {
        let ident = &f.ident;
        quote! {
            #ident: self.#ident.clone().ok_or(concat!(stringify!(#ident), " is not set"))
        }
    });

    quote! {
        pub struct #bident {
            #(#optionized,),*
        }

        impl #name {
            pub fn builder() -> #bident {
                #bident {
                    executable: None,
                    args: None,
                    env: None,
                    current_dir: None,
                }
            }
        }

        impl #bident {
            #(#methods)*
            pub fn build(&mut self) -> Result<#name, Box<dyn core::error::Error>> {
                Ok(#name {
                    #(#build_fields),*
                })
            }
        }
    }
    .into()
}
