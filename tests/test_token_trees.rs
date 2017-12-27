#![cfg(feature = "extra-traits")]

#[macro_use]
extern crate quote;
extern crate syn;
extern crate proc_macro2;

use syn::{Lit, Macro};
use proc_macro2::{TokenNode, TokenTree, Spacing, Delimiter, TokenStream, Term};
use proc_macro2::Delimiter::*;

fn alone(c: char) -> TokenTree {
    proc_macro2::TokenTree {
        span: Default::default(),
        kind: TokenNode::Op(c, Spacing::Alone),
    }
}

fn joint(c: char) -> TokenTree {
    proc_macro2::TokenTree {
        span: Default::default(),
        kind: TokenNode::Op(c, Spacing::Joint),
    }
}

fn delimited(delim: Delimiter, tokens: Vec<TokenTree>) -> TokenTree {
    proc_macro2::TokenTree {
        span: Default::default(),
        kind: TokenNode::Group(delim, tokens.into_iter().collect()),
    }
}

fn word(sym: &str) -> TokenTree {
    proc_macro2::TokenTree {
        span: Default::default(),
        kind: TokenNode::Term(Term::intern(sym)),
    }
}

#[test]
fn test_struct() {
    let raw = "
        #[derive(Debug, Clone)]
        pub struct Item {
            pub ident: Ident,
            pub attrs: Vec<Attribute>,
        }
    ";

    let expected = vec![
        alone('#'),
        delimited(Bracket, vec![
           word("derive"),
           delimited(Parenthesis, vec![
               word("Debug"),
               alone(','),
               word("Clone"),
           ]),
        ]),
        word("pub"),
        word("struct"),
        word("Item"),
        delimited(Brace, vec![
           word("pub"),
           word("ident"),
           alone(':'),
           word("Ident"),
           alone(','),

           word("pub"),
           word("attrs"),
           alone(':'),
           word("Vec"),
           alone('<'),
           word("Attribute"),
           joint('>'),
           alone(','),
        ],
    )];

    fn wrap(tts: Vec<TokenTree>) -> Macro {
        Macro {
            path: "tts".into(),
            bang_token: Default::default(),
            tokens: tts,
        }
    }

    let result = raw.parse::<TokenStream>().unwrap()
                    .into_iter()
                    .collect();
    let result = wrap(result);
    let expected = wrap(expected);
    if result != expected {
        panic!("{:#?}\n!=\n{:#?}", result.tokens, expected.tokens);
    }
}

#[test]
fn test_literal_mangling() {
    let raw = "0_4";
    let parsed: Lit = syn::parse_str(raw).unwrap();
    assert_eq!(raw, quote!(#parsed).to_string());
}
