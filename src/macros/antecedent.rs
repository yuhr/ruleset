use crate::replacements::*;
use proc_macro2::TokenStream;
use quote::quote;
use quote::ToTokens;
use std::fmt::Display;
use syn::parse::Parse;
use syn::visit::Visit;
use syn::visit_mut::VisitMut;
use syn::Expr;
use syn::Pat;
use syn::Token;

#[derive(Debug)]
pub struct Antecedent {
	negated: bool,
	pattern: Pat,
	replace_pat: ReplacePat,
	guard: ReplaceExprAnd,
	original: TokenStream,
}

impl Parse for Antecedent {
	fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
		let mut original = TokenStream::new();

		let negated = if let Ok(token) = input.parse::<Token![!]>() {
			original.extend(token.into_token_stream());
			true
		} else {
			false
		};

		let mut pattern = Pat::parse_multi(input)?;
		original.extend(pattern.to_token_stream());
		let mut replace_pat = ReplacePat::new();
		replace_pat.visit_pat_mut(&mut pattern);
		let mut guard = ReplaceExprAnd::new();
		if let Ok(token) = input.parse::<Token![if]>() {
			original.extend(token.into_token_stream());
			let mut expr = input.parse::<Expr>()?;
			// replace_pat.visit_expr_mut(&mut expr);
			original.extend(expr.to_token_stream());
			ReplaceExprTry.visit_expr_mut(&mut expr);
			guard.visit_expr(&expr);
		};

		Ok(Self { negated, pattern, replace_pat, guard, original })
	}
}

impl ToTokens for Antecedent {
	fn to_tokens(&self, tokens: &mut TokenStream) { tokens.extend(self.original.clone()) }
}

impl Display for Antecedent {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		Display::fmt(&self.to_token_stream().to_string(), f)
	}
}

impl Antecedent {
	pub fn replace_pat(&self) -> &ReplacePat { &self.replace_pat }
	pub fn guard(&self) -> &ReplaceExprAnd { &self.guard }
	pub fn is_negated(&self) -> bool { self.negated }

	pub fn expand_pat(&self) -> TokenStream {
		let pattern = &self.pattern;
		quote! { #pattern }
	}
}
