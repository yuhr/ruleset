mod antecedent;
mod consequent;
mod replacements;
mod ruleset;
mod subrule;

use crate::ruleset::Ruleset;
use crate::subrule::Subrule;
use convert_case::Case;
use convert_case::Casing;
use in_definite::get_a_or_an;
use proc_macro::TokenStream;
use proc_macro_error::abort;
use proc_macro_error::proc_macro_error;
use quote::quote;
use syn::parse_macro_input;
use syn::parse_str;
use syn::Item;
use syn::ItemEnum;

#[proc_macro_error]
#[proc_macro]
pub fn ruleset(input: TokenStream) -> TokenStream {
	let ruleset = parse_macro_input!(input as Ruleset);
	ruleset.expand().into()
}

#[doc(hidden)]
#[proc_macro_error]
#[proc_macro_derive(AsRef)]
pub fn as_ref(input: TokenStream) -> TokenStream {
	let item = parse_macro_input!(input as Item);
	let (r#type, generics) = match item {
		Item::Enum(r#enum) => (r#enum.ident.clone(), r#enum.generics.clone()),
		Item::Struct(r#struct) => (r#struct.ident.clone(), r#struct.generics.clone()),
		_ => abort!(item, "only `struct`s and `enum`s are supported by `AsRef`"),
	};
	let (generics_impl, generics_type, where_clause) = generics.split_for_impl();
	quote! {
		impl #generics_impl std::convert::AsRef<#r#type #generics_type> for #r#type #generics_type #where_clause {
			fn as_ref(&self) -> &Self { self }
		}
	}
	.into()
}

#[doc(hidden)]
#[proc_macro_error]
#[proc_macro_derive(Enum)]
pub fn r#enum(input: TokenStream) -> TokenStream {
	let r#enum = parse_macro_input!(input as ItemEnum);
	let r#type = &r#enum.ident;
	let (generics_impl, generics_type, where_clause) = &r#enum.generics.split_for_impl();
	let impls = r#enum.variants.iter().map(|variant| {
		let ident = &variant.ident;
		if variant.fields.len() != 1 {
			abort!(variant.fields, "only 1-field variants are supported by `Enum`");
		}
		let field = variant.fields.iter().nth(0).unwrap();
		let name = &ident.to_string().to_case(Case::Lower);
		let article = get_a_or_an(name);

		let fn_name = parse_str::<syn::Ident>(&ident.to_string().to_case(Case::Snake)).unwrap();
		let doc = format!(" Returns a reference to the inner value if this is {article} {name}.");

		let fn_name_is =
			parse_str::<syn::Ident>(&format!("is_{}", &ident.to_string().to_case(Case::Snake)))
				.unwrap();
		let doc_is = format!(" Returns whether this is {article} {name}.");

		let fn_name_into =
			parse_str::<syn::Ident>(&format!("into_{}", &ident.to_string().to_case(Case::Snake)))
				.unwrap();
		let doc_into = format!(" Returns the inner value if this is {article} {name}.");

		quote! {
			impl #generics_impl #r#type #generics_type #where_clause {
				#[doc = #doc]
				pub fn #fn_name(&self) -> std::option::Option<&#field> {
					match self {
						Self::#ident(value) => std::option::Option::Some(value),
						_ => std::option::Option::None,
					}
				}
				#[doc = #doc_is]
				pub fn #fn_name_is(&self) -> bool {
					self.#fn_name().is_some()
				}
				#[doc = #doc_into]
				pub fn #fn_name_into(self) -> std::option::Option<#field> {
					match self {
						Self::#ident(value) => std::option::Option::Some(value),
						_ => std::option::Option::None,
					}
				}
			}
		}
	});
	quote! {
		#(#impls)*
	}
	.into()
}