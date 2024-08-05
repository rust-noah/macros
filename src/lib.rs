mod enum_from;

use enum_from::process_enum_from;
use proc_macro::TokenStream;

/// for enum, we'd like to generate From impls for each variant
#[proc_macro_derive(EnumFrom)]
pub fn derive_enum_from(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    println!("{:#?}", input);

    process_enum_from(input).into()
}
