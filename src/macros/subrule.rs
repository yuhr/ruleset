use crate::antecedent::Antecedent;
use crate::consequent::Consequent;
use proc_macro2::TokenStream;
use proc_macro_error::abort;
use quote::quote;
use quote::quote_spanned;
use quote::ToTokens;
use std::fmt::Display;
use syn::parse::Parse;
use syn::parse_str;
use syn::spanned::Spanned;
use syn::Attribute;
use syn::Token;

#[derive(Debug)]
pub struct Subrule {
	metadata: Option<TokenStream>,
	antecedents: Vec<Antecedent>,
	consequents: Vec<Consequent>,
	original: TokenStream,
}

impl Parse for Subrule {
	fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
		let mut original = TokenStream::new();

		let mut metadata = None;
		if let Ok(attributes) = input.fork().call(Attribute::parse_outer) {
			for attribute in attributes {
				original.extend(attribute.to_token_stream());
				match attribute.meta {
					syn::Meta::NameValue(syn::MetaNameValue { path, value, .. })
						if path == parse_str::<syn::Path>("metadata").unwrap() =>
					{
						metadata = Some(value.to_token_stream());
					}
					_ => {}
				}
			}
		}

		let mut antecedents = Vec::new();
		while let Ok(antecedent) = input.parse::<Antecedent>() {
			original.extend(antecedent.to_token_stream());
			antecedents.push(antecedent);
			if let Ok(token) = input.parse::<Token![,]>() {
				original.extend(token.into_token_stream());
				continue;
			} else {
				break;
			}
		}

		let arrow = input.parse::<Token![=>]>()?;
		original.extend(arrow.into_token_stream());

		let mut consequents = Vec::new();
		while let Ok(consequent) = input.parse::<Consequent>() {
			original.extend(consequent.to_token_stream());
			consequents.push(consequent);
			if let Ok(token) = input.parse::<Token![,]>() {
				original.extend(token.into_token_stream());
				continue;
			} else {
				break;
			}
		}

		Ok(Self { metadata, antecedents, consequents, original })
	}
}

impl ToTokens for Subrule {
	fn to_tokens(&self, tokens: &mut TokenStream) { tokens.extend(self.original.clone()); }
}

impl Display for Subrule {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		Display::fmt(&self.to_token_stream().to_string(), f)
	}
}

impl Subrule {
	pub fn is_context_free(&self) -> bool { self.consequents.len() == 1 }

	pub fn expand(&self) -> TokenStream {
		let is_context_free = self.is_context_free();
		let len_antecedents = self.antecedents.len();
		let len_consequents = if is_context_free {
			len_antecedents
		} else {
			self.consequents.iter().map(Consequent::len).sum()
		};

		if self.antecedents.is_empty() {
			abort!(self, "at least one antecedent is required");
		}
		if self.consequents.is_empty() {
			abort!(self, "at least one consequent is required");
		}
		if self.consequents.iter().all(Consequent::is_placeholder) {
			abort!(self, "at least one non-placeholder is required");
		}
		if len_antecedents != len_consequents {
			abort!(
				self,
				"consequence must have exactly one consequent or the same number of consequents as antecedents";
				note = "received: antecedents = {}, consequents = {}", len_antecedents, len_consequents;
				note = "expected: consequents == 1 || antecedents == consequents"
			);
		}

		let consequents = &self.consequents;
		let consequents_span = consequents
			.iter()
			.fold(TokenStream::new(), |mut tokens, consequent| {
				tokens.extend(consequent.to_token_stream());
				tokens
			})
			.span();
		let consequents_debug_text = quote!(#(#consequents),*).to_string();
		let length_override = if is_context_free { Some(len_antecedents) } else { None };
		let consequents_expanded =
			self.consequents.iter().map(|consequent| consequent.expand(length_override));
		let mut reductum = quote_spanned! {
			consequents_span =>
			ruleset::Reductum::from(
				ruleset::Consequence::new_with_debug_text(
					[#(#consequents_expanded),*],
					#consequents_debug_text,
				)
			)
		};

		let mut debug_text = quote!(#(#consequents),*).to_string();
		for remaining in (0..self.antecedents.len())
			.map(|index| self.antecedents.iter().enumerate().rev().skip(index).collect::<Vec<_>>())
		{
			let (i, antecedent) = remaining.first().unwrap();
			let input_name = format!("_{i}").parse::<TokenStream>().unwrap();
			debug_text = if *i == len_antecedents - 1 {
				format!("{antecedent} => {debug_text}")
			} else {
				format!("{antecedent}, {debug_text}")
			};
			let pat = antecedent.expand_pat();
			let hit = quote_spanned! {
				reductum.span() =>
				ruleset::Match::Hit { reductum: #reductum }
			};
			let miss = quote_spanned! {
				antecedent.span() =>
				ruleset::Match::Miss
			};
			let (then, otherwise) = if antecedent.is_negated() { (miss, hit) } else { (hit, miss) };
			let then = quote! { { return #then; } };
			let then = antecedent.guard().expand(then);
			let then = antecedent.replace_pat().expand(
				then,
				&remaining
					.iter()
					.map(|(_, antecedent)| antecedent.replace_pat())
					.collect::<Vec<_>>(),
			);
			let matcher = quote_spanned! {
				pat.span() =>
				move |#input_name| {
					#[allow(irrefutable_let_patterns)]
					if let &#pat = #input_name { #then }
					#otherwise
				}
			};
			reductum = quote_spanned! {
				matcher.span() =>
				ruleset::Reductum::from(
					ruleset::Subrule::new(#matcher, Some(#debug_text))
				)
			};
		}

		quote_spanned! {
			reductum.span() =>
			ruleset::Subrule::try_from(#reductum).unwrap()
		}
	}
}
