use deki::*;
use mevy_ui_syntax::*;
use proc_macro::TokenStream as CompilerTokens;

// Macro \\

    /// One macro to rule them all:
    /// - `ui!{( width: 1px; )}`: css syntax => tuple of mentioned UI components
    /// - `ui!{func_name()}`: (quick prefab) css syntax => fn func_name() -> impl Bundle {..} 
    /// - ..more to come
    #[proc_macro]
    pub fn ui (tok:CompilerTokens) -> CompilerTokens {
        let tok: TokenStream = tok.into();
        let mut iter = tok.peek_iter();
    
        match iter.peek().unwrap() {
            TokenTree::Ident(ident) if ident.to_string().chars().next().unwrap().is_lowercase() => {
                let ident = iter.next().unwrap().risk_ident();
                match iter.peek().unwrap() {
                    TokenTree::Group(g) if g.delimiter().is_parenthesis() => {
                        let g = iter.next().unwrap().risk_group();
                        let bundle = bundle(g.stream().peek_iter(),iter.next());
                        qt!{pub fn #ident () -> impl Bundle {#bundle}}
                    }
                    _ => todo!{} 
                }
            }

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
