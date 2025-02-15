use crate::replacements::*;
use proc_macro2::TokenStream;
use quote::quote_spanned;
use quote::ToTokens;
use std::fmt::Display;
use syn::parse::Parse;
use syn::spanned::Spanned;
use syn::visit_mut::VisitMut;
use syn::Expr;
use syn::Token;

#[derive(Debug)]
pub enum Consequent {
	Placeholder { original: TokenStream },
	Value { dots: usize, value: Expr, original: TokenStream },
}

impl Parse for Consequent {
	fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
		let mut original = TokenStream::new();

		if let Ok(token) = input.parse::<Token![_]>() {
			original.extend(token.into_token_stream());
			if let Ok(token) = input.parse::<Token![.]>() {
				original.extend(token.into_token_stream());
				let mut dots = 1;
				while let (Ok(token0), Ok(token1)) =
					(input.parse::<Token![_]>(), input.parse::<Token![.]>())
				{
					original.extend(token0.into_token_stream());
					original.extend(token1.into_token_stream());
					dots += 1;
				}
				let mut value = input.parse::<Expr>()?;
				original.extend(value.to_token_stream());
				ReplaceExprTry.visit_expr_mut(&mut value);
				Ok(Self::Value { dots, value, original })
			} else {
				Ok(Self::Placeholder { original })
			}
		} else {
			let mut value = input.parse::<Expr>()?;
			original.extend(value.to_token_stream());
			ReplaceExprTry.visit_expr_mut(&mut value);
			Ok(Self::Value { dots: 0, value, original })
		}
	}
}

impl ToTokens for Consequent {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		match self {
			Self::Placeholder { original } | Self::Value { original, .. } =>
				tokens.extend(original.clone()),
		}
	}
}

impl Display for Consequent {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		Display::fmt(&self.to_token_stream().to_string(), f)
	}
}

impl Consequent {
	pub fn is_placeholder(&self) -> bool { matches!(self, Self::Placeholder { .. }) }

	pub fn len(&self) -> usize {
		match self {
			Self::Placeholder { .. } => 1,
			Self::Value { dots, .. } => *dots + 1,
		}
	}

	pub fn expand(&self, width_override: Option<usize>) -> TokenStream {
		match self {
			Self::Placeholder { .. } => quote_spanned! {
				self.span() =>
				std::option::Option::None
			},
			Self::Value { dots, value, .. } => {
				let width = width_override.unwrap_or_else(|| *dots + 1);
				quote_spanned! {
					self.span() =>
					std::option::Option::Some(ruleset::Consequent::try_from((#width, #value)).unwrap())
				}
			}
		}
	}
}