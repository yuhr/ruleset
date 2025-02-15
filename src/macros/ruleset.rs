use crate::Subrule;
use proc_macro2::TokenStream;
use quote::quote;
use quote::ToTokens;
use std::fmt::Display;
use syn::parse::Parse;
use syn::Token;

#[derive(Debug)]
pub struct Ruleset {
	rules: Vec<Subrule>,
	original: TokenStream,
}

impl Parse for Ruleset {
	fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
		let mut original = TokenStream::new();
		let mut rules = Vec::new();

		while !input.is_empty() {
			let rule = input.parse::<Subrule>()?;
			original.extend(rule.to_token_stream());
			rules.push(rule);
			let token = input.parse::<Token![;]>()?;
			original.extend(token.into_token_stream());
		}

		Ok(Self { rules, original })
	}
}

impl ToTokens for Ruleset {
	fn to_tokens(&self, tokens: &mut TokenStream) { tokens.extend(self.original.clone()); }
}

impl Display for Ruleset {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		Display::fmt(&self.to_token_stream().to_string(), f)
	}
}

impl Ruleset {
	pub fn expand(&self) -> TokenStream {
		let rules_expanded = self.rules.iter().map(Subrule::expand);
		quote! {
			ruleset::Ruleset::<_>::new([#(#rules_expanded),*])
		}
	}
}