use super::*;
use proc_macro2::TokenStream;
use proc_macro_error::emit_error;
use quote::quote;
use quote::quote_spanned;
use std::ops::Deref;
use std::ops::DerefMut;
use syn::parse_quote_spanned;
use syn::spanned::Spanned;
use syn::visit_mut::visit_pat_mut;
use syn::visit_mut::VisitMut;
use syn::Ident;
use syn::Pat;
use syn::PatIdent;
use syn::PatOr;
use syn::PatReference;

#[derive(Debug)]
pub struct ReplacePat(Vec<ReplacePatItem>);

#[derive(Debug)]
pub enum ReplacePatItem {
	Reference(Ident, Pat),
	Or(Ident, Vec<(Pat, ReplacePat)>),
	Ident(Ident, Ident),
}

impl Deref for ReplacePat {
	type Target = Vec<ReplacePatItem>;
	fn deref(&self) -> &Self::Target { &self.0 }
}

impl DerefMut for ReplacePat {
	fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}

impl VisitMut for ReplacePat {
	fn visit_pat_mut(&mut self, i: &mut Pat) {
		match i {
			Pat::Reference(PatReference { pat, .. }) => {
				let sym = gensym(pat.span());
				let mut pat = *pat.clone();
				self.visit_pat_mut(&mut pat);
				*i = parse_quote_spanned!(pat.span() => ref #sym);
				self.push(ReplacePatItem::Reference(sym, pat));
			}
			Pat::Or(PatOr { cases, .. }) => {
				let mut replacements = Vec::new();
				let sym = gensym(cases.span());
				for pat in cases.iter() {
					let mut pat = pat.clone();
					let mut replacement = ReplacePat::new();
					replacement.visit_pat_mut(&mut pat);
					replacements.push((pat, replacement));
				}
				*i = parse_quote_spanned!(cases.span() => ref #sym);
				self.push(ReplacePatItem::Or(sym, replacements));
			}
			Pat::Ident(PatIdent { ident, mutability, .. }) => {
				if mutability.is_some() {
					emit_error!(
						mutability,
						"captured variables in antecedent patterns can't be mutable"
					)
				}
				if !ident.to_string().chars().any(|char| char.is_ascii_uppercase()) {
					let sym = gensym(ident.span());
					let ident = ident.clone();
					*i = parse_quote_spanned!(ident.span() => #sym);
					self.push(ReplacePatItem::Ident(sym, ident));
				}
			}
			_ => {}
		}

		visit_pat_mut(self, i);
	}
}

impl ReplacePat {
	pub fn new() -> Self { Self(Vec::new()) }

	pub fn expand(&self, then: TokenStream, context: &[&ReplacePat]) -> TokenStream {
		self.iter().fold(then, |then, item| {
			use ReplacePatItem::*;
			match item {
				Reference(sym, pat) => {
					let pat = if let Pat::Lit(_) = pat {
						quote! { #pat }
					} else {
						quote_spanned! { pat.span() => &#pat }
					};
					let expr =
						quote_spanned! { pat.span() => let #pat = std::ops::Deref::deref(#sym) };
					quote! {
						if #expr { #then }
					}
				}
				Or(sym, replacements) => {
					let arms = replacements.iter().map(|(pat, replacement)| {
						let then = replacement.expand(then.clone(), context);
						quote_spanned! { pat.span() => &#pat => #then }
					});
					quote! { match #sym { #(#arms,)* _ => {} } }
				}
				Ident(sym, ident) => {
					let sym_target = context.iter().find_map(|replace_pat| {
						replace_pat.iter().find_map(|item| match item {
							Ident(sym_target, ident_target)
								if ident_target == ident && sym_target != sym =>
								Some(sym_target),
							_ => None,
						})
					});
					if let Some(sym_target) = sym_target {
						quote! { if #sym == #sym_target { #then } }
					} else {
						quote! { { #[allow(unused_variables)] let #ident = #sym; #then } }
					}
				}
			}
		})
	}
}