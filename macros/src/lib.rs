extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use chrono::prelude::*;

fn get_current_day() -> u32 {
    let t = Utc::now();
    let tz_offset = FixedOffset::west_opt(5 * 3600).unwrap();
    tz_offset.from_utc_datetime(&t.naive_utc()).day()
}

#[proc_macro]
pub fn today(_input: TokenStream) -> TokenStream {
    let current_day = syn::Ident::new(format!("Day{}", get_current_day()).as_str(), proc_macro2::Span::call_site());
    quote! {
        solution::#current_day
    }.into()
}
