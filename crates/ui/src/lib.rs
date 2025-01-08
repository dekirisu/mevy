use deki::*;
use mevy_ui_syntax::*;
use proc_macro::TokenStream as CompilerTokens;

// Macro \\

    /// Provides CSS-like syntax to either edit or create bevy_ui components.
    ///
    /// ## Available fields
    /// To see a full list of built-in fields, see readme of mevy_ui, here are some examples:
    /// ```rust 
    /// cmd.spawn(ui!((
    ///     size: 50px 50px;
    ///     background: #ff0000;
    ///     box_shadow: 0px 0px 5px 5px #ff0000;
    ///     border: 5px #00ff00;
    /// )));
    /// ```
    ///
    /// ## Possible Modes
    /// Depending on the delimiters & if there is a name defined, the function of this macro differs:
    /// - Inline Tuple Mode | `ui!{( width: 1px; )}`:
    ///     - returns a tuple of mentioned UI components
    /// - Function Tuple Mode | `ui!{func_name( width: 1px; )}`: 
    ///     - defines a function that returns a tuple of mentioned UI components
    ///     - these can then be used as fields like this: `ui!{( func_name; )}`
    /// - Function Edit Mode | `ui!{func_name{ width: 2px; }}`
    ///     - defines a function, that edits mentioned UI components
    ///     - the parameters of this function = the needed mutable components
    ///     - using `_` will keep the original values: e.g. `border:  _ #ff0000;`
    /// 
    /// ## Custom Fields
    /// Every function that returns any `Bundle` (even if it is just -> `impl Bundle`) can be used
    /// as custom field.
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
                    TokenTree::Group(g) if g.delimiter().is_brace() => {
                        let g = iter.next().unwrap().risk_group();
                        let prep = UiPrep::from_stream(g.stream());
                        let (expected,edits) = prep.get_edits();
                        let attr = expected.into_iter()
                            .map(|(v,t)|qt!(#v: &mut #t))
                            .collect::<Vec<_>>();
                        qt!{pub fn #ident (#(#attr),*) {#edits}}
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
