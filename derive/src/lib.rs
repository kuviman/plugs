use darling::{FromDeriveInput, FromField};
use proc_macro2::TokenStream;
use quote::quote;

#[proc_macro_derive(Bundle, attributes(bundle))]
pub fn derive_bundle(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input: syn::DeriveInput = syn::parse_macro_input!(input);
    match DeriveInput::from_derive_input(&input) {
        Ok(input) => input.derive().into(),
        Err(e) => e.write_errors().into(),
    }
}

#[derive(FromField)]
#[darling(attributes(bundle))]
struct Field {
    ident: Option<syn::Ident>,
    ty: syn::Type,
    #[darling(default)]
    other: bool,
}

#[derive(FromDeriveInput)]
struct DeriveInput {
    ident: syn::Ident,
    generics: syn::Generics,
    data: darling::ast::Data<(), Field>,
}

impl DeriveInput {
    pub fn derive(self) -> TokenStream {
        let plugs = quote! { plugs }; // TODO in deps or in geng::?, parse Cargo.toml
        let Self {
            ident,
            generics,
            data,
        } = self;
        let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
        let data = data.take_struct().unwrap();
        let mut plug_list = quote!(#plugs::frunk::hlist::HNil);
        let mut field_initializers = Vec::new();
        let mut field_refs = Vec::new();
        for (index, field) in data.iter().enumerate() {
            let field_ident = match &field.ident {
                Some(ident) => quote!(#ident),
                None => {
                    let index = syn::Index::from(index);
                    quote!(#index)
                }
            };
            let field_ty = &field.ty;
            if field.other {
                let other_plug_list = quote!(<#field_ty as #plugs::Bundle>::PlugList<'a>);
                plug_list = quote!(#plugs::frunk::hlist::HCons<#other_plug_list, #plug_list>);
                field_refs.push(
                    quote!(let refs = refs.prepend(#plugs::Bundle::refs(&self.#field_ident));),
                );
                plug_list = quote!(<#plug_list as std::ops::Add<#other_plug_list>>::Output);
                field_refs.push(
                    quote!(let refs = refs.extend(#plugs::Bundle::refs(&self.#field_ident));),
                );
                field_initializers.push(
                    quote!(#field_ident: <#field_ty as #plugs::Bundle>::from_refs(refs.pluck().0)),
                );
            } else {
                plug_list = quote!(#plugs::frunk::hlist::HCons<&'a #field_ty, #plug_list>);
                field_initializers
                    .push(quote!(#field_ident: (#plugs::Plug::clone(refs.get::<&#field_ty, _>()))));
                field_refs.push(quote!(let refs = refs.prepend(&self.#field_ident);));
            }
        }
        quote! {
            impl #impl_generics #plugs::Bundle for #ident #ty_generics #where_clause {
                type PlugList<'a> = #plug_list;
                fn from_refs(refs: Self::PlugList<'_>) -> Self {
                    Self {
                        #(#field_initializers,)*
                    }
                }
                fn refs(&self) -> Self::PlugList<'_> {
                    let refs = #plugs::frunk::hlist::HNil;
                    #(#field_refs)*
                    refs
                }
            }
        }
    }
}
