extern crate proc_macro;
use std::fs;

use chrono::prelude::*;
use proc_macro::TokenStream;
use proc_macro2::{Ident, Punct};
use quote::{format_ident, quote};
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, ItemStruct,
};

fn get_current_day() -> u32 {
    let t = Utc::now();
    let tz_offset = FixedOffset::west_opt(5 * 3600).unwrap();
    tz_offset.from_utc_datetime(&t.naive_utc()).day()
}

#[proc_macro]
pub fn today(_input: TokenStream) -> TokenStream {
    let current_day = syn::Ident::new(
        format!("Day{}", get_current_day()).as_str(),
        proc_macro2::Span::call_site(),
    );
    quote! {
        solution::#current_day
    }
    .into()
}

#[proc_macro]
pub fn today_result(_input: TokenStream) -> TokenStream {
    let current_day_p1 = format_ident!(
        "{}P1",
        syn::Ident::new(
            format!("Day{}", get_current_day()).as_str(),
            proc_macro2::Span::call_site()
        )
    );
    let current_day_p2 = format_ident!(
        "{}P2",
        syn::Ident::new(
            format!("Day{}", get_current_day()).as_str(),
            proc_macro2::Span::call_site()
        )
    );
    quote! {
        (solution::#current_day_p1, solution::#current_day_p2)
    }
    .into()
}

#[proc_macro]
pub fn import_all_days(_input: TokenStream) -> TokenStream {
    let all_imports = fs::read_dir("./src/solution")
        .unwrap()
        .filter(|file| {
            file.as_ref().unwrap().file_name().into_string().unwrap()[..3] == String::from("day")
        })
        .map(|file| {
            let ident = format_ident!(
                "{}",
                file.unwrap()
                    .file_name()
                    .into_string()
                    .unwrap()
                    .split(".")
                    .next()
                    .unwrap()
            );
            quote! {
                pub mod #ident;
                pub use #ident::*;
            }
        })
        .collect::<Vec<_>>();
    quote! {
        #(#all_imports)*
    }
    .into()
}

struct ReturnTypeAttributes {
    p1: Ident,
    p2: Ident,
}

impl Parse for ReturnTypeAttributes {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut ret = ReturnTypeAttributes {
            p1: Ident::new("hi", proc_macro2::Span::call_site()),
            p2: Ident::new("hi", proc_macro2::Span::call_site()),
        };
        let mut attr_name = input.parse::<Ident>()?;
        input.parse::<Punct>()?;
        let mut attr_type = input.parse::<Ident>()?;
        if attr_name.to_string() == "p1" {
            ret.p1 = attr_type;
        } else {
            ret.p2 = attr_type;
        }

        input.parse::<Punct>()?;
        attr_name = input.parse::<Ident>()?;
        input.parse::<Punct>()?;
        attr_type = input.parse::<Ident>()?;
        if attr_name.to_string() == "p1" {
            ret.p1 = attr_type;
        } else {
            ret.p2 = attr_type;
        }

        Ok(ret)
    }
}

#[proc_macro_attribute]
pub fn return_type(attr: TokenStream, input: TokenStream) -> TokenStream {
    let ReturnTypeAttributes { p1, p2 } = parse_macro_input!(attr as ReturnTypeAttributes);
    let input = parse_macro_input!(input as ItemStruct);
    let dp1_ident = format_ident!("{}P1", input.ident);
    let dp2_ident = format_ident!("{}P2", input.ident);
    quote! {
        #input

        pub type #dp1_ident = #p1;
        pub type #dp2_ident = #p2;

    }
    .into()
}
