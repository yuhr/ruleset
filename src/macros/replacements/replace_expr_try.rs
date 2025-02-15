use syn::parse_quote;
use syn::visit_mut::visit_expr_mut;
use syn::visit_mut::VisitMut;
use syn::Expr;

pub struct ReplaceExprTry;

impl VisitMut for ReplaceExprTry {
	fn visit_expr_mut(&mut self, i: &mut Expr) {
		if let Expr::Try(expr) = i {
			let expr = &expr.expr;
			*i = parse_quote!(ruleset::try_!(#expr));
		}

		visit_expr_mut(self, i);
	}
}