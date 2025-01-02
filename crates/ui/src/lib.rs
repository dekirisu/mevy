use deki::*;
use mevy_ui_syntax::*;
use proc_macro::TokenStream as CompilerTokens;

// Macro \\

    /// One macro to rule them all:
    /// - `ui!{( width: 1px; )}`: css syntax => tuple of mentioned UI components
    /// - `ui!{{ let v = 1px; }}`: (soon) edit by css syntax
    /// - ..more to come
    #[proc_macro]
    pub fn ui (tok:CompilerTokens) -> CompilerTokens {
        let tok: TokenStream = tok.into();
        let mut iter = tok.peek_iter();
    
        match iter.peek().unwrap() {
            TokenTree::Group(_) => {
                kill!{*Some(TokenTree::Group(group)) = iter.next()}
                match group.delimiter() {
                    Delimiter::Parenthesis => bundle(group.stream().peek_iter(),iter.next()),
                    _ => todo!{}
                }
            }
            _ => todo!{}
        }.into()
    }


// EOF \\
