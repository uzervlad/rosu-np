use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{bracketed, parse::{Parse, ParseStream}, parse_macro_input, Ident, LitStr, Token, Type};

struct StructUpdateCondition(Ident);

impl ToTokens for StructUpdateCondition {
  fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
    let name = &self.0;

    tokens.extend(quote! {
      if let Some(value) = new_data.#name {
        self.#name = value;
      }
    });
  }
}

struct ParsedStructMember {
  name: Ident,
  ty: Type,
}

impl ToTokens for ParsedStructMember {
  fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
    let name = &self.name;
    let ty = &self.ty;

    tokens.extend(quote! {
      pub #name: #ty,
    });
  }
}

struct ParsedPartialStructMember {
  name: Ident,
  ty: Type,
  serde_name: LitStr,
}

impl ToTokens for ParsedPartialStructMember {
  fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
    let name = &self.name;
    let ty = &self.ty;
    let serde_name = &self.serde_name;

    tokens.extend(quote! {
      #[serde(rename = #serde_name)]
      pub #name: Option<#ty>,
    });
  }
}

struct ParsedStruct {
  name: Ident,
  members: Vec<ParsedStructMember>,
  partial_members: Vec<ParsedPartialStructMember>,
}

impl Parse for ParsedStruct {
  fn parse(input: ParseStream) -> syn::Result<Self> {
    let struct_name = input.parse::<Ident>()?;

    let mut members = vec![];
    let mut partial_members = vec![];

    input.parse::<Token![,]>()?;

    while !input.is_empty() {
      let name = match input.parse::<Ident>() {
        Ok(name) => name,
        _ => break,
      };

      input.parse::<Token![:]>()?;

      let ty = match input.parse::<Type>() {
        Ok(ty) => ty,
        _ => panic!("Expected a type"),
      };

      let serde_name_buffer;
      let _bracket = bracketed!(serde_name_buffer in input);

      let serde_name = match serde_name_buffer.parse::<LitStr>() {
        Ok(serde_name) => serde_name,
        _ => panic!("Expected a string literal"),
      };

      members.push(ParsedStructMember { name: name.clone(), ty: ty.clone(), });
      partial_members.push(ParsedPartialStructMember { name, ty, serde_name });

      if let Err(_) = input.parse::<Token![,]>() {
        break
      };
    }

    Ok(ParsedStruct {
      name: struct_name,
      members,
      partial_members,
    })
  }
}

#[proc_macro]
pub fn partial_struct(item: TokenStream) -> TokenStream {
  let input = parse_macro_input!(item as ParsedStruct);

  let name = input.name;
  let partial_name = syn::parse_str::<Ident>(format!("Partial{}", name).as_str()).unwrap();
  let members = input.members;
  let partial_members = input.partial_members;
  let keys = partial_members.iter().map(|m| &m.serde_name).collect::<Vec<_>>();
  let updates = partial_members.iter().map(|m| StructUpdateCondition(m.name.clone())).collect::<Vec<_>>();

  quote! {
    #[derive(Debug, Default)]
    pub struct #name {
      #(#members)*
    }

    #[derive(Deserialize)]
    pub struct #partial_name {
      #(#partial_members)*
    }

    impl #name {
      pub fn update(&mut self, new_data: #partial_name) {
        #(#updates)*
      }

      pub fn get_keys() -> Vec<&'static str> {
        vec![#(#keys),*]
      }
    }
  }.into()
}