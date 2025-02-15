use proc_macro2::TokenStream;
use quote::quote;
use std::ops::Deref;
use std::ops::DerefMut;
use syn::visit::Visit;
use syn::BinOp;
use syn::Expr;
use syn::ExprBinary;

#[derive(Debug)]
pub struct ReplaceExprAnd(Vec<Expr>);

impl Deref for ReplaceExprAnd {
	type Target = Vec<Expr>;
	fn deref(&self) -> &Self::Target { &self.0 }
}

impl DerefMut for ReplaceExprAnd {
	fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}

impl Visit<'_> for ReplaceExprAnd {
	fn visit_expr(&mut self, i: &Expr) {
		if let Expr::Binary(ExprBinary { op: BinOp::And(_), left, right, .. }) = i {
			self.visit_expr(left);
			self.visit_expr(right);
		} else {
			self.push(i.clone());
		}
	}
}

impl ReplaceExprAnd {
	pub fn new() -> Self { Self(Vec::new()) }

	pub fn expand(&self, then: TokenStream) -> TokenStream {
		self.iter().rev().fold(then, |then, expr| {
			quote! { if #expr { #then } }
		})
	}
}