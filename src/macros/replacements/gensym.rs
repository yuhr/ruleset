use nanoid::nanoid;
use proc_macro2::Span;
use syn::Ident;

pub fn gensym(span: Span) -> Ident {
	const ALPHABET: [char; 16] =
		['1', '2', '3', '4', '5', '6', '7', '8', '9', '0', 'a', 'b', 'c', 'd', 'e', 'f'];
	Ident::new(format!("_{}", nanoid!(8, &ALPHABET)).as_str(), span)
}